use math::Point;
use math::Vector;

#[derive(Clone,Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn normalize(&mut self) -> Result<f32, ()> {
        self.direction.normalize()
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_at() {}
}
