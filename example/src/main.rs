use leptos::*;
use leptos::html::Canvas;
use wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

enum Color {
    Red,
    Green,
    Blue,
    Custom { red: u8, green: u8, blue: u8 },
}

#[derive(Clone, PartialEq)]
enum Size {
    Small,
    Big,
}

enum Duration {
    Short,
    Normal,
    Long,
}

enum Easing {
    Linear,
    Smooth,
    Overshoot,
    Elastic,
}

fn main() {
    mount_to_body(|cx| {
        let (target_color, set_target_color) = create_signal(cx, Color::Red);
        let (target_position, set_target_position) = create_signal(cx, (200, 200));
        let (target_size, set_target_size) = create_signal(cx, Size::Small);
        let (target_rotation, set_target_rotation) = create_signal(cx, 0);

        let (duration, set_duration) = create_signal(cx, Duration::Normal);

        let canvas_ref = create_node_ref::<Canvas>(cx);


        create_effect(cx, move |_| {
            if let Some(canvas) = canvas_ref.get() {
                let ctx = canvas.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.reset_transform().unwrap();
                ctx.scale(2.0, 2.0).unwrap();

                ctx.clear_rect(0.0, 0.0, 800.0, 800.0);

                let size = match target_size.get() {
                    Size::Small => 50.0,
                    Size::Big => 100.0,
                };

                ctx.set_fill_style(&JsValue::from_str("red"));
                ctx.fill_rect(50.0, 50.0, size, size);
                ctx.stroke_rect(50.0, 50.0, size, size);
            }
        });

        view! { cx,
            <div>
                <h1>"Animation Demo"</h1>
                <main>
                    <div class="options">
                        <fieldset>
                            <legend>"Color"</legend>
                            <input type="radio" id="color_red"/>
                            <label for="color_red">"Red"</label>
                            <input type="radio" id="color_green"/>
                            <label for="color_green">"Green"</label>
                            <input type="radio" id="color_blue"/>
                            <label for="color_blue">"Blue"</label>
                            <input type="color" id="color_custom"/>
                            <label for="color_custom">"Custom"</label>
                        </fieldset>
                        <fieldset>
                            <legend>"Rotation"</legend>
                            <input type="radio" id="rotation_0"/>
                            <label for="rotation_0">"0°"</label>
                            <input type="radio" id="rotation_180"/>
                            <label for="rotation_180">"180°"</label>
                            <input type="radio" id="rotation_360"/>
                            <label for="rotation_360">"360°"</label>
                            <input type="range" min="0" max="360"/>
                            {target_rotation.get()}
                            "°"
                        </fieldset>
                        <fieldset>
                            <legend>"Size"</legend>
                            <input
                                type="radio"
                                id="size_small"
                                on:input=move |_| { set_target_size.set(Size::Small) }
                                prop:checked=move || { target_size.get() == Size::Small }
                            />
                            <label for="size_small">"Small"</label>
                            <input
                                type="radio"
                                id="size_big"
                                on:input=move |_| { set_target_size.set(Size::Big) }
                                prop:checked=move || { target_size.get() == Size::Big }
                            />
                            <label for="size_big">"Big"</label>
                        </fieldset>
                        <fieldset>
                            <legend>"Duration"</legend>
                            <input type="radio" id="duration_short"/>
                            <label for="duration_short">"Short"</label>
                            <input type="radio" id="duration_normal"/>
                            <label for="duration_normal">"Normal"</label>
                            <input type="radio" id="duration_long"/>
                            <label for="duration_long">"Long"</label>
                        </fieldset>
                        <fieldset>
                            <legend>"Easing"</legend>
                            <input type="radio" id="easing_linear"/>
                            <label for="easing_linear">"Linear"</label>
                            <input type="radio" id="easing_smooth"/>
                            <label for="easing_smooth">"Cubic"</label>
                            <input type="radio" id="easing_overshoot"/>
                            <label for="easing_overshoot">"Overshoot"</label>
                            <input type="radio" id="easing_elastic"/>
                            <label for="easing_elastic">"Elastic"</label>
                        </fieldset>
                    </div>
                    <div class="canvas">
                        <div class="container">
                            <canvas width="800" height="800" _ref=canvas_ref></canvas>
                            <button class="top_left">"Move here"</button>
                            <button class="top_right">"Move here"</button>
                            <button class="bottom_left">"Move here"</button>
                            <button class="bottom_right">"Move here"</button>
                        </div>
                        <div>
                            "Left drag: StartOrReplace" <br/> "Middle drag: SnapOrReplace" <br/>
                            "Right click: Snap"
                        </div>
                    </div>
                </main>
            </div>
        }
    })
}