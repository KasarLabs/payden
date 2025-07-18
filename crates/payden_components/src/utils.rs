use leptos::prelude::*;
use leptos_meta::*;

#[macro_export]
macro_rules! sig {
    ($signal:expr) => {{ move || $signal }};
}

#[component]
pub fn Preview(class: &'static str, children: Children) -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class={class}>
                    { children() }
                </div>
            </div>
        </main>
    }
}
