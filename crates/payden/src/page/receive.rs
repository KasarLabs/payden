use leptos::logging;
use leptos::prelude::*;
use payden_model::*;
use reactive_stores::Store;

use crate::common::*;
use crate::sig;

#[component]
pub fn PageReceive() -> impl IntoView {
    let model = expect_context::<Store<Model>>();

    view! {
        <QrCode data=sig! { model.address().get() }/>
        <InputTitle title="Address">
            <WireButtonCopyAddress
                address=sig! { model.address().get() }
                on_press=sig! { logging::log!("Address copied") }
            />
        </InputTitle>
    }
}
