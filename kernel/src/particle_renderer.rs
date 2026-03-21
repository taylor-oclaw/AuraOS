extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    unsafe { particle_renderer_init(); }
}

#[repr(C)]
pub struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    color: [u8; 4],
    size: f32,
    lifetime: f32,
}

impl Particle {
    pub fn new(position: [f32; 3], velocity: [f32; 3], color: [u8; 4], size: f32, lifetime: f32) -> Self {
        Particle {
            position,
            velocity,
            color,
            size,
            lifetime,
        })
    }

    pub fn update(&mut self, delta_time: f32) {
        for i in 0..3 {
            self.position[i] += self.velocity[i] * delta_time;
        }
        self.lifetime -= delta_time;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    pub fn get_color(&self) -> [u8; 4] {
        self.color
    }
}

#[repr(C)]
pub struct ParticleRenderer {
    particles: Vec<Particle>,
}

impl ParticleRenderer {
    pub fn new() -> Self {
        ParticleRenderer {
            particles: Vec::new(),
        })
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }
}

pub extern "C" fn particle_renderer_init() {
    unsafe {
        let mut renderer = ParticleRenderer::new();
        renderer.add_particle(Particle::new(
            [0.0, 0.0, 0.0],
            [1.0, 2.0, 3.0],
            [255, 0, 0, 255],
            1.0,
            10.0,
        ;
        renderer.update_particles(0.1);
    }
}
