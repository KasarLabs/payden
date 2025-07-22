use leptos::logging;
use leptos::prelude::*;
use payden_model::*;

use crate::common::*;
use crate::constants::*;
use crate::sig;

#[component]
pub fn PageReceive() -> impl IntoView {
    let context = expect_context::<Context>();

    context.read().as_ref().map(|controller| {
        controller.model.page().set(Page::Receive);
    });

    view! {
        <div class="flex flex-col grow items-center">
            <QrCode data=sig! {
                context.read()
                    .as_ref()
                    .expect("Controller has already loaded")
                    .model.address().get()
            }/>
        </div>
        <InputTitle title="Address">
            <WireButtonCopyAddress
                address=sig! {
                    context.read()
                        .as_ref()
                        .expect("Controller has already loaded")
                        .model.address().get()
                }
                on_press=sig! { logging::log!("Address copied") }
            />
        </InputTitle>
    }
}
