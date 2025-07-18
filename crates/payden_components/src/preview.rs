mod utils;

use leptos::prelude::*;
use leptos::{logging, mount};
use payden_components::*;
use utils::*;

fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");

    #[cfg(feature = "button_primary")]
    mount::mount_to_body(sig! {
        view! {
            <Preview class="flex flex-col justify-end m-auto">
                <ButtonPrimary on_press=|| { logging::log!("Button pressed") }>
                    <IconSend size=24 {..} class="stroke-current stroke-1 m-auto"/>
                    Send
                </ButtonPrimary>
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
