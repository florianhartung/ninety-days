use leptos::{component, create_signal, For, IntoView, Scope, SignalUpdate, view};
use leptos::{ReadSignal, WriteSignal};

use crate::submit_field::SubmitField;
use crate::todo_entry::TodoEntry;

#[component]
/// A simple to-do list
pub fn TodoList(cx: Scope) -> impl IntoView {
    // The id used for the next to-do entry
    let mut next_entry_id: u32 = 0;

    let (entries, set_entries) = create_signal(
        cx,
        Vec::<(ReadSignal<TodoEntry>, WriteSignal<TodoEntry>)>::new(),
    );

    // Handler for when a new entry text is submitted
    let on_submit = move |text| {
        set_entries.update(|entries| {
            entries.push(create_signal(cx, TodoEntry::new(next_entry_id, text)));
            next_entry_id += 1;
        });
    };

    // How a to-do entry will be viewed
    let view_entry = move |cx,
                           (current_entry, set_current_entry): (
                               ReadSignal<TodoEntry>,
                               WriteSignal<TodoEntry>,
                           )| {
        let delete_current_entry = move |_| {
            set_entries.update(|entries| entries.retain(|e| e.0().id != current_entry().id))
        };

        let toggle_done = move |_| {
            set_current_entry.update(|e| e.is_done = !e.is_done);
        };

        let text_style = move || {
            current_entry()
                .is_done
                .then(|| "text-decoration: line-through;")
                .unwrap_or_default()
        };

        view! { cx,
            <li>
                <button on:click=delete_current_entry>"❌"</button>
                <button on:click=toggle_done>
                    { move || current_entry().is_done.then(|| "↩️").unwrap_or("✅") }
                </button>
                <span style=text_style>
                    {current_entry().name}
                </span>
            </li>
        }
    };

    view! { cx,
        <div>
            <h1> My Todo list </h1>
            <SubmitField on_submit=on_submit submit_label="Add todo".to_owned() />
            <ul>
                <For
                    each=entries
                    key=|(entry, _)| entry().id
                    view=view_entry
                />
            </ul>
        </div>
    }
}
