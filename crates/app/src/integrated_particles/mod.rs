use super::app_core::api::Plugin;
pub mod plugins;
use plugins::*;

#[no_mangle]
pub fn plugin() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Sand::new()), Box::new(Water::new()), Box::new(Dust::new()), Box::new(Steam::new()), Box::new(Lava::new()), Box::new(Rock::new())]
}
