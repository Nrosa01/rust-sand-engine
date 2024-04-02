use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Particle {
    pub id: u32,
}

#[derive(Debug)]
pub struct ParticleDefinition {
    pub name: String,
    pub update_func: fn(&mut GameState, usize, usize) -> (),
    pub color: Color
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
}

impl GameState {
    pub fn new(width: usize, height: usize) -> GameState {
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Set the filter mode to nearest to avoid blurring the pixels

        GameState {
            particles: vec![vec![Particle { id: 0 }; width]; height],
            current_x: 0,
            current_y: 0,
            width,
            height,
            particle_definitions: vec![ParticleDefinition {
                name: String::from("empty"),
                update_func: |_, _, _| {}, // Función vacía
                color: BLACK,
            }],
            image: image,
            texture: texture,
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

    pub fn set_particle(&mut self, x: usize, y: usize, id: u32) -> () {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return;
        }
        self.particles[y][x] = Particle { id: id };
    }

    pub fn get_particle_id(&self, x: usize, y: usize) -> u32 {
        self.particles[y][x].id
    }

    pub fn update(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_x = x;
                self.current_y = y;
                let particle_id = self.get_particle_id(x, y);
                if particle_id == 0 {
                    continue;
                }

                (self.particle_definitions[particle_id as usize].update_func)(self, x, y);
            }
        }
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
