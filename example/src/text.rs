use leptos::*;
use leptos_animation::*;
use std::ops::Sub;

#[component]
pub fn Text(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, "");

    let animated_text = create_animated_signal(
        cx,
        move || text.get().into(),
        |from, to, progress| {
            // Animate between strings by taking the beginning of the to-string
            // followed by the end of the from-string
            TextString(String::from_iter(
                to.chars()
                    .take((progress * to.len() as f64) as usize)
                    .chain(from.chars().skip((progress * from.len() as f64) as usize)),
            ))
        },
    );

    view! { cx,
        <main class="text">
            <button on:click=move |_| { set_text.set("") }>"Empty"</button>
            <button on:click=move |_| { set_text.set("Hello World") }>"Hello"</button>
            <button on:click=move |_| { set_text.set(AUSTIN) }>"Austin"</button>
            <button on:click=move |_| { set_text.set(LOREM_IPSUM) }>"Lorem Ipsum"</button>
            <textarea prop:value=move || animated_text.get().0></textarea>
            <a
                class="source"
                href="https://github.com/PaulWagener/leptos_animation/blob/main/example/src/text.rs"
            >
                "View Source"
            </a>
        </main>
    }
}

#[derive(Clone)]
struct TextString(String);

/// create_animated_signal requires interpolated values to be subtractable to allow for overlapping
/// animations. This is a very ill-defined operations for strings, so we basically cheat by not providing
/// a real implementation. This will result in glitches when there are multiple overlapping animations.
impl Sub for TextString {
    type Output = TextString;
    fn sub(self, rhs: Self) -> Self::Output {
        rhs
    }
}

const AUSTIN: &str = "IT is a truth universally acknowledged, that a single man in possession of a good fortune must be in want of a wife.
However little known the feelings or views of such a man may be on his first entering a neighbourhood, this truth is so well fixed in the minds of the surrounding families, that he is considered as the rightful property of some one or other of their daughters.
``My dear Mr. Bennet,'' said his lady to him one day, ``have you heard that Netherfield Park is let at last?''
Mr. Bennet replied that he had not.
``But it is,'' returned she; ``for Mrs. Long has just been here, and she told me all about it.''
Mr. Bennet made no answer.
``Do not you want to know who has taken it?'' cried his wife impatiently.
``You want to tell me, and I have no objection to hearing it.''
This was invitation enough.
``Why, my dear, you must know, Mrs. Long says that Netherfield is taken by a young man of large fortune from the north of England; that he came down on Monday in a chaise and four to see the place, and was so much delighted with it that he agreed with Mr. Morris immediately; that he is to take possession before Michaelmas, and some of his servants are to be in the house by the end of next week.''";

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque non arcu at ligula condimentum varius sit amet vel erat. Vivamus aliquet ornare lobortis. Vivamus vestibulum nisi sed turpis rhoncus, id vestibulum turpis accumsan. Nulla facilisi. Suspendisse efficitur porta massa, non porta nisl suscipit vitae. Etiam varius eleifend urna id porta. Nulla facilisi. Morbi non quam egestas, elementum mauris eu, gravida eros. Maecenas blandit mi ac massa tincidunt scelerisque eget id ante. Fusce cursus in nulla ut fermentum. Proin dictum semper velit, sed bibendum velit rhoncus eget. Quisque ut sodales ipsum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris efficitur mi sit amet nulla sollicitudin sodales.
Ut blandit erat nec quam dictum, sit amet venenatis dolor efficitur. Sed arcu massa, rutrum vel massa eget, accumsan bibendum nisl. Maecenas auctor elementum semper. Donec quis turpis ex. In laoreet erat purus, sed rutrum turpis finibus vel. Pellentesque vulputate sodales velit, at dignissim velit tristique eu. Nulla scelerisque lobortis magna, a efficitur lacus placerat in. Proin pharetra tortor libero, id posuere nibh consectetur eu. Etiam fringilla finibus quam, ut placerat augue aliquam eget. Suspendisse potenti. Praesent eget tincidunt ante. Praesent imperdiet lectus maximus nibh fringilla, eu dictum nunc malesuada. Duis sodales mollis sem, ut accumsan lacus porttitor eget. Phasellus diam purus, condimentum sit amet mollis in, volutpat ut nisl.";
