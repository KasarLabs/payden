use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;
use payden_model::*;
use reactive_stores::Store;

use crate::common::*;
use crate::sig;

#[component]
pub fn PageSend() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    // TODO: move this to url-as-source-of-truth
    let navigate = use_navigate();
    let recipient = model.address_send().get();
    let recipient = urlencoding::encode(&recipient);
    let amount = model.amount_send().get();
    let amount = urlencoding::encode(&amount);
    navigate(&format!("?r={recipient}&a={amount}"), Default::default());

    view! {
        <Form
            method="GET"
            action=""
            { .. }
            class="flex flex-col gap-4"
        >
            <InputTitle title="Recipient">
                <InputFieldAddress
                    address=sig! { model.address_send().get() }
                    address_update=sig! { address => model.address_send().set(address) }
                    url_encode="r"
                />
            </InputTitle>
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=sig! { model.amount_send().get() }
                    amount_update=sig! { amount => model.amount_send().set(amount) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! { logging::log!("Sending...") }
                message="Transaction Sent!"
            >
                Send
            </ButtonFullNotify>
        </Form>
    }
}
