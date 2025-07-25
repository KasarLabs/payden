use crate::prelude::*;
use leptos::prelude::*;

use leptos_router::hooks::use_location;
use private::*;

#[component]
pub fn ButtonNavigateSend() -> impl IntoView {
    let location = use_location();

    view! {
        <ButtonNavigate
            active=sig!{ location.pathname.get().contains(PATH_SEND) }
            destination=PATH_SEND
        >
            <IconSend size={ ICON_L } {..} class="fill-current m-auto"/>
            Send
        </ButtonNavigate>
    }
}

#[component]
pub fn ButtonNavigateReceive() -> impl IntoView {
    let location = use_location();

    view! {
        <ButtonNavigate
            active=sig! { location.pathname.get().contains(PATH_RECEIVE) }
            destination=PATH_RECEIVE
        >
            <IconQr size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Receive
        </ButtonNavigate>
    }
}

#[component]
pub fn ButtonNavigateFaucet() -> impl IntoView {
    let location = use_location();

    view! {
        <ButtonNavigate
            active=sig! { location.pathname.get().contains(PATH_FAUCET) }
            destination=PATH_FAUCET
        >
            <IconDrop size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Faucet
        </ButtonNavigate>
    }
}

#[component]
pub fn ButtonNavigateActivity() -> impl IntoView {
    view! {
        <ButtonNavigate
            active=sig! { false }
            destination=PATH_SEND
        >
            <IconPulse size={ ICON_L } {..} class="stroke-current stroke-[1.5] m-auto"/>
            Activity
        </ButtonNavigate>
    }
}

mod private {
    use leptos::prelude::*;
    use leptos_use::*;

    use crate::prelude::*;

    #[component]
    pub fn ButtonNavigate(
        active: impl Fn() -> bool + Field,
        destination: &'static str,
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

        let on_click = move |_| {
            if !active() {
                animate_set.set(true);
                animate_start(());
            }
        };

        view! {
            <a href=destination class="flex grow">
                <button
                    on:click=on_click
                    class="
                    font-body text-xl font-extrabold
                    flex flex-col justify-start grow
                    bg-current rounded-md
                    border-current border-1
                    transition-all duration-{ANIMATION_DURATION}
                    cursor-pointer
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
            </a>
        }
    }
}
