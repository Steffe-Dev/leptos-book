use leptos::prelude::*;

// Clutch docs feature built into the #[component] macro
// ! Use it!!
/// Shows progress towards a goal
#[component]
pub fn ProgressBar(
	// optional prop
	// #[prop(optional)]
	// default prop
	/// The max value of the progress bar
	#[prop(default = 100)]
	max: u16,
	// Using impl syntax
	// progress: impl Fn() -> i32 + Send + Sync + 'static
	/// How much progress is displayed
	#[prop(into)]
	progress: Signal<i32>
) -> impl IntoView {
	view! {
		<progress
			max=max
			value=progress
		/>
		<br />
	}
}