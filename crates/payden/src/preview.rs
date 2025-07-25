use leptos::prelude::*;

use crate::sig;
use crate::utils::*;
use leptos::logging;
use payden::*;
use payden_model::*;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    let model = Store::new(Model::default());
    model.address().set("0x84f5946bb3Bf4630Afe6aB94EAC561bD015F67c0".to_string());

    if cfg!(feature = "icons") {
        view! {
            <Preview class="flex flex-col m-auto border-1 border-red-600">
                <div class="flex flex row m-8 gap-8">
                    <LogoPayden/> <LogoKasar/>
                </div>
                <div class="grid grid-cols-5 gap-8">
                    <IconBook size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconClose size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconCopy size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconDrop size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconGithubCircle size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconGithub size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconMiden size={ ICON_XXL } {..} class="fill-(--miden-branding)"/>
                    <IconPulse size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconQr size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconSend size={ ICON_XXL } {..} class="fill-black stroke-[1.5]"/>
                    <IconSettings size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconTelegram size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                    <IconTwitter size={ ICON_XXL } {..} class="stroke-black stroke-[1.5]"/>
                </div>
            </Preview>
        }
    } else if cfg!(feature = "components") {
        view! {
            <Preview class="m-auto bg-orange-50">
                <Card>
                    <div class="flex flex-col gap-4 border-1 border-red-600">
                        <Address address=sig! { model.address().get() }/>
                        <Balance balance=sig! { model.balance().get() }/>
                        <div class="grid grid-cols-2 grow gap-8 border-1 border-red-600 grow">
                            <ButtonNavigateSend/>
                            <ButtonNavigateReceive/>
                            <ButtonNavigateFaucet/>
                            <ButtonNavigateActivity/>
                        </div>
                        <WireButtonCopyAddress address=sig! { model.address().get() } on_press=sig! { logging::log!("Copy") }/>
                        <InputFieldAddress
                            address=sig! { Some(model.address().get()) }
                            validity_update=sig! { _ => () }
                            url_encode="r"
                        />
                        <InputFieldAmount
                            amount=sig! { Some("100".to_string()) }
                            validity_update=sig! { _ => () }
                            url_encode="a"
                        />
                        <ButtonFullNotify
                            on_press=sig! { logging::log!("Send") }
                            active=sig! { true }
                            message="Transaction Sent!"
                        >
                            Send
                        </ButtonFullNotify>
                    </div>
                </Card>
            </Preview>
        }
    } else if cfg!(feature = "banners") {
        view! {
            <Preview  class="
                flex flex-col justify-between
                border-1 border-red-600
                bg-orange-50
                h-screen
            ">
                <Header/>
                <Footer/>
            </Preview>
        }
    } else if cfg!(feature = "qr") {
        view! {
            <Preview  class="
                    border-1 border-red-600
                    m-auto
                ">
                <QrCode data=sig!{ model.address().get() }/>
            </Preview>
        }
    } else {
        view! {
            <Preview  class="
                    border-1 border-red-600
                    m-auto
                ">
                Preview
            </Preview>
        }
    }
}
