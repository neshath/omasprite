use crate::advanced::ParticleEmitter;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub position: [f32; 2],
    pub age: f32,
}
#[derive(Default)]
pub struct System {
    pub particles: Vec<Particle>,
    accumulator: f32,
}
impl System {
    pub fn update(&mut self, emitter: &ParticleEmitter, dt: f32) {
        self.accumulator += emitter.rate.max(0.0) * dt;
        while self.accumulator >= 1.0 {
            self.accumulator -= 1.0;
            let seed = self.particles.len() as f32 * 0.618;
            self.particles.push(Particle {
                position: [seed.fract() * 16.0, 0.0],
                age: 0.0,
            });
        }
        for p in &mut self.particles {
            p.age += dt;
            p.position[0] += emitter.wind[0] * dt;
            p.position[1] += (1.0 + emitter.wind[1]) * dt;
        }
        self.particles.retain(|p| p.age < emitter.lifetime.max(0.0));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn particles_spawn_move_and_expire() {
        let e = ParticleEmitter {
            name: "snow".into(),
            rate: 10.0,
            lifetime: 1.0,
            color: [255; 4],
            wind: [1.0, 0.0],
        };
        let mut s = System::default();
        s.update(&e, 0.5);
        assert_eq!(s.particles.len(), 5);
        let x = s.particles[0].position[0];
        s.update(&e, 0.25);
        assert!(s.particles[0].position[0] > x);
        s.update(&e, 1.0);
        assert!(s.particles.iter().all(|p| p.age < 1.0));
    }
}
