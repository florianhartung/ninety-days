use leptos::{
    component, create_node_ref, create_signal, For, IntoView, mount_to_body, NodeRef, Scope, Signal,
    SignalUpdate, view,
};
use leptos::ev::SubmitEvent;
use leptos::html::Input;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <>
            <TodoList />
        </>
    }
}

#[component]
/// An awesome progress bar
fn ProgressBar(
    cx: Scope,
    /// The current amount of progress that is showed.
    #[prop(into)]
    progress: Signal<i32>,
    /// The maximum value of the progress bar. Default is 100.
    #[prop(optional)]
    max: Option<i32>,
) -> impl IntoView {
    view! { cx,
        <progress max=max.unwrap_or(100) value=progress />
    }
}

#[component]
/// A simple to-do list
fn TodoList(cx: Scope) -> impl IntoView {
    let mut next_entry_id: usize = 0;

    let (entries, set_entries) = create_signal(cx, Vec::<(usize, String)>::new());

    let label_input: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let label_element = label_input().expect("<input> to exist");
        let label = label_element.value();
        label_element.set_value("");

        if !label.is_empty() {
            set_entries.update(|entries| {
                entries.push((next_entry_id, label));
                next_entry_id += 1;
            });
        }
    };

    view! { cx,
        <div>
            <h1> My Todo list </h1>
            <form on:submit=on_submit>
                <input type="text" value="" node_ref=label_input />
                <input type="submit" value="New" />
            </form>
            <ul>
                <For
                    each=entries
                    key=|entry| entry.0
                    view=move |cx, (id, label)| {
                        let remove_current_entry = move |_| set_entries.update(|entries| entries.retain(|e| e.0 != id));

                        view! { cx,
                            <li>
                                <button on:click=remove_current_entry>"❌"</button>
                                {label}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}
