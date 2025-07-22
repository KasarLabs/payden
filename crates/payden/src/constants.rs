pub const ICON_BASE: u32 = 24;
pub const ICON_M: u32 = 28;
pub const ICON_L: u32 = 32;
pub const ICON_XXL: u32 = 64;

pub const REGEX_HEX: &str = r#"[0-9a-f]{0,40}"#;
pub const REGEX_AMOUNT: &str = r#"\d+\.?\d*"#;

pub const PATH_DEFAULT: &str = "/wallet/send/?r=deadbeef&a=0.000000";
pub const PATH_SEND: &str = "/wallet/send?r=deadbeef&a=0.000000";
pub const PATH_RECEIVE: &str = "/wallet/receive";
pub const PATH_FAUCET: &str = "/wallet/faucet?a=0.000000";

pub const DEFAULT_ADDRESS: &str = "deadbeef";
pub const DEFAULT_AMOUNT: &str = "0.000000";
