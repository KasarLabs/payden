use leptos::prelude::*;

use crate::{sig, utils::*};

#[component]
pub fn Balance(balance: impl Fn() -> f64 + Field) -> impl IntoView {
    view! {
        <div class="
            text-black text-7xl
            font-body font-extrabold
            border-1 border-red-600
            px-2 py-4
        ">
            ${ sig! { format!("{:.2}", balance()) } }
        </div>
    }
}
