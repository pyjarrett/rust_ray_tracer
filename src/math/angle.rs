/// Provides units for planar angles.
///
/// We often want to be explicit about the units we're dealing with, so this lets us be explicit.
pub enum PlanarAngle<T = f32> {
    Radians(T),
    Degrees(T),
}

impl PlanarAngle {
    pub fn to_degrees(&self) -> f32 {
        match *self {
            PlanarAngle::Radians(value) => value.to_degrees(),
            PlanarAngle::Degrees(value) => value,
        }
    }

    pub fn to_radians(&self) -> f32 {
        match *self {
            PlanarAngle::Radians(value) => value,
            PlanarAngle::Degrees(value) => value.to_radians(),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::PlanarAngle;

    #[test]
    fn test_to_and_from_degrees() {
        let p = PlanarAngle::Radians(PI * 1.0);
        let d = p.to_degrees();
        assert_relative_eq!(PlanarAngle::Degrees(d).to_radians(), p.to_radians());
    }

    #[test]
    fn test_to_and_from_radians() {
        let p = PlanarAngle::Degrees(180.0_f32);
        let r = p.to_radians();
        assert_relative_eq!(PlanarAngle::Radians(r).to_degrees(), p.to_degrees());
    }
}
