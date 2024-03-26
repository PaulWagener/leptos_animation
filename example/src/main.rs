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

    mount_to_body(|| {
        // Initialize a single AnimationContext for all three demo's
        AnimationContext::provide();

        view! {
            <Router trailing_slash=TrailingSlash::Exact>
                <h1>"Animation Demo"</h1>
                <nav>
                    <a href="/leptos_animation/">"Full"</a>
                    <a href="/leptos_animation/simple">"Simple"</a>
                    <a href="/leptos_animation/text">"Text"</a>
                </nav>
                <Routes>
                    <Route
                        path="/leptos_animation/"
                        view=|| {
                            view! { <Full/> }
                        }
                    />

                    <Route
                        path="/leptos_animation/simple"
                        view=|| {
                            view! { <Simple/> }
                        }
                    />

                    <Route
                        path="/leptos_animation/text"
                        view=|| {
                            view! { <Text/> }
                        }
                    />

                </Routes>
            </Router>
        }
    })
}
