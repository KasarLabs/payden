use leptos::prelude::*;

use crate::{ICON_L, IconBook, IconGithub, IconMiden, LogoPayden};
use private::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="
            flex flex-row justify-between items-center
            w-screen
            py-4 px-8
        ">
            <a href="/wallet/send">
                <LogoPayden/>
            </a>
            <div class="flex flex-row gap-8">
                <LogoLink href="https://miden.xyz/">
                    <IconMiden size={ ICON_L } {..} class="fill-black"/>
                    miden
                </LogoLink>
                <LogoLink href="https://0xmiden.github.io/miden-docs/index.html">
                    <IconBook size={ ICON_L } {..} class="stroke-black stroke-[1.5]"/>
                    docs
                </LogoLink>
                <LogoLink href="https://github.com/KasarLabs/payden">
                    <IconGithub size={ ICON_L } {..} class="stroke-black stroke-[1.5]"/>
                    code
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
                class="
                    flex flex-row items-end gap-2
                    font-menu font-semibold text-2xl
                "
            >
                { children() }
            </a>
        }
    }
}
