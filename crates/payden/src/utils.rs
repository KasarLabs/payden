use leptos::prelude::*;
use leptos_meta::*;
use thaw::{ConfigProvider, ToasterProvider};

pub trait Field: Send + Sync + Copy + 'static {}
impl<T: Send + Sync + Copy + 'static> Field for T {}

#[macro_export]
macro_rules! sig {
    ($signal:expr) => {{ move || $signal }};
    ($signal:expr;) => {{ move || { $signal; } }};
    ($($param:tt),+ $(,)? => $signal:expr) => {{ move |$($param,)+| $signal }};
    (_ => $signal:expr) => {{ move |_| $signal }};
}

#[macro_export]
macro_rules! sig_async {
    ($signal:expr) => {{ move || Suspend::new(async move { $signal })}};
    ($signal:expr;) => {{ move || { Suspend::new(async move { $signal }); } }};
    ($($param:tt),+ $(,)? => $signal:expr) => {{ move |$($param,)+| Suspend::new(async move { $signal }) }};
    (_ => $signal:expr) => {{ move |_| Suspend::new(async move { $signal }) }};
}

#[component]
pub fn Preview(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    provide_meta_context();

    view! {
        <ConfigProvider>
            <ToasterProvider>
                <Stylesheet id="leptos" href="/style/output.css"/>
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <main>
                    <div class="flex flex-col min-h-screen">
                        <div class={class}>
                            { children() }
                        </div>
                    </div>
                </main>
            </ToasterProvider>
        </ConfigProvider>
    }
}
