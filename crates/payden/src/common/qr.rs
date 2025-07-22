use crate::{sig, utils::Field};
use leptos::prelude::*;

#[component]
pub fn QrCode(data: impl Fn() -> String + Field) -> impl IntoView {
    let qr = move || {
        let qr = qrcode::QrCode::new(data())
            .unwrap()
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(50, 50)
            .quiet_zone(false)
            .build();

        let start = qr.find("<path").unwrap();
        let stop = qr.rfind("</svg>").unwrap();
        let qr = &qr[start..stop];

        format!(r#"<svg viewbox="0, 0, 58, 58" fill="none" xmlns="http://www.w3.org/2000/svg">{qr}</svg>"#)
    };

    view! {
        <div inner_html=sig! { qr() } class="
            min-x-50 min-y-50 max-w-[33vh] w-full
            p-8
            grow
        "/>
    }
}
