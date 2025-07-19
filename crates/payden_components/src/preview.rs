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

    const ICON_SIZE: u32 = 18;

    #[cfg(feature = "button_primary")]
    mount::mount_to_body(sig! {
        view! {
            <Preview class="grid grid-cols-2 m-auto gap-8 border-1 border-red-600">
                <ButtonSwitch
                    on_press=sig!{ model.page().set(Page::Send) }
                    active=sig!{ model.page().get() == Page::Send }
                >
                    <IconSend size={ ICON_SIZE } {..} class="stroke-current stroke-1.5 m-auto"/>
                    Send
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=sig!{ model.page().set(Page::Receive) }
                    active=sig!{ model.page().get() == Page::Receive }
                >
                    <IconQr size={ ICON_SIZE } {..} class="stroke-current stroke-1.5 m-auto"/>
                    Receive
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=sig!{ model.page().set(Page::Faucet) }
                    active=sig!{ model.page().get() == Page::Faucet }
                >
                    <IconDrop size={ ICON_SIZE } {..} class="stroke-current stroke-1.5 m-auto"/>
                    Faucet
                </ButtonSwitch>
                <ButtonSwitch
                    on_press=sig!{ model.page().set(Page::Activity) }
                    active=sig!{ model.page().get() == Page::Activity }
                >
                    <IconPulse size={ ICON_SIZE } {..} class="stroke-current stroke-1.5 m-auto"/>
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

    model
        .addres()
        .set("0x84f5946bb3Bf4630Afe6aB94EAC561bD015F67c0".to_string());

    #[cfg(feature = "input_text")]
    mount::mount_to_body(sig! {
        view! {
            <Preview class="flex flex-row m-auto border-1 border-red-600">
                <InputText
                    text=sig! { model.addres().get() }
                    text_update=sig! { text => model.addres().update(|address| {
                        if text.len() < 2 {
                            *address = "0x".to_string();
                        } else if text.len() <= 42 {
                            *address = text;
                        }
                    }) }
                    text_validate=sig! { c => c.is_ascii_hexdigit() }
                />
            </Preview>
        }
    })
}
