struct Film {
    width: i32,
    height: i32,
}


impl Film {
    pub fn new(width: i32, height: i32) -> Film {
        Film {
            width: width,
            height: height,
        }
    }

    pub fn to_ndc(&self, x: i32, y: i32) -> (f32, f32) {
        (x as f32 / self.width as f32, (self.height - y) as f32 / self.height as f32)
    }

    pub fn from_ndc(&self, x: f32, y: f32) -> (i32, i32) {
        (((x * (self.width as f32)).round() as i32), (((1.0 - y) * self.height as f32) as i32))
    }
}


#[cfg(test)]
mod test {
    use super::Film;
    use std::ops::Sub;
    use std::cmp::PartialOrd;

    fn assert_close<T>(lhs: (T, T), rhs: (T, T))
        where T: Sub<T> + Copy,
              <T as Sub<T>>::Output: PartialOrd<f32>
    {
        let (x1, y1): (T, T) = lhs;
        let (x2, y2): (T, T) = rhs;
        assert!((x2 - x1) < 1e-6);
        assert!((y2 - y1) < 1e-6);
        assert!((x2 - x1) > -1e-6);
        assert!((y2 - y1) > -1e-6);
    }

    #[test]
    fn test_film_to_NDC() {
        let f = Film::new(800, 600);
        assert_close(f.to_ndc(0, 0), (0.0, 1.0));
        assert_close(f.to_ndc(800, 0), (1.0, 1.0));
        assert_close(f.to_ndc(800, 600), (1.0, 0.0));
        assert_close(f.to_ndc(0, 600), (0.0, 0.0));
        assert_close(f.to_ndc(400, 300), (0.5, 0.5));
    }

    #[test]
    fn test_ndc_to_film() {
        let f = Film::new(800, 600);
        // Try back converting our previous points.
        assert_eq!(f.from_ndc(0.0, 1.0), (0, 0));
        assert_eq!(f.from_ndc(1.0, 1.0), (800, 0));
        assert_eq!(f.from_ndc(1.0, 0.0), (800, 600));
        assert_eq!(f.from_ndc(0.0, 0.0), (0, 600));
        assert_eq!(f.from_ndc(0.5, 0.5), (400, 300));
    }
}
