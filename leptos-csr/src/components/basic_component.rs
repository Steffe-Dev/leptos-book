use crate::components::list::List;
use crate::components::progress_bar::ProgressBar;
use leptos::html::{button, div, input, span};
use leptos::{ev, prelude::*};

#[component]
pub fn BasicComponent() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            // on:click=move |_| set_count.set(count.get() + 1)
            // on:click=move |_| set_count.update(|n| *n += 1)
            on:click=move |_| *set_count.write() += 1
            class=(["red", "italic"], move || count.get() % 2 == 0)
            // set the style attribute
            style="position: absolute"
            // toggle individual CSS properies with `style:`
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({},{},100)", count.get(), 100)
            style:max-width="400px"
            // set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {double_count}
        </p>
        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count)/>
        <List/>
        // <WrapsChildren>
        //     "A"
        //     "B"
        //     "C"
        // </WrapsChildren>
        <ViewBuilderCounter
            initial_value=0
            // step=2
        />
    }
}

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        // .collect::<Vec<_>>();
        .collect_view();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}

#[component]
fn ViewBuilderCounter(initial_value: i32, #[prop(default = 1)] step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);

    div().child((
        button()
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child(("-", step)),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child(("+", step)),
    ))
}
