use leptos::{component, create_node_ref, IntoView, NodeRef, Scope, view};
use leptos::ev::SubmitEvent;
use leptos::html::Input;

#[component]
/// A submittable text field. Automatically clears on submit.
pub fn SubmitField<F: FnMut(String) + 'static>(
    cx: Scope,
    /// A callback for when a text is submitted.
    mut on_submit: F,
    /// Label of the submit button. Default is "Submit".
    #[prop(optional)]
    submit_label: Option<String>,
) -> impl IntoView {
    // A reference to the uncontrolled text field
    let input_ref: NodeRef<Input> = create_node_ref(cx);

    // Handler for the submit button event
    let on_submit_event = move |ev: SubmitEvent| {
        ev.prevent_default();

        let input_element = input_ref.get().expect("<input> to exist");
        let input_value = input_element.value();

        if !input_value.is_empty() {
            input_element.set_value("");
            on_submit(input_value);
        }
    };

    view! { cx,
        <div>
            <form on:submit=on_submit_event>
                <input type="text" value="" node_ref=input_ref />
                <input type="submit" value=submit_label.unwrap_or("Submit".to_owned())/>
            </form>
        </div>
    }
}
