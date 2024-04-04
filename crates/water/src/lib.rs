use app_core::{api::Particle, ParticleApi, Plugin};

struct Water;

pub fn move_if_empty(api: &mut ParticleApi, x: i32, y: i32) -> bool {
    if api.is_empty(x, y) {
        return api.move_to(x, y);
    }
    false
}

impl Plugin for Water {
    fn register(&mut self) -> app_core::api::ParticleCommonData {
        app_core::api::ParticleCommonData {
            name: String::from("Water"),
            color: app_core::Color::from_hex(0x00FFFF),
        }
    }

    fn update(&self, _: Particle, api: &mut ParticleApi) {
        let dir_x = api.gen_range(-1, 1);
        let dir_y = -1;

        let _ = move_if_empty(api, 0, dir_y) || 
                move_if_empty(api, dir_x, dir_y) || 
                move_if_empty(api, -dir_x, dir_y) || 
                move_if_empty(api, dir_x, 0) || 
                move_if_empty(api, -dir_x, 0);
    }
}

#[no_mangle]
pub fn plugin() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Water)]
}
