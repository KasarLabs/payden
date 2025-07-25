use leptos::prelude::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="
            flex flex-col 
            bg-white rounded-2xl
            w-fit
            border-1 border-gray-100
            drop-shadow-md
            p-8 m-8
        ">
            { children() }
        </div>
    }
}
