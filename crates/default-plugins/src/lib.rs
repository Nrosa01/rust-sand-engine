use app_core::Plugin;
pub mod plugins;
use crate::plugins::*;

#[no_mangle]
pub fn plugin() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Sand::new()), Box::new(Water::new())]
}
