use derive_more::{Add, Mul, Sub};
use leptos::html::Canvas;
use leptos::*;
use leptos_animation::*;
use palette::{self, convert::FromColorUnclamped, rgb::Rgb, FromColor, Hsv, Mix};
use std::f64::consts::PI;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn Full(cx: Scope) -> impl IntoView {
    // These are the target values that the animation is trying to reach
    let (target_color, set_target_color) = create_signal(
        cx,
        Color {
            red: 255,
            green: 0,
            blue: 0,
        },
    );
    let (target_size, set_target_size) = create_signal(cx, Size::Small);
    let (target_rotation, set_target_rotation) = create_signal(cx, 0.0);
    let (target_position, set_target_position) =
        create_signal(cx, (Position { x: 200.0, y: 200.0 }, AnimationMode::Snap));

    // Animation mode, easings & durations are normally hardcoded, they are only signals here for demo purposes
    let (duration, set_duration) = create_signal(cx, Duration::Normal);
    let (easing, set_easing) = create_signal(cx, Easing::Smooth);
    let (mode, set_mode) = create_signal(cx, MouseMoveAnimationMode::None);

    // Animated derived signals
    let size = create_animated_signal(
        cx,
        move || AnimationTarget {
            target: target_size.get(),
            duration: duration.get_untracked().into(),
            easing: easing.get_untracked().into(),
            mode: AnimationMode::Start,
        },
        |from, to, progress| tween_default(&from.to_pixels(), &to.to_pixels(), progress),
    );

    let rotation = create_animated_signal(
        cx,
        move || AnimationTarget {
            target: target_rotation.get(),
            duration: duration.get_untracked().into(),
            easing: easing.get_untracked().into(),
            mode: AnimationMode::Start,
        },
        tween_default,
    );

    let position = create_animated_signal(
        cx,
        move || {
            let (target, mode) = target_position.get();
            AnimationTarget {
                target,
                duration: duration.get_untracked().into(),
                easing: match easing.get_untracked() {
                    Easing::Linear => easing::LINEAR,
                    Easing::Smooth => easing::CUBIC_IN_OUT,
                    Easing::Overshoot => easing::BACK_IN_OUT,
                    Easing::Elastic => easing::ELASTIC_IN_OUT,
                },
                mode,
            }
        },
        tween_default,
    );

    let color = create_animated_signal(
        cx,
        move || AnimationTarget {
            target: target_color.get(),
            duration: duration.get_untracked().into(),
            easing: easing.get_untracked().into(),
            mode: AnimationMode::Start,
        },
        |from, to, progress| -> Color {
            // Convert to HSV to do the tweening
            let from: Hsv<_, _> = from.into();
            let to = to.into();

            let mix = Rgb::from_color(from.mix(to, progress));

            Color {
                red: (mix.red * 255.0) as i16,
                green: (mix.green * 255.0) as i16,
                blue: ((mix.blue * 255.0) as i16),
            }
        },
    );

    // Draw a square with the animated signals
    let canvas_ref = create_node_ref::<Canvas>(cx);
    create_effect(cx, move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.reset_transform().unwrap();
            ctx.clear_rect(0.0, 0.0, 800.0, 800.0);
            ctx.scale(2.0, 2.0).unwrap();

            let Position { x, y } = position.get();
            ctx.translate(x, y).unwrap();
            ctx.rotate(rotation.get() / 180.0 * PI).unwrap();

            let Color { red, green, blue } = color.get();
            ctx.set_fill_style(&JsValue::from_str(&format!("rgb({red}, {green}, {blue})")));

            let size = size.get();
            ctx.fill_rect(-size / 2.0, -size / 2.0, size, size);
            ctx.stroke_rect(-size / 2.0, -size / 2.0, size, size);
        }
    });

    // Everything below this line is UI boilerplate
    view! { cx,
        <div>
            <main>
                <div class="options">
                    <fieldset>
                        <legend>"Color"</legend>
                        <input
                            type="radio"
                            id="color_red"
                            on:input=move |_| {
                                set_target_color
                                    .set(Color {
                                        red: 255,
                                        green: 0,
                                        blue: 0,
                                    })
                            }
                            prop:checked=move || { matches!(target_color.get(), Color { red : 255, green : 0, blue : 0 }) }
                        />
                        <label for="color_red">"Red"</label>
                        <input
                            type="radio"
                            id="color_green"
                            on:input=move |_| {
                                set_target_color
                                    .set(Color {
                                        red: 0,
                                        green: 255,
                                        blue: 0,
                                    })
                            }
                            prop:checked=move || { matches!(target_color.get(), Color { red : 0, green : 255, blue : 0 }) }
                        />
                        <label for="color_green">"Green"</label>
                        <input
                            type="radio"
                            id="color_blue"
                            on:input=move |_| {
                                set_target_color
                                    .set(Color {
                                        red: 0,
                                        green: 0,
                                        blue: 255,
                                    })
                            }
                            prop:checked=move || { matches!(target_color.get(), Color { red : 0, green : 0, blue : 255 }) }
                        />
                        <label for="color_blue">"Blue"</label>
                        <input
                            type="color"
                            on:input=move |e| {
                                let color = event_target_value(&e);
                                set_target_color
                                    .set(Color {
                                        red: i16::from_str_radix(&color[1..3], 16).unwrap(),
                                        green: i16::from_str_radix(&color[3..5], 16).unwrap(),
                                        blue: i16::from_str_radix(&color[5..7], 16).unwrap(),
                                    });
                            }
                            prop:value=move || {
                                let color = target_color.get();
                                format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
                            }
                        />
                    </fieldset>
                    <fieldset>
                        <legend>"Rotation"</legend>
                        <input
                            type="radio"
                            id="rotation_0"
                            on:input=move |_| { set_target_rotation.set(0.0) }
                            prop:checked=move || { target_rotation.get() == 0.0 }
                        />
                        <label for="rotation_0">"0°"</label>
                        <input
                            type="radio"
                            id="rotation_180"
                            on:input=move |_| { set_target_rotation.set(180.0) }
                            prop:checked=move || { target_rotation.get() == 180.0 }
                        />
                        <label for="rotation_180">"180°"</label>
                        <input
                            type="radio"
                            id="rotation_360"
                            on:input=move |_| { set_target_rotation.set(360.0) }
                            prop:checked=move || { target_rotation.get() == 360.0 }
                        />
                        <label for="rotation_360">"360°"</label>
                        <br/>
                        <input
                            type="range"
                            min="0"
                            max="360"
                            prop:value=move || { target_rotation.get() }
                            on:input=move |e| { set_target_rotation.set(event_target_value(&e).parse().unwrap()) }
                        />
                        {move || { target_rotation.get() }}
                        "°"
                    </fieldset>
                    <fieldset>
                        <legend>"Size"</legend>
                        <input
                            type="radio"
                            id="size_small"
                            on:input=move |_| { set_target_size.set(Size::Small) }
                            prop:checked=move || { matches!(target_size.get(), Size::Small) }
                        />
                        <label for="size_small">"Small"</label>
                        <input
                            type="radio"
                            id="size_big"
                            on:input=move |_| { set_target_size.set(Size::Big) }
                            prop:checked=move || { matches!(target_size.get(), Size::Big) }
                        />
                        <label for="size_big">"Big"</label>
                    </fieldset>
                    <fieldset>
                        <legend>"Duration"</legend>
                        <input
                            type="radio"
                            id="duration_short"
                            on:input=move |_| { set_duration.set(Duration::Short) }
                            prop:checked=move || { matches!(duration.get(), Duration::Short) }
                        />
                        <label for="duration_short">"Short"</label>
                        <input
                            type="radio"
                            id="duration_normal"
                            on:input=move |_| { set_duration.set(Duration::Normal) }
                            prop:checked=move || { matches!(duration.get(), Duration::Normal) }
                        />
                        <label for="duration_normal">"Normal"</label>
                        <input
                            type="radio"
                            id="duration_long"
                            on:input=move |_| { set_duration.set(Duration::Long) }
                            prop:checked=move || { matches!(duration.get(), Duration::Long) }
                        />
                        <label for="duration_long">"Long"</label>
                    </fieldset>
                    <fieldset>
                        <legend>"Easing"</legend>
                        <input
                            type="radio"
                            id="easing_linear"
                            on:input=move |_| { set_easing.set(Easing::Linear) }
                            prop:checked=move || { matches!(easing.get(), Easing::Linear) }
                        />
                        <label for="easing_linear">"Linear"</label>
                        <input
                            type="radio"
                            id="easing_smooth"
                            on:input=move |_| { set_easing.set(Easing::Smooth) }
                            prop:checked=move || { matches!(easing.get(), Easing::Smooth) }
                        />
                        <label for="easing_smooth">"Smooth"</label>
                        <input
                            type="radio"
                            id="easing_overshoot"
                            on:input=move |_| { set_easing.set(Easing::Overshoot) }
                            prop:checked=move || { matches!(easing.get(), Easing::Overshoot) }
                        />
                        <label for="easing_overshoot">"Overshoot"</label>
                        <input
                            type="radio"
                            id="easing_elastic"
                            on:input=move |_| { set_easing.set(Easing::Elastic) }
                            prop:checked=move || { matches!(easing.get(), Easing::Elastic) }
                        />
                        <label for="easing_elastic">"Elastic"</label>
                    </fieldset>
                    <fieldset>
                        <legend>"Animation mode (mouse move)"</legend>
                        <input
                            type="radio"
                            id="mode_none"
                            on:input=move |_| { set_mode.set(MouseMoveAnimationMode::None) }
                            prop:checked=move || { matches!(mode.get(), MouseMoveAnimationMode::None) }
                        />
                        <label for="mode_none">"None"</label>
                        <input
                            type="radio"
                            id="mode_replace_or_start"
                            on:input=move |_| { set_mode.set(MouseMoveAnimationMode::ReplaceOrStart) }
                            prop:checked=move || { matches!(mode.get(), MouseMoveAnimationMode::ReplaceOrStart) }
                        />
                        <label for="mode_replace_or_start">"ReplaceOrStart"</label>
                        <input
                            type="radio"
                            id="mode_replace_or_snap"
                            on:input=move |_| { set_mode.set(MouseMoveAnimationMode::ReplaceOrSnap) }
                            prop:checked=move || { matches!(mode.get(), MouseMoveAnimationMode::ReplaceOrSnap) }
                        />
                        <label for="mode_replace_or_snap">"ReplaceOrSnap"</label>
                        <input
                            type="radio"
                            id="mode_snap"
                            on:input=move |_| { set_mode.set(MouseMoveAnimationMode::Snap) }
                            prop:checked=move || { matches!(mode.get(), MouseMoveAnimationMode::Snap) }
                        />
                        <label for="mode_snap">"Snap"</label>
                    </fieldset>
                </div>
                <div class="canvas">
                    <canvas
                        width="800"
                        height="800"
                        _ref=canvas_ref
                        on:mousedown=move |e| {
                            set_target_position
                                .set((
                                    Position {
                                        x: e.offset_x() as f64,
                                        y: e.offset_y() as f64,
                                    },
                                    AnimationMode::Start,
                                ));
                        }
                        on:mousemove=move |e| {
                            let position = Position {
                                x: e.offset_x() as f64,
                                y: e.offset_y() as f64,
                            };
                            match mode.get_untracked() {
                                MouseMoveAnimationMode::None => {}
                                MouseMoveAnimationMode::ReplaceOrStart => {
                                    set_target_position.set((position, AnimationMode::ReplaceOrStart))
                                }
                                MouseMoveAnimationMode::ReplaceOrSnap => {
                                    set_target_position.set((position, AnimationMode::ReplaceOrStart))
                                }
                                MouseMoveAnimationMode::Snap => {
                                    set_target_position.set((position, AnimationMode::Snap))
                                }
                            }
                        }
                    ></canvas>
                </div>
            </main>
        </div>
    }
}

