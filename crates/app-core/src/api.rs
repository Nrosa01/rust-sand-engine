use macroquad::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub id: usize,
    pub clock: bool,
}

impl Particle {
    pub const EMPTY: Particle = Particle {
        id: 0,
        clock: false,
    };

    pub(crate) const INVALID: Particle = Particle {
        id: usize::MAX,
        clock: false,
    };
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub struct ParticleDefinition {
    pub name: String,
    pub update_func: fn(Particle, &mut GameState) -> (),
    pub color: Color,
}

#[derive(Debug)]
pub struct GameState {
    pub particles: Vec<Vec<Particle>>,
    particle_definitions: Vec<ParticleDefinition>,
    current_x: usize,
    current_y: usize,
    pub width: usize,
    pub height: usize,
    image: Image,
    texture: Texture2D,
    clock: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> GameState {
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Set the filter mode to nearest to avoid blurring the pixels

        GameState {
            particles: vec![
                vec![
                    Particle {
                        id: 0,
                        clock: false
                    };
                    width
                ];
                height
            ],
            current_x: 0,
            current_y: 0,
            width,
            height,
            particle_definitions: vec![ParticleDefinition {
                name: String::from("empty"),
                update_func: |_, _| {}, // Función vacía
                color: BLACK,
            }],
            image: image,
            texture: texture,
            clock: false,
        }
    }

    pub fn get_particle_definitions(&self) -> &Vec<ParticleDefinition> {
        &self.particle_definitions
    }

    pub fn add_particle_definition(&mut self, particle_definition: ParticleDefinition) -> () {
        self.particle_definitions.push(particle_definition);

        // Print the name of the particle definition
        println!(
            "Added particle definition: {}",
            self.particle_definitions.last().unwrap().name
        );
    }

    pub fn set_particle(&mut self, x: usize, y: usize, id: usize) -> () {
        if !self.is_inside(x, y) {
            return;
        }

        self.particles[y][x].id = id;
        self.particles[y][x].clock = !self.clock;
    }

    pub fn get(&self, x: i32, y: i32) -> Particle {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside(local_x, local_y) {
            return Particle::INVALID; // TODO: Change this to return a particle with id max usize value
        }

        self.particles[local_y][local_x]
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) -> () {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside(local_x, local_y) {
            return;
        }

        self.particles[local_y][local_x] = particle;
        self.particles[local_y][local_x].clock = !self.clock;
    }

    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_particle_id(&self, x: usize, y: usize) -> usize {
        self.particles[y][x].id
    }

    pub fn update(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_x = x;
                self.current_y = y;
                let current_particle = &self.particles[y][x];
                let particle_id = self.get_particle_id(x, y);
                if particle_id == 0 || current_particle.clock != self.clock {
                    continue;
                }

                (self.particle_definitions[particle_id].update_func)(self.particles[y][x], self);
            }
        }

        self.clock = !self.clock;
    }

    pub fn draw(&mut self) -> () {
        // Draw the particles by modifying the buffer
        for y in 0..self.height as u32 {
            for x in 0..self.width as u32 {
                let particle = &self.particles[y as usize][x as usize];
                let particle_definition = &self.particle_definitions[particle.id as usize];
                let color = particle_definition.color;
                self.image.set_pixel(x, y, color);
            }
        }

        self.texture.update(&self.image);

        // Draw the texture
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}
