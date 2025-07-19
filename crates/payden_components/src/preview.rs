mod utils;

use leptos::prelude::*;
use leptos::{logging, mount};
use payden_components::*;
use payden_model::*;
use reactive_stores::Store;
use utils::*;

fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");

    let model = Store::new(Model::default());

    #[cfg(feature = "button_primary")]
    mount::mount_to_body(sig! {
        view! {
            <Preview class="grid grid-cols-2 m-auto gap-8 border-1 border-red-600">
                <ButtonSwitch
                    on_press=move || { model.page().set(Page::Send) }
                    active=move || { model.page().get() == Page::Send }
                >
                    <IconSend size=24 {..} class="stroke-current stroke-1 m-auto"/>
                    Send
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=move || { model.page().set(Page::Receive) }
                    active=move || { model.page().get() == Page::Receive }
                >
                    <IconQr size=24 {..} class="stroke-current stroke-1 m-auto"/>
                    Receive
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=move || { model.page().set(Page::Faucet) }
                    active=move || { model.page().get() == Page::Faucet }
                >
                    <IconDrop size=24 {..} class="stroke-current stroke-1 m-auto"/>
                    Faucet
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=move || { model.page().set(Page::Activity) }
                    active=move || { model.page().get() == Page::Activity }
                >
                    <IconPulse size=24 {..} class="stroke-current stroke-1 m-auto"/>
                    Activity
                </ButtonSwitch>
            </Preview>
        }
    });

    #[cfg(feature = "icons")]
    mount::mount_to_body(sig! {
        view! {
            <Preview class="grid grid-cols-5 m-auto gap-8">
                <IconBook size=64 {..} class="stroke-black stroke-1.5"/>
                <IconClose size=64 {..} class="stroke-black stroke-1.5"/>
                <IconCopy size=64 {..} class="stroke-black stroke-1.5"/>
                <IconDrop size=64 {..} class="stroke-black stroke-1.5"/>
                <IconGithubCircle size=64 {..} class="stroke-black stroke-1.5"/>
                <IconGithub size=64 {..} class="stroke-black stroke-1.5"/>
                <IconMiden size=64 {..} class="fill-(--miden-branding)"/>
                <IconPulse size=64 {..} class="stroke-black stroke-1.5"/>
                <IconQr size=64 {..} class="stroke-black stroke-1.5"/>
                <IconSend size=64 {..} class="fill-black stroke-1.5"/>
                <IconSettings size=64 {..} class="stroke-black stroke-1.5"/>
                <IconTelegram size=64 {..} class="stroke-black stroke-1.5"/>
                <IconTwitter size=64 {..} class="stroke-black stroke-1.5"/>
            </Preview>
        }
    });
}