#[derive(Clone, Sub)]
struct Color {
    red: i16,
    green: i16,
    blue: i16,
}

impl From<&Color> for Hsv<Rgb, f64> {
    fn from(Color { red, green, blue }: &Color) -> Self {
        Hsv::from_color_unclamped(Rgb::new(
            *red as f64 / 255.0,
            *green as f64 / 255.0,
            *blue as f64 / 255.0,
        ))
    }
}

#[derive(Clone)]
enum Size {
    Small,
    Big,
}

impl Size {
    fn to_pixels(&self) -> f64 {
        match self {
            Size::Small => 50.0,
            Size::Big => 100.0,
        }
    }
}

#[derive(Clone)]
enum Duration {
    Short,
    Normal,
    Long,
}

#[derive(Add, Sub, Mul, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        match value {
            Duration::Short => std::time::Duration::from_secs_f64(0.5),
            Duration::Normal => std::time::Duration::from_secs_f64(1.0),
            Duration::Long => std::time::Duration::from_secs_f64(2.0),
        }
    }
}

#[derive(Clone)]
enum Easing {
    Linear,
    Smooth,
    Overshoot,
    Elastic,
}

#[derive(Clone)]
enum MouseMoveAnimationMode {
    None,
    ReplaceOrStart,
    ReplaceOrSnap,
    Snap,
}

impl From<Easing> for leptos_animation::Easing {
    fn from(value: Easing) -> Self {
        match value {
            Easing::Linear => easing::LINEAR,
            Easing::Smooth => easing::CUBIC_OUT,
            Easing::Overshoot => easing::BACK_OUT,
            Easing::Elastic => easing::ELASTIC_OUT,
        }
    }
}
