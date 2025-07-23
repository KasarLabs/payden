use leptos::prelude::*;
use thaw::*;

use crate::{sig, utils::Field};

use privtate::*;

#[component]
pub fn ButtonFullNotify(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
    message_valid: &'static str,
    message_invalid: &'static str,
    children: Children,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let toast_dispatch_valid = move || {
        toaster.dismiss_all();
        toaster.dispatch_toast(
            sig! {
                view! {
                    <Toast>
                        <ToastTitle>
                            { message_valid }
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top),
        )
    };

    let toast_dispatch_invalid = move || {
        toaster.dismiss_all();
        toaster.dispatch_toast(
            sig! {
                view! {
                    <Toast>
                        <ToastTitle>
                            { message_invalid }
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Error).with_position(ToastPosition::Top),
        )
    };

    let on_press_valid = move || {
        on_press();
        toast_dispatch_valid()
    };

    let on_press_invalid = move || toast_dispatch_invalid();

    view! {
        <ButtonFull
            on_press_valid=on_press_valid
            on_press_invalid=on_press_invalid
            valid=active
        >
            { children() }
        </ButtonFull>
    }
}

mod privtate {
    use leptos::prelude::*;
    use leptos_use::*;

    use crate::{sig, utils::Field};

    #[component]
    pub fn ButtonFull(
        on_press_valid: impl Fn() + Field,
        on_press_invalid: impl Fn() + Field,
        valid: impl Fn() -> bool + Field,
        children: Children,
    ) -> impl IntoView {
        const ANIMATION_DURATION: f64 = 150.0;

        let (animate, animate_set) = signal(false);
        let UseTimeoutFnReturn { start: animate_start, .. } = use_timeout_fn(
            move |_: ()| {
                animate_set.set(false);
            },
            ANIMATION_DURATION + 50.0,
        );

        view! {
            <button
                // disabled=sig! { !active() }
                on:click=sig! { _  => {
                    if valid() {
                        on_press_valid();
                    } else {
                        on_press_invalid();
                    }
                    animate_set.set(true);
                    animate_start(());
                }}
                class="
                    font-body text-base text-white font-bold
                    bg-white rounded-md
                    border-1 border-black
                    transition-all duration-{ANIMATION_DURATION}
                    cursor-pointer
                "
                class=(["border-rose-400"], sig! { !valid() })
                class=(["pb-0.5", "mt-0.5"], sig! { !animate.get() })
                class=(["hover:pb-1", "hover:mt-0"], sig! { !animate.get() && valid() })
                class=(["pb-0", "mt-1"], sig! { animate.get() })
            >
                <div
                    class="
                        flex flex-col
                        bg-black rounded-md
                        p-1.5
                    "
                    class=(["bg-rose-400"], sig! { !valid() })
                >
                    { children() }
                </div>
            </button>
        }
    }
}
