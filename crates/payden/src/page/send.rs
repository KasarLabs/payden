use leptos::Params;
use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use payden_model::*;
use reactive_stores::Store;

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
    let model = expect_context::<Store<Model>>();
    model.page().set(Page::Send);

    let query = use_query::<QuerySend>();
    let recipient = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|q| q.r.clone())
            .unwrap_or(DEFAULT_ADDRESS.to_string())
    };
    let amount = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|q| q.a.clone())
            .unwrap_or(DEFAULT_AMOUNT.to_string())
    };

    model.address_send().set(recipient());
    model.amount_send().set(amount());

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
                    address_update=sig! { address => model.address_send().set(address) }
                    url_encode="r"
                />
            </InputTitle>
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=amount
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
