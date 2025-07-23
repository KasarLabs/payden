use crate::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Balance(balance: impl Fn() -> f64 + Field) -> impl IntoView {
    view! {
        <div class="
            text-black text-7xl
            font-body font-extrabold
            px-2
        ">
            ${ sig! { format!("{:.2}", balance()) } }
        </div>
    }
}
