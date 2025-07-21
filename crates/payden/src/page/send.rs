use leptos::logging;
use leptos::prelude::*;
use payden_model::*;
use reactive_stores::Store;

use crate::common::*;
use crate::sig;

#[component]
pub fn PageSend() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    view! {
        <InputTitle title="Recipient">
            <InputFieldAddress
                address=sig! { model.address().get() }
                address_update=sig! { address => model.address().set(address) }
            />
        </InputTitle>
        <InputTitle title="Amount">
            <InputFieldAmount
                amount=sig! { model.amount_send().get() }
                amount_update=sig! { amount => model.amount_send().set(amount) }
            />
        </InputTitle>
        <ButtonFullNotify
            on_press=sig! { logging::log!("Sending...") }
            message="Transaction Sent!"
        />
    }
}
