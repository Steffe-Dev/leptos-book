use leptos::prelude::*;
use reactive_stores::*;

#[derive(Store, Clone, Debug)]
struct Data {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<DatabaseEntry>,
}

#[derive(Clone, Debug)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn ComplexIter() -> impl IntoView {
    let data = Store::new(Data {
        rows: vec![
            DatabaseEntry {
                key: "foo".to_string(),
                value: 10,
            },
            DatabaseEntry {
                key: "bar".to_string(),
                value: 11,
            },
            DatabaseEntry {
                key: "baz".to_string(),
                value: 12,
            },
        ],
    });
    view! {
        <button
            on:click=move |_| {
                for row in data.rows().iter_unkeyed() {
                    row.update(|entry| entry.value *= 2);
                };

            leptos::logging::log!("{:?}", data.get());
        }>
            "Update values"
        </button>

        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=move |child| {
                view! {
                    <p>{move || child.get().value}</p>
                }
            }
        />
    }
}
