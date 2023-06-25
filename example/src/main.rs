use leptos::*;
use leptos_animation::*;
use leptos_router::*;
extern crate console_error_panic_hook;
use std::panic;
mod full;
mod simple;
use full::Full;
use simple::Simple;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|cx| {
        AnimationContext::provide(cx);

        view! {cx,
            <Router>
            <h1>"Animation Demo"</h1>
            <div><a href="/">"Full"</a><a href="/simple">"Simple"</a></div>
            <Routes>
                <Route path="/" view=|cx| view! { cx, <Full/> }/>
                <Route path="/simple" view=|cx| view! { cx, <Simple/> }/>
            </Routes>
            </Router>
        }
    })
}
