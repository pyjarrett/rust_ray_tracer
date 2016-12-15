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
    }
}
