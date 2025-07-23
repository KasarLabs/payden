use leptos::Params;
use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use payden_model::*;

use crate::common::*;
use crate::constants::*;
use crate::sig;

#[derive(Params, PartialEq)]
struct QuerySend {
    r: Option<String>, // recipient
    a: Option<String>, // amount
}

#[component]
pub fn PageSend() -> impl IntoView {
    let context = expect_context::<Context>();

    let query = use_query::<QuerySend>();
    let recipient = move || query.read().as_ref().ok().and_then(|q| q.r.clone());
    let amount = move || query.read().as_ref().ok().and_then(|q| q.a.clone());

    let (valid_recipient, valid_recipient_set) = signal(true);
    let (valid_amount, valid_amount_set) = signal(true);

    Effect::new(move |_| {
        context.read().as_ref().map(|controller| {
            controller.model.page().set(Page::Send);
        })
    });

    view! {
        <Form
            method="GET"
            action=""
            { .. }
            class="flex flex-col gap-4"
        >
            <InputTitle title="Recipient">
                <InputFieldAddress
                    address=recipient
                    address_update=sig! { address => {
                        context.read()
                            .as_ref()
                            .map(|controller| controller.model.address_send().set(address));
                    }}
                    validity_update=sig! { valid => valid_recipient_set.set(valid) }
                    url_encode="r"
                    {..}
                />
            </InputTitle>
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=amount
                    amount_update=sig! { amount => {
                        context.read()
                            .as_ref()
                            .map(|controller| controller.model.amount_send().set(amount));
                    }}
                    validity_update=sig! { valid => valid_amount_set.set(valid) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! { logging::log!("Sending...") }
                active=sig! { valid_recipient.get() && valid_amount.get() }
                message_valid="Transaction Sent!"
                message_invalid="Invalid recipient or amount!"
            >
                Send
            </ButtonFullNotify>
        </Form>
    }
}
