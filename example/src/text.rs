use std::ops::{Sub};
use leptos::*;
use leptos_animation::*;
use levenshtein_diff::{generate_edits, distance, Edit, apply_edits};


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

    let (text, set_text) = create_signal(cx, "Hallo".to_string());


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
            <button on:click=move |_| { set_text.set("Hallo".to_string()) }>"Empty"</button>
            <button on:click=move |_| { set_text.set("Lorem Ipsum".to_string()) }>
                "Lorem Ipsum"
            </button>
            <textarea prop:value=move || animated_text.get().0></textarea>
        </div>
    }
}
