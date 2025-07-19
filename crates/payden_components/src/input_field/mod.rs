use leptos::{logging, prelude::*};
use leptos_use::*;

use crate::{IconCopy, sig};

#[component]
pub fn InputText(
    text: impl Fn() -> String + Send + Sync + Copy + 'static,
    text_update: impl Fn(String) + Send + Sync + Copy + 'static,
    text_validate: impl Fn(char) -> bool + Send + Sync + Copy + 'static,
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
        <form
            class="
                font-body text-sm text-black font-bold
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                focus-within:pb-0.5 focus-within:mt-0
            "
            class=(["mt-0.5"], sig! { !animate.get() })
            class=(["pb-0.5", "mt-0"], sig! { animate.get() })
        >
            <div class="
                flex flex-row justify-center gap-1.5
                bg-white rounded-md
                p-1.5
            ">
                <button
                    type="button"
                    on:mousedown=sig! { ev => ev.prevent_default() }
                    on:click=sig! { _ => {
                        animate_set.set(true);
                        animate_start(())
                    }}
                >
                    <IconCopy size=20 {..} class="stroke-1.5 stroke-current"/>
                </button>
                <input
                    on:keypress=sig! { ev => {
                        let key = ev.key();
                        if key.len() == 1 {
                            let c = key.chars().next().expect("Checked above");
                            if !text_validate(c) {
                                ev.prevent_default();
                            }
                        }
                    }}
                    on:input:target=sig! { ev => text_update(ev.target().value()) }
                    type="text"
                    prop:value=sig! { text() }
                    class="
                        text-ellipsis
                        focus:outline-none
                    "
                />
            </div>
        </form>
    }
}
