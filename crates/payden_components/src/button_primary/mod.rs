use leptos::prelude::*;

#[component]
pub fn ButtonPrimary(text: &'static str) -> impl IntoView {
    view! {
        <button class="flex flex-col">
            {text}
        </button>
    }
}
