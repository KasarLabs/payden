use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::prelude::*;

#[component]
pub fn PageLoading() -> impl IntoView {
    const ANIMATION_DURATION: u64 = 400;
    const ANIMATION_FRAMES: [&str; 4] = ["", ".", "..", "..."];

    let (index, index_set) = signal(0);
    let _pausable = use_interval_fn(
        move || {
            index_set.update(|index| *index = (*index + 1) % ANIMATION_FRAMES.len());
        },
        ANIMATION_DURATION,
    );

    view! {
        <div class="
            flex flex-row items-center justify-center grow gap-2
            font-body font-bold italic text-xl
        ">
            <p class="test-gray-400">Loading account data</p>
            <p class="text-(--miden-branding)">{ sig! { ANIMATION_FRAMES[index.get()] } }</p>
        </div>
    }
}
