use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::{ICON_M, IconCopy, sig, utils::Field};

#[component]
pub fn Address(address: impl Fn() -> String + Field) -> impl IntoView {
    const ANIMATION_DURATION: f64 = 150.0;

    let (animate, set_animate) = signal(false);
    let UseTimeoutFnReturn {
        start: start_animate,
        ..
    } = use_timeout_fn(
        move |_: ()| {
            set_animate.set(false);
        },
        ANIMATION_DURATION + 50.0,
    );

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
                .with_position(ToastPosition::Bottom),
        )
    };

    view! {
        <button
            on:click=sig! { _ => {
                set_animate.set(true);
                start_animate(());
                toast_dispatch();
            }}
            class="
                font-body font-bold text-(--miden-branding) text-lg
                bg-current rounded-md
                transition-all duration-{ANIMATION_DURATION}
            "
            class=(["mt-0.5"], sig! { !animate.get() })
            class=(["pb-0.5", "mt-0"], sig! { animate.get() })
        >
            <div class="
                flex flex-row gap-2 items-center
                bg-white rounded-md
                border-1 border-white
                p-1
            ">
                <IconCopy size={ ICON_M } {..} class="stroke-[1.5] stroke-current"/>
                { sig! { address_short() }}
            </div>
        </button>
    }
}
