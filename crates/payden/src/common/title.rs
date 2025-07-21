use leptos::prelude::*;

#[component]
pub fn InputTitle(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <h2 class="font-body font-bold text-black text-lg">{ title }</h2>
            { children() }
        </div>
    }
}
