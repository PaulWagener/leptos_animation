use leptos::*;
use leptos_animation::*;

#[component]
pub fn Simple() -> impl IntoView {
    // Note that the necessary AnimatedContext is initialized in main.rs in this demo

    let (value, set_value) = create_signal(0.0);

    let animated_value = create_animated_signal(move || value.get().into(), tween_default);

    let clear = move |_| set_value.set(0.0);
    let decrement = move |_| set_value.update(|value| *value -= 1.0);
    let increment = move |_| set_value.update(|value| *value += 1.0);

    view! {
        <main class="simple">
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <button on:click=increment>"+1"</button>
            <div>"Value: " {value} <br/> "Animated value: " {animated_value}</div>
            <a
                class="source"
                href="https://github.com/PaulWagener/leptos_animation/blob/master/example/src/simple.rs"
            >
                "View Source"
            </a>
        </main>
    }
}
