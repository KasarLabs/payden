use leptos::Params;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::prelude::*;

#[derive(Params, PartialEq)]
struct QuerySend {
    r: Option<String>, // recipient
    a: Option<String>, // amount
}

#[component]
pub fn PageSend() -> impl IntoView {
    let message_bus = expect_context::<MessageBus>();

    let query = use_query::<QuerySend>();
    let recipient = move || query.read().as_ref().ok().and_then(|q| q.r.clone());
    let amount = move || query.read().as_ref().ok().and_then(|q| q.a.clone());

    let (valid_recipient, valid_recipient_set) = signal(true);
    let (valid_amount, valid_amount_set) = signal(true);

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
                    validity_update=sig! { valid => valid_recipient_set.set(valid) }
                    url_encode="r"
                    {..}
                />
            </InputTitle>
            <InputTitle title="Amount">
                <InputFieldAmount
                    amount=amount
                    validity_update=sig! { valid => valid_amount_set.set(valid) }
                    url_encode="a"
                />
            </InputTitle>
            <ButtonFullNotify
                on_press=sig! {{
                    if let (Some(amount), Some(recipient)) = (amount(), recipient()) {
                        let amount = amount.parse().unwrap();
                        let recipient = format!("mtst1{recipient}");
                        let message = payden_controller::ControllerAction::Send { amount , recipient };
                        message_bus.dispatch(message);
                    }
                };}
                active=sig! { valid_recipient.get() && valid_amount.get() }
                message="Transaction Sent!"
            >
                Send
            </ButtonFullNotify>
        </Form>
    }
}
