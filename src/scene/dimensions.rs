/// A object with width and height.  Typically these are rectangular though no requirement exists
/// for this.
pub trait Dimensions2<T>
where
    f64: From<T>,
{
    fn width(&self) -> T;
    fn height(&self) -> T;

    /// The ratio of width to height.
    fn aspect_ratio(&self) -> f32 {
        ((f64::from(self.width())) / f64::from(self.height())) as f32
    }
}

#[derive(Clone, Copy)]
/// An immutable rectangle.
pub struct BasicDimensions2<T> {
    width: T,
    height: T,
}


impl<T> BasicDimensions2<T>
where
    f64: From<T>,
    T: Copy,
{
    pub fn new(width: T, height: T) -> BasicDimensions2<T> {
        BasicDimensions2 {
            width: width,
            height: height,
        }
    }
}

impl<T> Dimensions2<T> for BasicDimensions2<T>
where
    f64: From<T>,
    T: Copy,
{
    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}
