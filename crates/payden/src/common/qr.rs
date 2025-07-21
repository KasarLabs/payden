use crate::{sig, utils::Field};
use leptos::prelude::*;

#[component]
pub fn QrCode(data: impl Fn() -> String + Field) -> impl IntoView {
    let qr = move || {
        qrcode::QrCode::new(data())
            .unwrap()
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(50, 50)
            .quiet_zone(false)
            .build()
            .replace("width=\"66\"", "")
            .replace("height=\"66\"", "")
    };

    view! {
        <div inner_html=sig! { qr() } class="min-x-50 min-y-50 p-8 grow"/>
    }
}
