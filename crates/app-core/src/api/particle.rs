use macroquad::rand;

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub id: u8,
    pub clock: bool,
    pub light: u8,
    pub extra: u8,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            id: 0,
            clock: false,
            light: rand::gen_range(200, 256) as u8,
            extra: 0,
        }
    }

    pub fn from_id(id: u8) -> Particle {
        //print something
        Particle {
            id,
            clock: false,
            light: rand::gen_range(200, 256) as u8,
            extra: 0,
        }
    }

    pub const EMPTY: Particle = Particle {
        id: 0,
        clock: false,
        light: 0,
        extra: 0,
    };

    pub(crate) const INVALID: Particle = Particle {
        id: u8::MAX,
        clock: false,
        light: 0,
        extra: 0,
    };
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<usize> for Particle {
    fn eq(&self, other: &usize) -> bool {
        self.id == *other as u8
    }
}

impl PartialEq<u8> for Particle {
    fn eq(&self, other: &u8) -> bool {
        self.id == *other
    }
}

impl From<usize> for Particle {
    fn from(id: usize) -> Self {
        Particle::from_id(id as u8)
    }
}

impl From<u8> for Particle {
    fn from(id: u8) -> Self {
        Particle::from_id(id as u8)
    }
}