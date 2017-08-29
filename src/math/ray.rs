use math::Point;
use math::Vector;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn normalize(&mut self) -> Result<f32, ()> {
        self.direction.normalize()
    }

    pub fn is_normalized(&self) -> bool {
        self.direction.is_normalized()
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Origin {} Direction {}", self.origin, self.direction)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_at() {}
}
