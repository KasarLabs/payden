use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::prelude::*;

use private::*;

#[component]
pub fn WireButtonCopyAddress(address: impl Fn() -> String + Field, on_press: impl Fn() + Field) -> impl IntoView {
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
            ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top),
        )
    };

    let UseClipboardReturn { copy, .. } = use_clipboard();

    view! {
        <WireButton text=address on_press=sig! {{
            on_press();
            copy(&address());
            toast_dispatch();
        }}>
            <IconCopy size={ ICON_BASE } {..} class="stroke-1 stroke-current"/>
        </WireButton>
    }
}

mod private {
    use leptos::prelude::*;
    use leptos_use::*;

    use crate::prelude::*;

    #[component]
    pub fn WireButton(
        text: impl Fn() -> String + Field,
        on_press: impl Fn() + Send + Sync + 'static,
        children: Children,
    ) -> impl IntoView {
        const ANIMATION_DURATION: f64 = 150.0;

        let (animate, animate_set) = signal(false);
        let UseTimeoutFnReturn { start: animate_start, .. } = use_timeout_fn(
            move |_: ()| {
                animate_set.set(false);
            },
            ANIMATION_DURATION,
        );

        view! {
            <button
                type="button"
                on:click=sig! { _ => {
                    animate_set.set(true);
                    on_press();
                    animate_start(());
                }}
                style:cursor="pointer"
                class="
                flex flex-row w-full
                font-body text-base text-black
                border-current border-1
                bg-current rounded-md
                transition-all duration-{ANIMATION_DURATION}
            "
                class=(["mt-0.5"], sig! { !animate.get() })
                class=(["pb-0.5", "mt-0"], sig! { animate.get() })
            >
                <div class="
                flex flex-row gap-1.5 min-w-0 grow
                bg-white rounded-md 
                px-2.5 py-1.5 
            ">
                    { children() }
                    <p class="truncate">{ sig! { text() }}</p>
                </div>
            </button>
        }
    }
}
