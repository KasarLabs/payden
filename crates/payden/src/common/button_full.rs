use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::{sig, utils::Field};

#[component]
pub fn ButtonFull(on_press: impl Fn() + Field, children: Children) -> impl IntoView {
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
            on:click=sig! { _  => {
                on_press();
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
            class=(["pb-0.5", "mt-0.5", "hover:pb-1", "hover:mt-0"], sig! { !animate.get() })
            class=(["pb-0", "mt-1"], sig! { animate.get() })
        >
            <div class="
                flex flex-col
                bg-black rounded-md
                p-1.5
            ">
                { children() }
            </div>
        </button>
    }
}

#[component]
pub fn ButtonFullNotify(on_press: impl Fn() + Field, message: &'static str, children: Children) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let toast_dispatch = move || {
        toaster.dismiss_all();
        toaster.dispatch_toast(
            sig! {
                view! {
                    <Toast>
                        <ToastTitle>
                            { message }
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top),
        )
    };

    let on_press = move || {
        on_press();
        toast_dispatch()
    };

    view! {
        <ButtonFull on_press=on_press>
            { children() }
        </ButtonFull>
    }
}
