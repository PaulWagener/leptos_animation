use leptos::*;
use leptos_animation::*;
use leptos_router::*;

extern crate console_error_panic_hook;

use std::panic;

mod full;
mod simple;
mod text;

use full::Full;
use simple::Simple;
use text::Text;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|cx| {
        // Initialize a single AnimationContext for all three demo's
        AnimationContext::provide(cx);

        view! { cx,
            <Router>
                <h1>"Animation Demo"</h1>
                <div>
                    <a href="/">"Full"</a>
                    <a href="/simple">"Simple"</a>
                    <a href="/text">"Text"</a>
                </div>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <Full/> }
                        }
                    />
                    <Route
                        path="/simple"
                        view=|cx| {
                            view! { cx, <Simple/> }
                        }
                    />
                    <Route
                        path="/text"
                        view=|cx| {
                            view! { cx, <Text/> }
                        }
                    />
                </Routes>
            </Router>
        }
    })
}
