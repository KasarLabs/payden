use crate::{sig, utils::Field};
use leptos::prelude::*;

#[component]
pub fn QrCode(data: impl Fn() -> String + Field) -> impl IntoView {
    let qr = move || {
        qrcode::QrCode::new(data())
            .unwrap()
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(50, 50)
            .build()
            .replace("width=\"82\"", "")
            .replace("height=\"82\"", "")
    };

    view! {
        <div inner_html=sig! { qr() } class="size-100"/>
    }
}
