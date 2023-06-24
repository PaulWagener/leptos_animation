use leptos::*;
use leptos_animation::*;

#[component]
pub fn Simple(cx: Scope) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, 0);

    let animated_value =
        create_animated_signal(cx, move || (value.get() as f64).into(), tween::default());

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! { cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <button on:click=increment>"+1"</button>
            <div>"Value: " {value} "!"</div>
            <div>"Animated value: " {animated_value} "!"</div>
        </div>
    }
}
