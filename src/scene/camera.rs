#![allow(dead_code)]
use std::f32::consts::PI;
use std::convert::From;
use math::{Matrix4x4, Point, Ray};

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
struct BasicDimensions2<T> {
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


/// The mapping between the raster (film) and the image plane of the camera.
pub struct Film {
    size: BasicDimensions2<u16>,
    raster_to_screen: Matrix4x4,
    screen_to_raster: Matrix4x4,
}

impl Film {
    /// Converting from screen to raster:
    /// 1.) Offset the screen by half its dimensions so all X,Y coordinates become non-negative.
    /// 2.) Squash the screen so X, Y range from [0, 1].
    /// 3.) Scale to the raster range X in [0, raster_width], Y in [0, raster_height].
    pub fn new(width: u16, height: u16) -> Film {
        let size = BasicDimensions2::<u16>::new(width, height);
        let screen = Film::screen_space_from_aspect_ratio(size.aspect_ratio());

        let screen_to_raster = Matrix4x4::scale(size.width() as f32, size.height() as f32, 1.0) *
            Matrix4x4::scale(1.0 / screen.width(), 1.0 / screen.height(), 1.0) *
            Matrix4x4::translate(screen.width() / 2.0, screen.height() / 2.0, 0.0);

        Film {
            size: size,
            raster_to_screen: screen_to_raster.inverse().unwrap(),
            screen_to_raster: screen_to_raster,
        }
    }

    /// Gives the dimension of the image plane (screen space).
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - the ratio of width to height.
    fn screen_space_from_aspect_ratio(aspect_ratio: f32) -> BasicDimensions2<f32> {
        assert!(aspect_ratio > 0.0);
        let w: f32;
        let h: f32;

        if aspect_ratio > 1.0 {
            w = 2.0 * aspect_ratio;
            h = 2.0 * 1.0;
        } else {
            w = 2.0 * 1.0;
            h = 2.0 * aspect_ratio;
        }

        BasicDimensions2::<f32>::new(w, h)
    }

    pub fn raster_to_screen(&self) -> &Matrix4x4 {
        &self.raster_to_screen
    }

    pub fn screen_to_raster(&self) -> &Matrix4x4 {
        &self.screen_to_raster
    }
}

impl Dimensions2<u16> for Film {
    fn width(&self) -> u16 {
        self.size.width()
    }

    fn height(&self) -> u16 {
        self.size.height()
    }
}

/// Provides units for planar angles.
///
/// We often want to be explicit about the units we're dealing with, so this lets us be explicit.
pub enum PlanarAngle<T = f32> {
    Radians(T),
    Degrees(T),
}

pub trait Projection {
    fn screen_to_camera(&self) -> &Matrix4x4;
    fn camera_to_screen(&self) -> &Matrix4x4;
}

pub struct Perspective {
    camera_to_screen: Matrix4x4,
    screen_to_camera: Matrix4x4,
}

impl Perspective {
    /// Generates a perspective transform with appropriate ranges.
    ///
    /// # Arguments
    /// * `near` - distance to near plane
    /// * `far` - distance to far plane
    /// * `fov` - field of view, and its units.
    ///
    pub fn new(near: f32, far: f32, fov: PlanarAngle) -> Perspective {
        assert!(near > 0.0);
        assert!(far > near);

        let fov_radians = match fov {
            PlanarAngle::Degrees(value) => value.to_radians(),
            PlanarAngle::Radians(value) => value,
        };
        assert!(fov_radians > 0.0 && fov_radians < 2.0 * PI);

        let projection = Matrix4x4::perspective(near, far, fov_radians);
        Perspective {
            camera_to_screen: projection,
            screen_to_camera: projection.inverse().unwrap(),
        }
    }
}

impl Projection for Perspective {
    fn screen_to_camera(&self) -> &Matrix4x4 {
        &self.screen_to_camera
    }

    fn camera_to_screen(&self) -> &Matrix4x4 {
        &self.camera_to_screen
    }
}

/// A film and projection melded into a single functional component, providing ray casting from the
/// viewing to the scene.
///
pub struct Camera {
    raster_to_camera: Matrix4x4,
    camera_to_raster: Matrix4x4,
}

impl Camera {
    // TODO: #9 Add camera_to_world transform (be sure to normalize rays!)
    pub fn new(film: &Film, projection: &Projection) -> Camera {
        let raster_to_camera = projection.screen_to_camera() * film.raster_to_screen();
        let camera_to_raster = raster_to_camera.inverse().unwrap();

        Camera {
            raster_to_camera: raster_to_camera,
            camera_to_raster: camera_to_raster,
        }
    }

    pub fn raster_to_camera(&self) -> Matrix4x4 {
        self.raster_to_camera
    }

    pub fn camera_to_raster(&self) -> Matrix4x4 {
        self.camera_to_raster
    }

    /// Generates a ray for use in ray tracing.
    pub fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let origin = Point::new(0.0, 0.0, 0.0);
        let image_plane = self.raster_to_camera * Point::new(x as f32, y as f32, 0.0);
        let direction = image_plane - origin;

        let mut ray = Ray {
            origin: origin,
            direction: direction,
        };
        ray.normalize().unwrap();
        ray
    }
}
