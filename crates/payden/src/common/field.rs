use crate::prelude::*;
use leptos::prelude::*;

use private::*;

#[component]
pub fn InputFieldAmount(
    amount: impl Fn() -> Option<String> + Field,
    amount_update: impl Fn(String) + Field,
    validity_update: impl Fn(bool) + Field,
    url_encode: &'static str,
) -> impl IntoView {
    view! {
        <InputField
            text=amount
            text_update=amount_update
            validity_update=validity_update
            text_placeholder=PLACEHOLDER_AMOUNT
            text_prefix=PREFIX_AMOUNT
            text_pattern=REGEX_AMOUNT
            url_encode=url_encode
        />
    }
}

#[component]
pub fn InputFieldAddress(
    address: impl Fn() -> Option<String> + Field,
    address_update: impl Fn(String) + Field,
    validity_update: impl Fn(bool) + Field,
    url_encode: &'static str,
) -> impl IntoView {
    view! {
        <InputField
            text=address
            text_update=address_update
            validity_update=validity_update
            text_placeholder=PLACEHOLDER_ADDRESS
            text_prefix=PREFIX_ADDRESS
            text_pattern=REGEX_ADDRESS
            url_encode=url_encode
        />
    }
}

mod private {
    use crate::prelude::*;
    use leptos::prelude::*;

    #[component]
    pub fn InputField(
        text: impl Fn() -> Option<String> + Field,
        text_update: impl Fn(String) + Field,
        validity_update: impl Fn(bool) + Field,
        text_placeholder: &'static str,
        text_prefix: &'static str,
        text_pattern: &'static str,
        url_encode: &'static str,
    ) -> impl IntoView {
        let node_ref = NodeRef::<leptos::html::Input>::new();

        // We run an effect to synchronize input validity with the state of the DOM.
        // This works since effects always run a tick later than the component they are part of.
        Effect::new(move |_| {
            validity_update(node_ref.get().map(|input| input.check_validity()).unwrap_or(false));
        });

        view! {
            <div class="
                font-body text-base text-black
                bg-current rounded-md
                border-current border-1
                transition-all duration-{ANIMATION_DURATION}
                mt-0.5
                focus-within:pb-0.5 focus-within:mt-0
                selection:bg-(--miden-branding) selection:text-white
            ">
                <div class="
                flex flex-row justify-start gap-1
                bg-white rounded-md 
                px-2.5 py-1.5
            ">
                    <span class="font-bold">{ text_prefix }</span>
                    <input
                        node_ref=node_ref
                        on:input:target=sig! { ev => {
                            let target = ev.target();
                            let text = target.value();

                            if let Some(trimmed) = text.strip_prefix(text_prefix) && text.len() == LEN_ADDRESS {
                                // Pasted  in address, strip prefix.
                                // This callback will re-run on the updated value.
                                target.set_value(trimmed);
                            }

                            if let Some(input) = node_ref.get() && input.check_validity() {
                                validity_update(true);
                                text_update(text);
                            } else {
                                validity_update(false);
                            }

                        }}
                        on:focus=sig! { _ =>  if let Some(element) = node_ref.get_untracked() {
                            element.select();
                        }}
                        type="text"
                        name=url_encode
                        autocomplete="off"
                        pattern={text_pattern}
                        oninput="this.form.requestSubmit()"
                        required
                        prop:value=sig! { text().unwrap_or_default() }
                        prop:placeholder=text_placeholder
                        class="
                            truncate
                            font-normal
                            focus:outline-none
                            grow
                        "
                    />
                </div>
            </div>
        }
    }
}
