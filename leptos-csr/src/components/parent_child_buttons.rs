use leptos::prelude::*;
use leptos::web_sys::MouseEvent;

#[component]
pub fn ParentChildButtons() -> impl IntoView {
    let (toggled, set_toggled) = signal(false);

    provide_context(set_toggled);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
        <ButtonB on_click=move |_| set_toggled.update(|cur| *cur = !*cur) />
        <ButtonC on:click=move |_| set_toggled.update(|cur| *cur = !*cur) />
        <ButtonD />
    }
}

#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                setter.update(|cur| *cur = !*cur);
            }
        >
            "Toggle WriteSignal"
        </button>
    }
}

#[component]
fn ButtonB(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button
            on:click=on_click
        >
            "Toggle Callback"
        </button>
    }
}

#[component]
fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle Event Listener"
        </button>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter");
    view! {
        <button
            on:click=move |_| {
                setter.update(|cur| *cur = !*cur);
            }
        >
            "Toggle Context"
        </button>
    }
}
