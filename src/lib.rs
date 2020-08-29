use wasm_bindgen::prelude::*;

const MIN_STAR_RADIUS: u32 = 8;
const MAX_STAR_RADIUS: u32 = 12;
const MIN_PLANET_RADIUS: u32 = 3;
const MAX_PLANET_RADIUS: u32 = 7;
const MIN_STAR_SYSTEM_RADIUS: u32 = 150;
const MAX_STAR_SYSTEM_RADIUS: u32 = 300;
const MIN_NUM_PLANETS: u32 = 1;
const MAX_NUM_PLANETS: u32 = 4;
const MIN_NUM_STARS: u32 = 3;
const MAX_NUM_STARS: u32 = 5;
const MIN_PLANET_DQ: u32 = 1;
const MAX_PLANET_DQ: u32 = 3;
const PLANET_ACTIVATE_RANGE: u32 = 2;
const MAX_TRIES: u32 = 10;
const SLOWDOWN_FACTOR: f64 = 0.5;

#[wasm_bindgen(module = "/util.js")]
extern "C" {
    fn gen_rand(start: u32, end: u32) -> u32;
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    star_x: Vec<u32>,
    star_y: Vec<u32>,
    star_radius: Vec<u32>,
    star_system_radius: Vec<u32>,
    planet_radius: Vec<u32>,
    planet_star: Vec<u8>,
    planet_distance: Vec<u32>,
    planet_q: Vec<f64>,
    planet_dq: Vec<f64>,
    planet_direction: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let mut universe = Universe {
            width,
            height,
            star_x: Vec::new(),
            star_y: Vec::new(),
            star_radius: Vec::new(),
            star_system_radius: Vec::new(),
            planet_radius: Vec::new(),
            planet_star: Vec::new(),
            planet_distance: Vec::new(),
            planet_q: Vec::new(),
            planet_dq: Vec::new(),
            planet_direction: Vec::new(),
        };
        universe.generate();
        universe
    }

    fn generate(&mut self) {
        loop {
            if !self.generate_stars() || !self.generate_planet_positions() {
                self.reset();
            } else {
                break;
            }
        }
        self.generate_planet_angles();
    }

    fn reset(&mut self) {
        self.star_x.clear();
        self.star_y.clear();
        self.star_radius.clear();
        self.star_system_radius.clear();
        self.planet_radius.clear();
        self.planet_star.clear();
        self.planet_distance.clear();
        self.planet_q.clear();
        self.planet_dq.clear();
        self.planet_direction.clear();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn num_stars(&self) -> usize {
        self.star_x.len()
    }

    pub fn num_planets(&self) -> usize {
        self.planet_star.len()
    }

    pub fn generate_stars(&mut self) -> bool {
        let num_stars = gen_rand(MIN_NUM_STARS, MAX_NUM_STARS);
        let mut outer_tries = 0;
        loop {
            outer_tries += 1;
            let mut generation_done = true;
            for _ in 0..num_stars {
                let mut tries = 0;
                loop {
                    tries += 1;
                    let mut overlap = false;
                    let x: u32 = gen_rand(0, self.width);
                    let y: u32 = gen_rand(0, self.height);
                    let sys_radius: u32 = gen_rand(MIN_STAR_SYSTEM_RADIUS, MAX_STAR_SYSTEM_RADIUS);
                    for star in 0..self.star_x.len() {
                        if circles_overlap(
                            x,
                            y,
                            sys_radius,
                            self.star_x[star],
                            self.star_y[star],
                            self.star_system_radius[star],
                        ) {
                            overlap = true;
                            break;
                        }
                    }
                    if !overlap && self.within_window(x, y, sys_radius) {
                        self.star_x.push(x);
                        self.star_y.push(y);
                        self.star_radius
                            .push(gen_rand(MIN_STAR_RADIUS, MAX_STAR_RADIUS));
                        self.star_system_radius.push(sys_radius);
                        break;
                    } else if tries == MAX_TRIES {
                        generation_done = false;
                        break;
                    }
                }
                if !generation_done {
                    break;
                }
            }
            if generation_done {
                break;
            }
            if outer_tries == MAX_TRIES {
                return false;
            }
            self.star_x.clear();
            self.star_y.clear();
            self.star_radius.clear();
            self.star_system_radius.clear();
        }
        true
    }

    pub fn generate_planet_positions(&mut self) -> bool {
        for star in 0..self.num_stars() {
            let mut generation_done;
            let system_index = self.planet_star.len();
            let mut outer_tries = 0;
            loop {
                outer_tries += 1;
                generation_done = true;
                let num_planets_in_star = gen_rand(MIN_NUM_PLANETS, MAX_NUM_PLANETS);
                for _ in 0..num_planets_in_star {
                    let mut tries = 0;
                    loop {
                        tries += 1;
                        let mut overlap = false;
                        let radius: u32 = gen_rand(MIN_PLANET_RADIUS, MAX_PLANET_RADIUS);
                        let distance: u32 = gen_rand(
                            self.star_radius[star] + PLANET_ACTIVATE_RANGE * radius,
                            self.star_system_radius[star] - PLANET_ACTIVATE_RANGE * radius,
                        );
                        for planet in system_index..self.planet_star.len() {
                            if ((distance as i32 - self.planet_distance[planet] as i32).abs())
                                < ((PLANET_ACTIVATE_RANGE * (radius + self.planet_radius[planet]))
                                    as i32)
                            {
                                overlap = true;
                                break;
                            }
                        }
                        if !overlap {
                            self.planet_star.push(star as u8);
                            self.planet_radius.push(radius);
                            self.planet_distance.push(distance);
                            break;
                        } else if tries == MAX_TRIES {
                            generation_done = false;
                            break;
                        }
                    }
                    if !generation_done {
                        break;
                    }
                }
                if generation_done {
                    break;
                }
                if outer_tries == MAX_TRIES {
                    return false;
                }
                self.planet_star.truncate(system_index);
                self.planet_radius.truncate(system_index);
                self.planet_distance.truncate(system_index);
            }
        }
        true
    }

    pub fn generate_planet_angles(&mut self) {
        for _ in 0..self.num_planets() {
            self.planet_q.push(gen_rand(0, 359) as f64);
            self.planet_dq
                .push(gen_rand(MIN_PLANET_DQ, MAX_PLANET_DQ) as f64 * SLOWDOWN_FACTOR);
            self.planet_direction.push(gen_rand(0, 100) % 2 == 1);
        }
    }

    pub fn star_x(&self, star: usize) -> u32 {
        if self.star_x.len() < star {
            0
        } else {
            self.star_x[star]
        }
    }

    pub fn star_y(&self, star: usize) -> u32 {
        if self.star_y.len() < star {
            0
        } else {
            self.star_y[star]
        }
    }

    pub fn star_system_radius(&self, star: usize) -> u32 {
        if self.star_system_radius.len() < star {
            0
        } else {
            self.star_system_radius[star]
        }
    }

    pub fn star_radius(&self, star: usize) -> u32 {
        if self.star_radius.len() < star {
            0
        } else {
            self.star_radius[star]
        }
    }

    pub fn planet_x(&self, planet: usize) -> f64 {
        self.star_x[self.planet_star[planet] as usize] as f64
            + (self.planet_distance[planet] as f64 * self.planet_q[planet].to_radians().cos())
    }

    pub fn planet_y(&self, planet: usize) -> f64 {
        self.star_y[self.planet_star[planet] as usize] as f64
            + (self.planet_distance[planet] as f64 * self.planet_q[planet].to_radians().sin())
    }

    pub fn planet_radius(&self, planet: usize) -> u32 {
        self.planet_radius[planet]
    }

    pub fn within_window(&self, x: u32, y: u32, radius: u32) -> bool {
        !((x as i32 - radius as i32) < 0
            || (x as i32 + radius as i32) > self.width as i32
            || (y as i32 - radius as i32) < 0
            || (y as i32 + radius as i32) > self.height as i32)
    }

    pub fn tick(&mut self) -> bool {
        for planet in 0..self.num_planets() {
            if self.planet_direction[planet] {
                self.planet_q[planet] = self.planet_q[planet] + self.planet_dq[planet];
            } else {
                self.planet_q[planet] = self.planet_q[planet] + (360.0 - self.planet_dq[planet]);
            }
            if self.planet_q[planet] > 360.0 {
                self.planet_q[planet] -= 360.0;
            }
        }
        true
    }
}

pub fn circles_overlap(x1: u32, y1: u32, radius1: u32, x2: u32, y2: u32, radius2: u32) -> bool {
    (x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)
        < (radius1 + radius2).pow(2) as i32
}
