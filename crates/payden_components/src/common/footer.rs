use leptos::prelude::*;

use crate::{ICON_BASE, IconGithubCircle, IconTelegram, IconTwitter, LogoKasar};
use private::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="
            flex flex-row justify-between items-center
            py-4 px-8
            bg-white
        ">
            <LogoKasar/>
            <div class="flex flex-row gap-4">
                <LogoLink href="https://x.com/kasarLabs">
                    <IconTwitter size={ ICON_BASE }/>
                </LogoLink>
                <LogoLink href="https://t.me/yourcompany">
                    <IconTelegram size={ ICON_BASE }/>
                </LogoLink>
                <LogoLink href="https://github.com/kasarlabs">
                    <IconGithubCircle size={ ICON_BASE }/>
                </LogoLink>
            </div>
        </div>
    }
}

mod private {
    use leptos::prelude::*;

    #[component]
    pub fn LogoLink(href: &'static str, children: Children) -> impl IntoView {
        view! {
            <a
                href=href
                target="_blank"
                class="stroke-[1.5] stroke-(--miden-branding)">
                { children() }
            </a>
        }
    }
}
