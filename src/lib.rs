// use std::time::{Duration, Instant};
// use leptos::{create_effect, create_memo, create_rw_signal, create_signal, Memo, provide_context, ReadSignal, request_animation_frame, RwSignal, Scope, SignalGet, SignalGetUntracked, SignalSet, store_value, StoredValue, use_context, WriteSignal};
//
// /// The AnimationTick is a placeholder value for use in a special signal that fires on each tick
// #[derive(Clone)]
// pub struct AnimationTick;
//
// #[derive(Clone)]
// enum AnimatedValue<T: Clone> {
//     Static(T),
//     Animated {
//         from: T,
//         to: T,
//         start: Instant,
//         duration: Duration,
//     },
// }
//
// #[derive(Clone)]
// pub struct AnimatedSignal<V: 'static + Clone, I: 'static> {
//     animation_status: RwSignal<AnimatedValue<V>>,
//     memo: Memo<I>,
// }
//
// /// Value between 0.0 (start of animation) and 1.0 (end of animation) inclusive
// /// Also provides easing methods. TODO: Easing methods
// pub struct AnimationProgress(f64);
//
//
// pub fn create_animated_signal<V, I>(
//     cx: Scope,
//     initial: &V,
//     tween: impl Fn(&V, &V, AnimationProgress) -> I + 'static,
// ) -> AnimatedSignal<V, I>
//     where V: Clone, I: PartialEq {
//     let context: AnimationContext = use_context(cx).expect("No AnimationContext present, call AnimationContext::new() in a parent scope");
//
//     let animation_status = create_rw_signal(cx, AnimatedValue::Static(initial.clone()));
//     let memo = create_memo::<I>(cx, move |_| {
//         match animation_status.get() {
//             AnimatedValue::Static(v) => tween(&v, &v, AnimationProgress(0.0)),
//             AnimatedValue::Animated { from, to, duration, start } => {
//                 let now = Instant::now();
//                 if now > start + duration {
//                     // Animation finished
//                     tween(&from, &to, AnimationProgress(1.0))
//                 } else {
//                     // Animation still running
//                     context.request_next_tick();
//
//                     tween(&from, &to, AnimationProgress((now - start).as_secs_f64() / duration.as_secs_f64()))
//                 }
//             }
//         }
//     });
//     AnimatedSignal {
//         animation_status,
//         memo,
//     }
// }
//
// impl<V, U> AnimatedSignal<V, U> where V: Clone, U: Clone {
//     /// Snaps directly to the new value, cancelling any previous animation
//     fn snap_to(&self, value: V) {
//         self.animation_status.set(AnimatedValue::Static(value));
//     }
//
//     /// Replace the target of a running animation (if any) with the new value
//     fn replace_to(&self, value: V) {
//         self.animation_status.set(match self.animation_status.get_untracked() {
//             AnimatedValue::Static(_) => AnimatedValue::Static(value),
//             AnimatedValue::Animated { from, start, duration, .. } =>
//                 AnimatedValue::Animated { from, to: value, start, duration }
//         });
//     }
//
//     /// Start a new animation, cancelling any previous one
//     fn animate_to(&self, value: V, duration: Duration) {
//         self.animation_status.set(AnimatedValue::Animated {
//             from: match self.animation_status.get_untracked() {
//                 AnimatedValue::Static(value) => value,
//
//                 // Use the `to` from the previous animation, effectively fast-fowarding it if it is still running
//                 AnimatedValue::Animated { to, .. } => to
//             },
//             to: value,
//             start: Instant::now(),
//             duration,
//         })
//     }
// }
//
// impl<V, I> SignalGet<I> for AnimatedSignal<V, I> where V: Clone, I: Clone {
//     fn get(&self) -> I {
//         self.memo.get()
//     }
//
//     fn try_get(&self) -> Option<I> {
//         self.memo.try_get()
//     }
// }
//
// #[derive(Clone)]
// pub struct AnimationContext {
//     ticks: ReadSignal<AnimationTick>,
//     set_ticks: WriteSignal<AnimationTick>,
//     next_tick_requested: StoredValue<NextTickRequested>,
// }
//
// #[derive(Clone)]
// struct NextTickRequested(bool);
//
// impl AnimationContext {
//     /// Set up an AnimationContext for this scope
//     /// This context has to be driven manually by calling tick() each frame when is_animation_running() is true
//     #[must_use]
//     pub fn new_manual(cx: Scope) -> AnimationContext {
//         let (ticks, set_ticks) = create_signal(cx, AnimationTick);
//         let next_tick_requested = store_value(cx, NextTickRequested(false));
//
//         let animation_context = AnimationContext { ticks, set_ticks, next_tick_requested };
//         provide_context(cx, animation_context.clone());
//         animation_context
//     }
//
//     pub fn new(cx: Scope) -> AnimationContext {
//         let animation_context = AnimationContext::new_manual(cx);
//         create_effect(cx, move |_| {
//             let animation_context = animation_context.clone();
//             // Subscribe to ticks
//             animation_context.ticks.get();
//
//             // Call tick() in next frame if there are still animations running
//             if animation_context.is_animation_running() {
//                 request_animation_frame(move || {
//                     animation_context.tick()
//                 });
//             }
//         });
//         animation_context
//     }
//
//     /// Update all animated signals in this AnimationContext
//     pub fn tick(&self) {
//         self.next_tick_requested.set_value(NextTickRequested(false));
//         self.set_ticks.set(AnimationTick);
//     }
//
//     pub fn is_animation_running(&self) -> bool {
//         self.next_tick_requested.get_value().0
//     }
//
//     /// Call this from a signal to automatically update it at the next animation tick
//     fn request_next_tick(&self) {
//         // Subscribe to the animation ticks
//         self.ticks.get();
//
//         self.next_tick_requested.set_value(NextTickRequested(true))
//     }
// }
//
