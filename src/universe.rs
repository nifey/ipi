use crate::packet::Packet;
use wasm_bindgen::prelude::*;

const MIN_STAR_RADIUS: u32 = 8;
const MAX_STAR_RADIUS: u32 = 12;
const MIN_PLANET_RADIUS: u32 = 3;
const MAX_PLANET_RADIUS: u32 = 8;
const MIN_STAR_SYSTEM_RADIUS: u32 = 150;
const MAX_STAR_SYSTEM_RADIUS: u32 = 300;
const MIN_NUM_PLANETS: u32 = 1;
const MAX_NUM_PLANETS: u32 = 6;
const MIN_NUM_STARS: u32 = 3;
const MAX_NUM_STARS: u32 = 5;
const MIN_PLANET_DQ: u32 = 1;
pub const MAX_PLANET_DQ: u32 = 8;
const PLANET_ACTIVATE_RANGE: u32 = 3;
const MAX_TRIES: u32 = 10;
pub const SLOWDOWN_FACTOR: f64 = 0.2;
const PACKET_SPEED: f64 = 13.0;
const PACKET_RADIUS: u32 = 7;
pub const MIN_PACKET_DQ: f64 = 1.0;

#[wasm_bindgen(module = "/util.js")]
extern "C" {
    fn gen_rand(start: u32, end: u32) -> u32;
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    score: u32,
    packet: Packet,
    packet_source: usize,
    packet_destination: usize,
    packet_reached_destination: bool,
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
            score: 0,
            packet: Packet::new(),
            packet_source: 0,
            packet_destination: 0,
            packet_reached_destination: false,
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
        self.generate_packet();
    }

    fn reset(&mut self) {
        self.packet = Packet::new();
        self.packet_source = 0;
        self.packet_destination = 0;
        self.packet_reached_destination = false;
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

    fn num_planets(&self) -> usize {
        self.planet_star.len()
    }

    fn generate_stars(&mut self) -> bool {
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

    fn generate_planet_positions(&mut self) -> bool {
        for star in 0..self.star_x.len() {
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

    fn generate_planet_angles(&mut self) {
        for _ in 0..self.num_planets() {
            self.planet_q.push(gen_rand(0, 359) as f64);
            self.planet_dq
                .push(gen_rand(MIN_PLANET_DQ, MAX_PLANET_DQ) as f64 * SLOWDOWN_FACTOR);
            self.planet_direction.push(gen_rand(0, 100) % 2 == 1);
        }
    }

    fn generate_packet(&mut self) {
        let mut source = 0;
        let mut destination = 0;
        if self.width > self.height {
            let mut min = self.planet_x(0);
            let mut max = self.planet_x(0);
            for planet in 0..self.num_planets() {
                let x = self.planet_x(planet);
                if x < min {
                    min = x;
                    source = planet;
                } else if x > max {
                    max = x;
                    destination = planet;
                }
            }
        } else {
            let mut min = self.planet_y(0);
            let mut max = self.planet_y(0);
            for planet in 0..self.num_planets() {
                let y = self.planet_y(planet);
                if y < min {
                    min = y;
                    source = planet;
                } else if y > max {
                    max = y;
                    destination = planet;
                }
            }
        }
        self.packet_source = source;
        self.packet_destination = destination;
        self.packet = Packet::set_bound(
            source,
            self.planet_q[source],
            self.planet_dq[source],
            self.planet_direction[source],
        );
    }

    fn planet_x(&self, planet: usize) -> f64 {
        self.star_x[self.planet_star[planet] as usize] as f64
            + (self.planet_distance[planet] as f64 * self.planet_q[planet].to_radians().cos())
    }

    fn planet_y(&self, planet: usize) -> f64 {
        self.star_y[self.planet_star[planet] as usize] as f64
            + (self.planet_distance[planet] as f64 * self.planet_q[planet].to_radians().sin())
    }

    fn packet_bound(&self) -> bool {
        self.packet.is_bound()
    }

    pub fn packet_end_x(&self) -> f64 {
        match self.packet {
            Packet::Bound {
                planet,
                q,
                dq: _,
                direction: _,
            } => {
                let px = self.planet_x(planet);
                let py = self.planet_y(planet);
                if q == 0.0 {
                    self.width as f64
                } else if q == 90.0 {
                    px
                } else if q == 180.0 {
                    0.0
                } else if q == 270.0 {
                    px
                } else if q > 0.0 && q < 90.0 {
                    // First quadrant
                    let new_x = py * (90.0 - q).to_radians().tan();
                    if new_x + px > self.width as f64 {
                        self.width as f64
                    } else {
                        px + new_x
                    }
                } else if q > 90.0 && q < 180.0 {
                    // Second quadrant
                    let new_x = py * (q - 90.0).to_radians().tan();
                    if new_x <= px {
                        px - new_x
                    } else {
                        0.0
                    }
                } else if q > 180.0 && q < 270.0 {
                    // Third quadrant
                    let new_x = (self.height as f64 - py) * (270.0 - q).to_radians().tan();
                    if new_x <= px {
                        px - new_x
                    } else {
                        0.0
                    }
                } else {
                    // Fourth quadrant
                    let new_x = (self.height as f64 - py) * (q - 270.0).to_radians().tan();
                    if new_x + px > self.width as f64 {
                        self.width as f64
                    } else {
                        px + new_x
                    }
                }
            }
            Packet::Free {
                x: _,
                y: _,
                dx: _,
                dy: _,
                last_planet: _,
            } => 0.0,
        }
    }

    pub fn packet_end_y(&self) -> f64 {
        match self.packet {
            Packet::Bound {
                planet,
                q,
                dq: _,
                direction: _,
            } => {
                let px = self.planet_x(planet);
                let py = self.planet_y(planet);
                if q == 0.0 {
                    py
                } else if q == 90.0 {
                    0.0
                } else if q == 180.0 {
                    py
                } else if q == 270.0 {
                    self.height as f64
                } else if q > 0.0 && q < 90.0 {
                    // First quadrant
                    let new_y = (self.width as f64 - px) * q.to_radians().tan();
                    if new_y <= py {
                        py - new_y
                    } else {
                        0.0
                    }
                } else if q > 90.0 && q < 180.0 {
                    // Second quadrant
                    let new_y = px * (180.0 - q).to_radians().tan();
                    if new_y <= py {
                        py - new_y
                    } else {
                        0.0
                    }
                } else if q > 180.0 && q < 270.0 {
                    // Third quadrant
                    let new_y = px * (q - 180.0).to_radians().tan();
                    if new_y + py > self.height as f64 {
                        self.height as f64
                    } else {
                        py + new_y
                    }
                } else {
                    // Fourth quadrant
                    let new_y = (self.width as f64 - px) * (360.0 - q).to_radians().tan();
                    if new_y + py > self.height as f64 {
                        self.height as f64
                    } else {
                        py + new_y
                    }
                }
            }
            Packet::Free {
                x: _,
                y: _,
                dx: _,
                dy: _,
                last_planet: _,
            } => 0.0,
        }
    }

    pub fn packet_x(&self) -> f64 {
        match self.packet {
            Packet::Bound {
                planet: _,
                q: _,
                dq: _,
                direction: _,
            } => 0.0,
            Packet::Free {
                x,
                y: _,
                dx: _,
                dy: _,
                last_planet: _,
            } => x,
        }
    }

    pub fn packet_y(&self) -> f64 {
        match self.packet {
            Packet::Bound {
                planet: _,
                q: _,
                dq: _,
                direction: _,
            } => 0.0,
            Packet::Free {
                x: _,
                y,
                dx: _,
                dy: _,
                last_planet: _,
            } => y,
        }
    }

    fn within_window(&self, x: u32, y: u32, radius: u32) -> bool {
        !((x as i32 - radius as i32) < 0
            || (x as i32 + radius as i32) > self.width as i32
            || (y as i32 - radius as i32) < 0
            || (y as i32 + radius as i32) > self.height as i32)
    }

    pub fn free_packet(&mut self) {
        match self.packet {
            Packet::Bound {
                planet,
                q,
                dq: _,
                direction: _,
            } => {
                self.packet = Packet::set_free(
                    self.planet_x(planet),
                    self.planet_y(planet),
                    PACKET_SPEED * q.to_radians().cos(),
                    -PACKET_SPEED * q.to_radians().sin(),
                    planet,
                );
            }
            Packet::Free {
                x: _,
                y: _,
                dx: _,
                dy: _,
                last_planet: _,
            } => {}
        }
    }

    pub fn tick(&mut self) -> *const u32 {
        let mut planet_data: Vec<u32> = Vec::new();
        for planet in 0..self.num_planets() {
            if self.planet_direction[planet] {
                self.planet_q[planet] = self.planet_q[planet] + self.planet_dq[planet];
            } else {
                self.planet_q[planet] = self.planet_q[planet] + (360.0 - self.planet_dq[planet]);
            }
            if self.planet_q[planet] > 360.0 {
                self.planet_q[planet] -= 360.0;
            }
            planet_data.push(self.planet_x(planet) as u32);
            planet_data.push(self.planet_y(planet) as u32);
            planet_data.push(self.planet_radius[planet] as u32);
        }
        let (within_window, packet) = Packet::tick(self.packet, self.width, self.height);
        if !within_window {
            self.packet_reached_destination = false;
            self.packet = Packet::set_bound(
                self.packet_source,
                self.planet_q[self.packet_source],
                self.planet_dq[self.packet_source],
                self.planet_direction[self.packet_source],
            );
        } else {
            self.packet = packet;
        }
        if !self.packet_bound() {
            let px = self.packet_x();
            let py = self.packet_y();
            for planet in 0..self.num_planets() {
                if planet != self.packet.get_last_planet()
                    && (px - self.planet_x(planet)).powf(2.0)
                        + (py - self.planet_y(planet)).powf(2.0)
                        < (self.planet_radius[planet] as f64 * PLANET_ACTIVATE_RANGE as f64
                            + PACKET_RADIUS as f64)
                            .powf(2.0)
                {
                    self.packet = Packet::set_bound(
                        planet,
                        self.planet_q[planet],
                        self.planet_dq[planet],
                        self.planet_direction[planet],
                    );
                    if !self.packet_reached_destination && planet == self.packet_destination {
                        self.packet_reached_destination = true;
                    }
                    if self.packet_reached_destination && planet == self.packet_source {
                        self.score += 1;
                        self.reset();
                        self.generate();
                    }
                    break;
                }
            }
        }
        let mut data: Vec<u32> = Vec::new();
        data.push(0);
        data.push(self.score);
        // Push packet or active planet data
        match self.packet {
            Packet::Bound {
                planet,
                q: _,
                dq: _,
                direction: _,
            } => {
                data.push(self.planet_x(planet) as u32);
                data.push(self.planet_y(planet) as u32);
                data.push(self.packet_end_x() as u32);
                data.push(self.packet_end_y() as u32);
                data.push(1);
                data.push(self.planet_x(planet) as u32);
                data.push(self.planet_y(planet) as u32);
                data.push((self.planet_radius[planet] as f32 * 1.5) as u32);
            }
            Packet::Free {
                x,
                y,
                dx: _,
                dy: _,
                last_planet: _,
            } => {
                data.push(0);
                data.push(1);
                data.push(x as u32);
                data.push(y as u32);
                data.push(PACKET_RADIUS);
            }
        }
        // Push source and destination planet
        data.push(2);
        data.push(self.planet_x(self.packet_destination) as u32);
        data.push(self.planet_y(self.packet_destination) as u32);
        data.push((self.planet_radius[self.packet_destination] as f32 * 1.8) as u32);
        data.push(self.planet_x(self.packet_source) as u32);
        data.push(self.planet_y(self.packet_source) as u32);
        data.push((self.planet_radius[self.packet_source] as f32 * 1.8) as u32);
        // Push star data
        data.push(self.star_x.len() as u32);
        for i in 0..self.star_x.len() {
            data.push(self.star_x[i]);
            data.push(self.star_y[i]);
            data.push(self.star_radius[i]);
        }
        // Push planet data
        data.push(self.planet_q.len() as u32);
        data.append(&mut planet_data);
        data.as_ptr()
    }
}

fn circles_overlap(x1: u32, y1: u32, radius1: u32, x2: u32, y2: u32, radius2: u32) -> bool {
    (x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)
        < (radius1 + radius2).pow(2) as i32
}
