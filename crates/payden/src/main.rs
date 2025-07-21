mod app;
mod common;
mod constants;
mod icons;
mod page;
mod preview;
mod utils;

use common::*;
use constants::*;
use icons::*;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    #[cfg(feature = "preview")]
    mount::mount_to_body(preview::App);
    #[cfg(not(feature = "preview"))]
    mount::mount_to_body(app::App);
}
