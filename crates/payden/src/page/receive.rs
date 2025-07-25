use leptos::logging;
use leptos::prelude::*;
use payden_model::*;

use crate::prelude::*;

#[component]
pub fn PageReceive() -> impl IntoView {
    let context = expect_context::<Context>();

    view! {
        <div class="flex flex-col grow items-center">
            <QrCode data=sig! {
                context.read()
                    .as_ref()
                    .map(|controller| controller.borrow().model.address().get())
                    .unwrap_or_default()
            }/>
        </div>
        <InputTitle title="Address" center=true>
            <WireButtonCopyAddress
                address=sig! {
                    context.read()
                        .as_ref()
                        .map(|controller| controller.borrow().model.address().get())
                        .unwrap_or_default()
                }
                on_press=sig! { logging::log!("Address copied") }
            />
        </InputTitle>
    }
}
