use leptos::prelude::*;
use thaw::*;

use crate::{ICON_M, IconCopy, sig, utils::Field};

#[component]
pub fn Address(address: impl Fn() -> String + Field) -> impl IntoView {
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
            ToastOptions::default()
                .with_intent(ToastIntent::Success)
                .with_position(ToastPosition::Top),
        )
    };

    view! {
        <button
            on:click=sig! { _ => toast_dispatch() }
            class="
                flex flex-row gap-2 items-center
                font-body text-(--miden-branding) text-lg
                bg-white rounded-md
                p-1 pb-0
                cursor-pointer
            "
        >
            <IconCopy size={ ICON_M } {..} class="stroke-1 stroke-current"/>
            { sig! { address_short() }}
        </button>
    }
}
