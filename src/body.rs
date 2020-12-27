use serde::{Serialize, Deserialize};
use crate::vec::*;


const G: f64 = 0.01;
const MIN_VEL: f64 = f64::MIN_POSITIVE*16777216f64;


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Body {
    mass: f64,
    pub pos: Vec2,
    pub vel: Vec2,
}


impl Body {
    #[allow(dead_code)]
    pub fn new(mass: f64, pos: Vec2, vel: Vec2) -> Body {
        Body { mass, pos, vel }
    }

    pub fn estimate_timestep(&self, other: &Body, dist_threshold: f64) -> Option<f64> {
        if self.vel.norm() <= MIN_VEL {
            return None;
        }
        let diff = other.pos-self.pos;
        Some(diff.norm()*dist_threshold/self.vel.norm())
    }

    pub fn step(&mut self, dt: f64, others: Vec<&Body>) {
        self.pos += dt/2.0*self.vel;

        // F = m a
        // F = - G m M r / |r|^3
        // => Δv = - Σ_i Δt G M_i (r - r_i)/|r-r_i|^3
        // => Δr = Δt v
        let s: Vec2 = others.iter().map(|b| {
            let diff = self.pos-b.pos;
            diff*(b.mass/diff.norm().powi(3))
        }).sum();

        self.vel -= dt*G*s;
        self.pos += dt/2.0*self.vel;
    }

    pub fn energy(&self, others: Vec<&Body>) -> f64 {
        0.5*self.mass*self.vel.norm_sq()
            - G*self.mass*others.iter().map(|b| b.mass/(self.pos-b.pos).norm()).fold(0.0, |a,b| a+b)
    }
}


pub fn center_of_mass(bodys: &Vec<Body>) -> Vec2 {
    let m: f64 = bodys.iter().map(|b| b.mass).sum();
    let mr: Vec2 = bodys.iter().map(|b| b.mass*b.pos).sum();
    1.0/m*mr
}


pub fn center_of_mass_velocity(bodys: &Vec<Body>) -> Vec2 {
    let m: f64 = bodys.iter().map(|b| b.mass).sum();
    let mr: Vec2 = bodys.iter().map(|b| b.mass*b.vel).sum();
    1.0/m*mr
}
