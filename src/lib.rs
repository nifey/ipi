use wasm_bindgen::prelude::*;

const MIN_STAR_RADIUS: u32 = 20;
const MAX_STAR_RADIUS: u32 = 40;
const MIN_STAR_SYSTEM_RADIUS: u32 = 100;
const MAX_STAR_SYSTEM_RADIUS: u32 = 400;

#[wasm_bindgen(module = "/util.js")]
extern "C" {
    fn gen_rand(start: u32, end: u32) -> u32;
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    num_stars: u32,
    star_x: Vec<u32>,
    star_y: Vec<u32>,
    star_system_radius: Vec<u32>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32, num_stars: u32) -> Universe {
        let mut universe = Universe {
            width,
            height,
            num_stars,
            star_x: Vec::new(),
            star_y: Vec::new(),
            star_system_radius: Vec::new(),
        };
        universe.generate_stars(width, height, num_stars);
        universe
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn num_stars(&self) -> u32 {
        self.num_stars
    }

    pub fn generate_stars(&mut self, width: u32, height: u32, num_stars: u32) {
        let mut star_x: Vec<u32> = Vec::with_capacity(num_stars as usize);
        let mut star_y: Vec<u32> = Vec::with_capacity(num_stars as usize);
        let mut star_system_radius: Vec<u32> = Vec::with_capacity(num_stars as usize);
        for _ in 0..num_stars {
            loop {
                let mut overlap = false;
                let x: u32 = gen_rand(0, width);
                let y: u32 = gen_rand(0, height);
                let sys_radius: u32 = gen_rand(MIN_STAR_SYSTEM_RADIUS, MAX_STAR_SYSTEM_RADIUS);
                for star in 0..star_x.len() {
                    if circles_overlap(
                        x,
                        y,
                        sys_radius,
                        star_x[star],
                        star_y[star],
                        star_system_radius[star],
                    ) {
                        overlap = true;
                        break;
                    }
                }
                if !overlap && self.within_window(x, y, sys_radius) {
                    star_x.push(x);
                    star_y.push(y);
                    star_system_radius.push(sys_radius);
                    break;
                }
            }
        }
        self.star_x = star_x;
        self.star_y = star_y;
        self.star_system_radius = star_system_radius;
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

    pub fn within_window(&self, x: u32, y: u32, radius: u32) -> bool {
        !((x as i32 - radius as i32) < 0
            || (x as i32 + radius as i32) > self.width as i32
            || (y as i32 - radius as i32) < 0
            || (y as i32 + radius as i32) > self.height as i32)
    }
}

pub fn circles_overlap(x1: u32, y1: u32, radius1: u32, x2: u32, y2: u32, radius2: u32) -> bool {
    (x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)
        < (radius1 + radius2).pow(2) as i32
}
