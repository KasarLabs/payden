use leptos::Params;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::prelude::*;

#[derive(Params, PartialEq)]
struct QuerySend {
    a: Option<String>, // amount
}

#[component]
pub fn PageFaucet() -> impl IntoView {
    let message_bus = expect_context::<MessageBus>();

    let query = use_query::<QuerySend>();
    let amount = move || query.read().as_ref().ok().and_then(|q| q.a.clone());

    let (valid_amount, valid_amount_set) = signal(true);

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
                    validity_update=sig! { valid => valid_amount_set.set(valid) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! {{
                    if let Some(amount) = amount() {
                        let amount = amount.parse().unwrap();
                        let message = payden_controller::ControllerAction::Mint { amount };
                        message_bus.dispatch(message);
                    }
                }}
                active=sig! { valid_amount.get() }
                message="Requested Funds!"
            >
                Deposit
            </ButtonFullNotify>
        </Form>
    }
}
