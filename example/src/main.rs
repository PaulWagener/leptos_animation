use full::Full;
use leptos::prelude::*;
use leptos_animation::*;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::components::A;

use leptos_router::path;
use simple::Simple;
use std::panic;
use text::Text;

extern crate console_error_panic_hook;

mod full;
mod simple;
mod text;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|| {
        // Initialize a single AnimationContext for all three demo's
        AnimationContext::provide();

        view! {
            <Router base="/leptos_animation">
                <h1>"Animation Demo"</h1>
                <nav>
                    <A href="/leptos_animation/">Full</A>
                    <A href="/leptos_animation/simple">Simple</A>
                    <A href="/leptos_animation/text">Text</A>
                </nav>
                <Routes fallback=|| "Not found">

                    <Route
                        path=path!("/")
                        view=|| {
                            view! { <Full/> }
                        }
                    />
                    <Route
                        path=path!("simple")
                        view=|| {
                            view! { <Simple/> }
                        }
                    />

                    <Route
                        path=path!("text")
                        view=|| {
                            view! { <Text/> }
                        }
                    />

                </Routes>
            </Router>
        }
    })
}
