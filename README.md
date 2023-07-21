[Docs](https://docs.rs/leptos_animation/latest/leptos_animation/)
| [Demo](https://paulwagener.github.io/leptos_animation/)

Create derived signals that are animated versions of the original signal

```rust
use leptos::*;
use leptos_animation::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    AnimationContext::provide(cx);

    let (value, set_value) = create_signal(cx, 0.0);

    let animated_value = create_animated_signal(cx, move || value.get().into(), tween_default);

    let clear = move |_| set_value.set(0.0);
    let decrement = move |_| set_value.update(|value| *value -= 1.0);
    let increment = move |_| set_value.update(|value| *value += 1.0);

    view! { cx,
        <main class="simple">
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <button on:click=increment>"+1"</button>
            <div>"Value: " {value} <br/> "Animated value: " {animated_value}</div>
        </main>
    }
}
```

# Features

* Allows for multiple animations playing simultaneously
* Efficiently calls `window.request_animation_frame()`: only when there are animations playing and only once per frame
  even if there are multiple animated signals running.
* Allows for custom durations, easing functions, target updates and tween methods. Can be made to work for any type.
* Animated signals are all updates simultaneously per frame. Effects that use multiple animated signals are called only
  once per frame.
