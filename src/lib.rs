// How can I integrate with openframeworks datatypes?
// How do I provide an opaque pointer in the point object?

pub mod ffi;
pub mod rectangle;
pub mod subdivision;

use ffi::UserData;
use rectangle::Rectangle;
use subdivision::QuadtreeSubdivisions;
#[repr(C)]
pub struct Quadtree {
    pub points: Vec<Point>,
    pub boundary: Rectangle,
    pub children: Option<QuadtreeSubdivisions>,
    pub capacity: u8,
}

impl Quadtree {
    pub fn new(boundary: &Rectangle) -> Quadtree {
        Quadtree {
            points: Vec::new(),
            boundary: *boundary,
            children: None,
            capacity: 10u8,
        }
    }

    pub fn insert_point(&mut self, point: &Point) -> Result<(), ()> {
        if !self.boundary.contains(point) {
            return Err(());
        }

        if self.points.len() < self.capacity.into() {
            self.points.push(*point);
            Ok(())
        } else {
            let mut sd = QuadtreeSubdivisions::new(&self.boundary);
            let result = sd
                .nw
                .insert_point(point)
                .or_else(|_| sd.ne.insert_point(point))
                .or_else(|_| sd.sw.insert_point(point))
                .or_else(|_| sd.se.insert_point(point));

            self.children = Some(sd);

            result
        }
    }
}

#[derive(Clone)]
pub struct Point {
    x: f32,
    y: f32,
    data: *const UserData,
}

impl Copy for Point {}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point::with_data(x, y, std::ptr::null())
    }

    pub fn with_data(x: f32, y: f32, data: *const UserData) -> Point {
        Point {
            x, y, data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_first_point() {
        let mut qt = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
        let point = Point::new(5., 5.);
        qt.insert_point(&point).expect("Could not insert point");
    }
}
