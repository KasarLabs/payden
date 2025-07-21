use leptos::prelude::*;
use leptos_use::*;

use crate::{ICON_L, IconDrop, IconPulse, IconQr, IconSend, utils::Field};

#[component]
fn ButtonToggle(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
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
        ANIMATION_DURATION + 50.0,
    );

    let on_click = move |_| {
        if !active() {
            on_press();
            animate_set.set(true);
            animate_start(());
        }
    };

    view! {
        <button
            on:click=on_click
            class="
                font-body text-xl font-extrabold
                flex flex-col justify-start grow
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
            "
            // Initial state
            class=([ "text-slate-500", "mt-1.5" ], move || !active())
            // Animation part 1 -hover
            class=([ "hover:pb-0.5", "hover:mt-0.5" ], move || !animate.get() && !active())
            // Animation part 2 -pressed
            class=([ "text-orange-500", "mt-1.5" ], move || animate.get() && active())
            // Animation part 3 -released
            class=([ "text-(--miden-branding)", "pb-1.5", "mt-0" ], move || !animate.get() && active())
        >
            <div class="
                bg-white rounded-md
                flex flex-col justify-center grow
                p-1.5
            ">
                {children()}
            </div>
        </button>
    }
}

#[component]
pub fn ButtonToggleSend(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
) -> impl IntoView {
    view! {
        <ButtonToggle
            on_press=on_press
            active=active
        >
            <IconSend size={ ICON_L } {..} class="fill-current m-auto"/>
            Send
        </ButtonToggle>
    }
}

#[component]
pub fn ButtonToggleReceive(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
) -> impl IntoView {
    view! {
        <ButtonToggle
            on_press=on_press
            active=active
        >
            <IconQr size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Receive
        </ButtonToggle>
    }
}

#[component]
pub fn ButtonToggleFaucet(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
) -> impl IntoView {
    view! {
        <ButtonToggle
            on_press=on_press
            active=active
        >
            <IconDrop size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Faucet
        </ButtonToggle>
    }
}

#[component]
pub fn ButtonToggleActivity(
    on_press: impl Fn() + Field,
    active: impl Fn() -> bool + Field,
) -> impl IntoView {
    view! {
        <ButtonToggle
            on_press=on_press
            active=active
        >
            <IconPulse size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Activity
        </ButtonToggle>
    }
}
