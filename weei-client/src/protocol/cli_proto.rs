use std::fmt;

pub struct P2pMessage {
    pub x: u32,
    pub y: u32,
}

pub fn new_b(a: u32, b: u32) -> P2pMessage {
    P2pMessage { x: a, y: b }
}

impl fmt::Display for P2pMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "this is:{},{}", self.x, self.y)
    }
}
