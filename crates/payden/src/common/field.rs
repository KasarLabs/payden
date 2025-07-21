use leptos::prelude::*;

use crate::{sig, utils::Field};

#[component]
fn InputField(
    text: impl Fn() -> String + Field,
    text_update: impl Fn(String) + Field,
    text_validate: impl Fn(char) -> bool + Field,
    text_prefix_len: u32,
) -> impl IntoView {
    let node_ref = NodeRef::new();

    view! {
        <form
            class="
                font-body text-base text-black
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                mt-0.5
                focus-within:pb-0.5 focus-within:mt-0
                focus-within:font-bold
                selection:bg-(--miden-branding) selection:text-white
            "
        >
            <div class="
                flex flex-row justify-start gap-1.5
                bg-white rounded-md 
                px-2.5 py-1.5
            ">
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
                    prop:value=text
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

#[component]
pub fn InputFieldAmount(
    amount: impl Fn() -> String + Field,
    amount_update: impl Fn(String) + Field,
) -> impl IntoView {
    let text_update = move |text_new: String| match text_new.len() {
        0..=1 => {
            amount_update("$0".to_string());
        }
        3 if text_new.starts_with("$0") && text_new.chars().nth(2).unwrap_or_default() != '.' => {
            amount_update(format!("${}", &text_new[2..]));
        }
        _ => {
            amount_update(text_new);
        }
    };

    let text_validate = move |c: char| c.is_ascii_digit() || (c == '.' && !amount().contains("."));

    view! {
        <InputField
            text=amount
            text_update=text_update
            text_validate=text_validate
            text_prefix_len=1
        />
    }
}

#[component]
pub fn InputFieldAddress(
    address: impl Fn() -> String + Field,
    address_update: impl Fn(String) + Field,
) -> impl IntoView {
    let text_update = move |text: String| {
        if text.len() < 2 {
            address_update("0x".to_string());
        } else if text.len() <= 42 {
            address_update(text);
        }
    };

    let text_validate = move |c: char| c.is_ascii_hexdigit();

    view! {
        <InputField
            text=address
            text_update=text_update
            text_validate=text_validate
            text_prefix_len=2
        />
    }
}
