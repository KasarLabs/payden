use leptos::logging;
use leptos::prelude::*;
use payden_model::*;
use reactive_stores::Store;

use crate::common::*;
use crate::sig;

#[component]
pub fn PageFaucet() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    view! {
        <InputTitle title="Amount">
            <InputFieldAmount
                amount=sig! { model.amount_faucet().get() }
                amount_update=sig! { amount => model.amount_faucet().set(amount) }
            />
        </InputTitle>
        <ButtonFullNotify
            on_press=sig! { logging::log!("Minting...") }
            message="Requested Funds!"
        >
            Deposit
        </ButtonFullNotify>
    }
}
