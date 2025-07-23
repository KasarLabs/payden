use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::prelude::*;

#[component]
pub fn Address(address: impl Fn() -> String + Field) -> impl IntoView {
    let message_bus = expect_context::<MessageBus>();

    let address_short = move || {
        let address = address();
        if address.len() > 12 {
            let start = address.chars().take(6).collect::<String>();
            let stop = address.chars().rev().take(4).collect::<String>();
            format!("{start}...{stop}")
        } else {
            address
        }
    };

    let toaster = ToasterInjection::expect_context();
    let toast_dispatch = move || {
        toaster.dismiss_all();
        toaster.dispatch_toast(
            sig! {
                view! {
                    <Toast>
                        <ToastTitle>
                            Address Copied!
                        </ToastTitle>
                        <ToastBody>
                            { address_short() }
                        </ToastBody>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top),
        )
    };

    let UseClipboardReturn { copy, .. } = use_clipboard();

    view! {
        <div class="
            flex flex-row gap-2 items-center justify-between
            font-body text-(--miden-branding) text-lg
            p-1 pb-0
        ">
            <button
                on:click=sig!{ _ => {
                    copy(&address());
                    toast_dispatch();
                }}
                class="
                    flex flex-row gap-2 items-center
                    bg-white
                    cursor-pointer
                "
            >
                <IconCopy size={ ICON_M } { .. } class="stroke-1 stroke-current"/>
                { sig! { address_short() }}
            </button>
            <button
                on:click=sig! { _ => {
                    message_bus.dispatch(payden_controller::ControllerAction::Refresh);
                }}
                class="
                    flex flex-row gap-2 place-items-center
                    bg-white
                    cursor-pointer
                "
            >
                <IconRefresh size={ ICON_M } { .. } class="stroke-1 stroke-current"/>
            </button>
        </div>
    }
}
