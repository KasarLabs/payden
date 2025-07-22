use leptos::prelude::*;

use crate::{REGEX_AMOUNT, REGEX_HEX, utils::Field};
use private::*;

#[component]
pub fn InputFieldAmount(
    amount: impl Fn() -> String + Field,
    amount_update: impl Fn(String) + Field,
    url_encode: &'static str,
) -> impl IntoView {
    view! {
        <InputField
            text=amount
            text_update=amount_update
            text_prefix="$"
            text_pattern=REGEX_AMOUNT
            url_encode=url_encode
        />
    }
}

#[component]
pub fn InputFieldAddress(
    address: impl Fn() -> String + Field,
    address_update: impl Fn(String) + Field,
    url_encode: &'static str,
) -> impl IntoView {
    view! {
        <InputField
            text=address
            text_update=address_update
            text_prefix="0x"
            text_pattern=REGEX_HEX
            url_encode=url_encode
        />
    }
}

mod private {
    use leptos::prelude::*;

    use crate::{sig, utils::Field};

    #[component]
    pub fn InputField(
        text: impl Fn() -> String + Field,
        text_update: impl Fn(String) + Field,
        text_prefix: &'static str,
        text_pattern: &'static str,
        url_encode: &'static str,
    ) -> impl IntoView {
        let node_ref = NodeRef::new();

        view! {
            <div class="
                font-body text-base text-black
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                mt-0.5
                focus-within:pb-0.5 focus-within:mt-0
                focus-within:font-bold
                selection:bg-(--miden-branding) selection:text-white
            ">
                <div class="
                flex flex-row justify-start
                bg-white rounded-md 
                px-2.5 py-1.5
            ">
                    <span>{ text_prefix }</span>
                    <input
                        node_ref=node_ref
                        on:input:target=sig! { ev => text_update(ev.target().value()) }
                        on:focus=sig! { _ =>  if let Some(element) = node_ref.get_untracked() {
                            element.select();
                        }}
                        type="text"
                        name=url_encode
                        autocomplete="off"
                        pattern={text_pattern}
                        oninput="this.form.requestSubmit()"
                        required=true
                        prop:value=text
                        class="
                            truncate
                            focus:outline-none
                            grow
                        "
                    />
                </div>
            </div>
        }
    }
}
