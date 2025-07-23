extern crate alloc;

use leptos::prelude::*;

pub const ICON_BASE: u32 = 24;
pub const ICON_M: u32 = 28;
pub const ICON_L: u32 = 32;
pub const ICON_XXL: u32 = 64;

pub const PATH_DEFAULT: &str = "/wallet/send";
pub const PATH_SEND: &str = "/wallet/send";
pub const PATH_RECEIVE: &str = "/wallet/receive";
pub const PATH_FAUCET: &str = "/wallet/faucet";

pub const REGEX_ADDRESS: &str = r#"[0-9a-z]{32}"#;
pub const REGEX_AMOUNT: &str = r#"\d+\.?\d{0,2}"#;

pub const PREFIX_ADDRESS: &str = "mtst1";
pub const PREFIX_AMOUNT: &str = "$";

pub const PLACEHOLDER_ADDRESS: &str = "deadbeef";
pub const PLACEHOLDER_AMOUNT: &str = "0.00";

pub const LEN_ADDRESS: usize = 37;

pub type Context = LocalResource<payden_controller::Controller>;
