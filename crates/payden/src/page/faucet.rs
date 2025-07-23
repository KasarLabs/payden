use leptos::Params;
use leptos::logging;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use payden_model::*;

use crate::prelude::*;

#[derive(Params, PartialEq)]
struct QuerySend {
    a: Option<String>, // amount
}

#[component]
pub fn PageFaucet() -> impl IntoView {
    let context = expect_context::<Context>();

    let query = use_query::<QuerySend>();
    let amount = move || query.read().as_ref().ok().and_then(|q| q.a.clone());

    let (valid_amount, valid_amount_set) = signal(true);

    Effect::new(move |_| {
        context.read().as_ref().map(|controller| {
            controller.borrow().model.page().set(Page::Faucet);
        })
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
                            .map(|controller| {
                                controller.borrow().model.amount_faucet().set(amount)
                            });
                    }}
                    validity_update=sig! { valid => valid_amount_set.set(valid) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! { logging::log!("Minting...") }
                active=sig! { valid_amount.get() }
                message="Requested Funds!"
            >
                Deposit
            </ButtonFullNotify>
        </Form>
    }
}
