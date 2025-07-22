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
    a: Option<String>, // amount
}

#[component]
pub fn PageFaucet() -> impl IntoView {
    let context = expect_context::<Context>();

    let query = use_query::<QuerySend>();
    let amount = move || query.read().as_ref().ok().and_then(|q| q.a.clone()).unwrap_or(DEFAULT_AMOUNT.to_string());

    context.read().as_ref().map(|controller| {
        controller.model.page().set(Page::Faucet);
        controller.model.amount_faucet().set(amount());
    });

    view! {
        <Form
            method="GET"
            action=""
            { .. }
            class="flex flex-col gap-4"
        >
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=amount
                    amount_update=sig! { amount => {
                        context
                            .read()
                            .as_ref()
                            .map(|controller| controller.model.amount_faucet().set(amount));
                    }}
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
