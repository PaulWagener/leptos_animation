use std::ops::{Sub};
use leptos::*;
use leptos_animation::*;
use levenshtein_diff::{generate_edits, distance, Edit, apply_edits};

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

#[component]
pub fn Text(cx: Scope) -> impl IntoView {
    #[derive(Clone)]
    struct Text(String);

    impl Sub for Text {
        type Output = Text;
        fn sub(self, rhs: Self) -> Self::Output {
            rhs
        }
    }

    let (text, set_text) = create_signal(cx, "");


    let animated_text =
        create_animated_signal(cx, move || text.get().into(), |from, to, progress| {
            let mut from = from.chars().collect::<Vec<char>>();
            let to = to.chars().collect::<Vec<char>>();

            let (_, matrix) = distance(&from, &to);
            let edits = generate_edits(&from, &to, &matrix).unwrap();

            for edit in &edits[0..((progress * edits.len() as f64) as usize)] {
                match edit {
                    Edit::Delete(i) => { from.remove(i - 1); }
                    Edit::Insert(i, c) => { from.insert(*i, *c); }
                    Edit::Substitute(i, c) => { from[i - 1] = *c; }
                };
            }

            Text(String::from_iter(from))
        });


    // create user interfaces with the declarative `view!` macro
    view! { cx,
        <div>
            <button on:click=move |_| { set_text.set("") }>"Empty"</button>
            <button on:click=move |_| { set_text.set(AUSTIN) }>"Austin"</button>
            <button on:click=move |_| { set_text.set(LOREM_IPSUM) }>"Lorem Ipsum"</button>
            <textarea prop:value=move || animated_text.get().0></textarea>
        </div>
    }
}
