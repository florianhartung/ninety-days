use leptos::{component, IntoView, mount_to_body, Scope, view};

use crate::todolist::TodoList;

pub(crate) mod submit_field;
pub(crate) mod todo_entry;
mod todolist;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <TodoList />
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
}
