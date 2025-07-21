use leptos::prelude::*;

#[component]
pub fn PageNotFound() -> impl IntoView {
    const SHRUG: &str = r#"¯\_(ツ)_/¯"#;
    view! {
        <div class="
            flex flex-col grow items-center
            font-body font-bold text-gray-300
        ">
            <p class="text-9xl">404</p>
            <p class="text-2xl">Not found</p>
            <p class="text-2xl">{ SHRUG }</p>
        </div>
    }
}
