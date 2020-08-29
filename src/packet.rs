use crate::universe::{MAX_PLANET_DQ, MIN_PACKET_DQ, SLOWDOWN_FACTOR};

#[derive(Clone, Copy)]
pub enum Packet {
    Bound {
        planet: usize,
        q: f64,
        dq: f64,
        direction: bool,
    },
    Free {
        x: f64,
        y: f64,
        dx: f64,
        dy: f64,
    },
}

impl Packet {
    pub fn new() -> Packet {
        Packet::Free {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn set_bound(planet: usize, q: f64, planet_dq: f64, direction: bool) -> Packet {
        Packet::Bound {
            planet,
            q,
            dq: (MAX_PLANET_DQ as f64 * SLOWDOWN_FACTOR - planet_dq + MIN_PACKET_DQ),
            direction,
        }
    }

    pub fn set_free(x: f64, y: f64, dx: f64, dy: f64) -> Packet {
        Packet::Free { x, y, dx, dy }
    }

    pub fn tick(packet: Packet, width: u32, height: u32) -> (bool, Packet) {
        match packet {
            Packet::Bound {
                planet,
                q,
                dq,
                direction,
            } => {
                let mut new_q;
                if direction {
                    new_q = q + dq;
                } else {
                    new_q = q + (360.0 - dq);
                }
                if new_q > 360.0 {
                    new_q -= 360.0;
                }
                (
                    true,
                    Packet::Bound {
                        planet,
                        q: new_q,
                        dq,
                        direction,
                    },
                )
            }
            Packet::Free { x, y, dx, dy } => {
                let new_x = x as f64 + dx;
                let new_y = y as f64 + dy;
                if new_x < 0.0 || new_x > width as f64 || new_y < 0.0 || new_y > height as f64 {
                    return (false, packet);
                }
                (
                    true,
                    Packet::Free {
                        x: new_x,
                        y: new_y,
                        dx,
                        dy,
                    },
                )
            }
        }
    }

    pub fn is_bound(&self) -> bool {
        match &self {
            Packet::Bound {
                planet: _,
                q: _,
                dq: _,
                direction: _,
            } => true,
            Packet::Free {
                x: _,
                y: _,
                dx: _,
                dy: _,
            } => false,
        }
    }
}
