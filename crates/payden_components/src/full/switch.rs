use leptos::prelude::*;
use leptos_use::*;

use crate::utils::Field;

#[component]
pub fn FullSwitch(
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
                flex flex-col justify-start
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
                grow
                flex flex-col justify-center
                px-14 p-1.5
            ">
                {children()}
            </div>
        </button>
    }
}
