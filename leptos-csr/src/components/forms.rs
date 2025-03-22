use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;

#[component]
pub fn Forms() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);

    let (uncntl, set_uncntl) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // prevent page reload
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        set_uncntl.set(value);
    };

    view! {
        <input type="text"
            on:input:target=move |ev| {
                set_name.set(ev.target().value())
            }
            prop:value=name
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send spam"
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You'll receive cool shizzle"</p>
        </Show>
        <form on:submit=on_submit>
            <input type="text"
                value=uncntl
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Unconrolled is: " {uncntl}</p>
    }
}
