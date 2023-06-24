use leptos::*;
use leptos_animation::*;

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
            <div><Simple /></div>
        }
    })
}
