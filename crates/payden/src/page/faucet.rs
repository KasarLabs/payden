use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;
use payden_model::*;
use reactive_stores::Store;

use crate::common::*;
use crate::sig;

#[component]
pub fn PageFaucet() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    let navigate = use_navigate();
    let amount = model.amount_send().get();
    let amount = urlencoding::encode(&amount);
    navigate(&format!("?a={amount}"), Default::default());

    view! {
        <Form
            method="GET"
            action=""
            { .. }
            class="flex flex-col gap-4"
        >
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=sig! { model.amount_faucet().get() }
                    amount_update=sig! { amount => model.amount_faucet().set(amount) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! { logging::log!("Minting...") }
                message="Requested Funds!"
            >
                Deposit
            </ButtonFullNotify>
        </Form>
    }
}
