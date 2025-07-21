use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::{ICON_BASE, IconCopy, sig, utils::Field};

// TODO: refactor components into twp types: full and wire
#[component]
pub fn WireButton(
    text: impl Fn() -> String + Field,
    on_press: impl Fn() + Field,
    children: Children,
) -> impl IntoView {
    const ANIMATION_DURATION: f64 = 150.0;

    let (animate, animate_set) = signal(false);
    let UseTimeoutFnReturn {
        start: animate_start,
        ..
    } = use_timeout_fn(
        move |_: ()| {
            animate_set.set(false);
        },
        ANIMATION_DURATION,
    );

    view! {
        <button
            type="button"
            // TODO: extract the  button logic into a separate container
            on:click=sig! { _ => {
                animate_set.set(true);
                on_press();
                animate_start(());
            }}
            style:cursor="pointer"
            class="
                font-body text-base text-black font-bold
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                flex flex-row
            "
            class=(["mt-0.5"], sig! { !animate.get() })
            class=(["pb-0.5", "mt-0"], sig! { animate.get() })
        >
            <div class="
                flex flex-row justify-start gap-1.5 grow
                bg-white rounded-md 
                px-2.5 py-1.5 
            ">
                { children() }
                { text }
            </div>
        </button>
    }
}

#[component]
pub fn WireButtonCopyAddress(
    address: impl Fn() -> String + Field,
    on_press: impl Fn() + Field,
) -> impl IntoView {
    let address_short = move || {
        let start = address().chars().take(6).collect::<String>();
        let stop = address().chars().rev().take(4).collect::<String>();
        format!("{start}...{stop}")
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

    let on_press = move || {
        on_press();
        toast_dispatch()
    };

    view! {
        <WireButton text=address on_press=on_press>
            <IconCopy size={ ICON_BASE } {..} class="stroke-[1.5] stroke-current"/>
        </WireButton>
    }
}
