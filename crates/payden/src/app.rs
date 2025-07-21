use leptos::prelude::*;
use leptos_meta::*;
use thaw::{ConfigProvider, ToasterProvider};

use crate::common::*;
use crate::constants::*;
use crate::icons::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <ToasterProvider>
                <Stylesheet id="leptos" href="/style/output.css"/>
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <main>
                    <div class="flex flex-col min-h-screen">
                        <Header/>
                    </div>
                </main>
            </ToasterProvider>
        </ConfigProvider>
    }
}
