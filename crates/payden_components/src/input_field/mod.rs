use leptos::prelude::*;
use leptos_use::*;
use thaw::*;

use crate::{IconCopy, sig, toast_copy::ToastCopy};

#[component]
pub fn InputField(
    text: impl Fn() -> String + Send + Sync + Copy + 'static,
    text_update: impl Fn(String) + Send + Sync + Copy + 'static,
    text_validate: impl Fn(char) -> bool + Send + Sync + Copy + 'static,
    text_prefix_len: u32,
    #[prop(default = false)] copy: bool,
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

    let node_ref = NodeRef::new();

    let address_short = move || {
        let start = text().chars().take(6).collect::<String>();
        let stop = text().chars().rev().take(4).collect::<String>();
        format!("{start}...{stop}")
    };

    let toaster = ToasterInjection::expect_context();
    let toast_dispatch = move || {
        toaster.dismiss_all();
        toaster.dispatch_toast(
            sig! {
                view! {
                    <ToastCopy text={address_short}/>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Success)
                .with_position(ToastPosition::Bottom),
        )
    };

    let copy = copy.then(|| {
        view! {
            <button
                type="button"
                on:mousedown=sig! { ev => ev.prevent_default() }
                on:click=sig! { _ => {
                    animate_set.set(true);
                    toast_dispatch();
                    animate_start(())
                }}
            >
                <IconCopy size=24 {..} class="stroke-1.5 stroke-current"/>
            </button>
        }
    });

    view! {
        <form
            class="
                font-body text-base text-black font-bold
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                focus-within:pb-0.5 focus-within:mt-0
            "
            class=(["mt-0.5"], sig! { !animate.get() })
            class=(["pb-0.5", "mt-0"], sig! { animate.get() })
        >
            <div class="
                flex flex-row justify-start gap-1.5
                bg-white rounded-md
                p-1.5
            ">
                {copy}
                <input
                    node_ref=node_ref
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
                    on:focus=sig! { _ =>  if let Some(element) = node_ref.get_untracked() {
                        element.select();
                        let _ = element.set_selection_direction(Some("backward"));
                        let _ = element.set_selection_start(Some(text_prefix_len));
                        let _ = element.set_selection_end(Some(text().len() as u32));
                    }}
                    type="text"
                    prop:value=sig! { text() }
                    class="
                        truncate
                        focus:outline-none
                        grow
                    "
                />
            </div>
        </form>
    }
}
