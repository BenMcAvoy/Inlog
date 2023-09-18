pub const BLUE: &str = "\x1b[34m";
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const GREEN: &str = "\x1b[32m";
pub const NORMAL: &str = "\x1b[0m";
pub const LIGHT_BLUE: &str = "\x1b[94m";

pub fn colour(colour: &str, string: &str) -> String {
    format!("{colour}{string}{NORMAL}")
}
