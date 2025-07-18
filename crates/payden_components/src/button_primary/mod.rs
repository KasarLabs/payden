use leptos::prelude::*;
use leptos_use::*;

#[component]
pub fn ButtonPrimary(
    on_press: impl Fn() + Send + Sync + 'static,
    children: Children,
) -> impl IntoView {
    const ANIMATION_DURATION: f64 = 150.0;

    let (pressed, set_pressed) = signal(false);
    let (active, set_active) = signal(false);
    let UseTimeoutFnReturn {
        start: start_pressed,
        ..
    } = use_timeout_fn(
        move |_: ()| {
            set_pressed.set(false);
            set_active.set(true);
        },
        ANIMATION_DURATION,
    );

    let on_click = move |_| {
        on_press();
        set_pressed.set(true);
        start_pressed(());
    };

    view! {
        <button
            on:click=on_click
            class="
                font-body
                flex flex-col justify-start
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
            "
            class=([ "pb-0.5", "mb-0.5"], move || { pressed.get() })
            class=([ "hover:pb-1", "hover:mb-1" ], move || { !pressed.get() && !active.get() })
            class=([ "text-slate-500" ], move || { !active.get() })
            class=([ "text-(--miden-branding)" ,"pb-1", "mb-1" ], move || { active.get() })
        >
            <div class="
                bg-white rounded-md
                flex flex-col justify-center
                px-14 p-1.5
            ">
                {children()}
            </div>
        </button>
    }
}
