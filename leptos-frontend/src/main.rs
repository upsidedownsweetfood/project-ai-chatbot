use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
	let (count, set_count) = signal(0);

	view! {
		<button on:click=move |_| *set_count.write() += 1>
			"click me: " {count}
		</button>
		<p>
			"Double count: " {move || count.get() * 2}
		</p>
	}
}
