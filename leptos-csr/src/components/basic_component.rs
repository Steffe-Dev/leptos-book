use leptos::prelude::*;
use crate::components::progress_bar::ProgressBar;

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
	}
}