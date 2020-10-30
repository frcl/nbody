use serde::{Serialize, Deserialize};
extern crate toml;

mod vec;
mod body;

use body::*;

use std::fs;


#[derive(Serialize, Deserialize)]
struct Config {
    max_time_step: f64,
    number_of_steps: usize,
    write_step: usize,
    dist_threshold: f64,
    bodies: Vec<Body>,
}


fn main() {
    let config_text = fs::read_to_string("config.toml")
        .expect("Could not read config file");

    let config: Config = toml::from_str(&config_text)
        .expect("Invalid config file");

    let mut bodies = config.bodies;

    let cm = center_of_mass(&bodies);
    let cmv = center_of_mass_velocity(&bodies);

    for b in &mut bodies {
        b.pos -= cm;
        b.vel -= cmv;
    }

    let max_dt = config.max_time_step;
    let n = config.number_of_steps;
    let ws = config.write_step;
    let dthr = config.dist_threshold;

    let mut new_bodies = bodies.clone();

    let mut t = 0.0;

    print_stats(t, &bodies);

    for i in 1..n {
        let mut min_times = Vec::new();
        for b in &mut new_bodies {
            for c in bodies.iter().filter(|&x| x != b) {
                if let Some(t) = b.estimate_timestep(c, dthr) {
                    min_times.push(t);
                }
            }
        }
        let dt = min_times.into_iter().fold(max_dt, f64::min);
        t += dt;

        for b in &mut new_bodies {
            b.step(dt, bodies.iter().filter(|&x| x != b).collect());
        }

        bodies = new_bodies.clone();
        if i%ws == 0 {
            print_stats(t, &bodies);
        }
    }
}


fn print_stats(i: f64, bodies: &Vec<Body>) {
    print!("{}", i);
    for b in bodies {
        print!(", {}", b.pos);
    }
    print!("\n");
}
