use leptos::prelude::*;
use thaw::*;

#[component]
pub fn ToastCopy(text: impl Fn() -> String + Send + Sync + Copy + 'static) -> impl IntoView {
    view! {
        <Toast>
            <ToastTitle>
                Address Copied!
            </ToastTitle>
            <ToastBody>
                { text() }
            </ToastBody>
        </Toast>
    }
}
