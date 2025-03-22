use leptos::prelude::*;

#[component]
pub fn List() -> impl IntoView {
    let values = vec![0, 1, 2];
    let counters = (1..=5).map(|idx| RwSignal::new(idx));

    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <p>{values.clone()}</p>

        <ul>
            // Normal pattern
            // {values.into_iter().map(|n| view! { <li>{n}</li>}).collect::<Vec<_>>()}
            // Using Leptos collect_view utility
            {values.into_iter().map(|n| view! { <li>{n}</li>}).collect_view()}
        </ul>
        <ul>
            {counter_buttons}
        </ul>
    }
}
