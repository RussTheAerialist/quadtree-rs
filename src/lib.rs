// How can I integrate with openframeworks datatypes?
// How do I provide an opaque pointer in the point object?

pub mod ffi;
pub mod rectangle;
pub mod subdivision;

use ffi::UserData;
use rectangle::Rectangle;
use subdivision::QuadtreeSubdivisions;
#[repr(C)]
#[derive(Debug)]
pub struct Quadtree {
    pub points: Vec<Point>,
    pub boundary: Rectangle,
    pub children: Option<QuadtreeSubdivisions>,
    pub capacity: u8,
}

impl Quadtree {
    pub fn new(boundary: &Rectangle) -> Quadtree {
        Quadtree::with_capacity(boundary, 10u8)
    }

    pub fn with_capacity(boundary: &Rectangle, capacity: u8) -> Quadtree {
        Quadtree {
            points: Vec::new(),
            boundary: *boundary,
            children: None,
            capacity,
        }
    }

    pub fn insert(&mut self, point: &Point) -> Result<(), ()> {
        if !self.boundary.contains(point) {
            return Err(());
        }

        if self.points.len() < self.capacity.into() {
            self.points.push(*point);
            Ok(())
        } else {
            if self.children.is_none() {
                self.children = Some(QuadtreeSubdivisions::new(&self.boundary, self.capacity));
            }

            let sd = self.children.as_mut().unwrap();
            sd
                .nw
                .insert(point)
                .or_else(|_| sd.ne.insert(point))
                .or_else(|_| sd.sw.insert(point))
                .or_else(|_| sd.se.insert(point))
        }
    }

    pub fn query(&self, pt: &Point, radius: f32) -> Vec<Point> {
        if !self.boundary.contains(pt) {
            return Vec::new();
        }

        let points = self.query_points(pt, radius);
         if let Some(c) = &self.children {
            points
                .chain(c.sw.query_points(pt, radius))
                .chain(c.se.query_points(pt, radius))
                .chain(c.nw.query_points(pt, radius))
                .chain(c.ne.query_points(pt, radius))
                .cloned().collect()
        } else {
            points.cloned().collect()
        }
    }

    fn query_points<'a>(&'a self, pt: &'a Point, radius: f32) -> impl Iterator<Item=&'a Point> + 'a {
        self.points.iter().filter(move |o| o.within(pt, radius))
    }
}

#[derive(Clone, Debug, PartialEq)]
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
        Point { x, y, data }
    }

    pub fn within(&self, pt: &Point, radius: f32) -> bool {
        let (dx, dy) = ((self.x - pt.x).abs(), (self.y - pt.y).abs());

        if dx + dy <= radius.powi(2) { return true; }
        if dx > radius { return false; }
        if dx > radius { return false; }

        dx.powi(2) + dy.powi(2) <= radius.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_first_point() {
        let mut qt = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
        let point = Point::new(5., 5.);
        qt.insert(&point).expect("Could not insert point");
    }

    #[test]
    fn test_query_not_inside() {
        let qt = Quadtree::new(&Rectangle::new(0., 0., 10., 10.));
        assert_eq!(qt.query(&Point::new(99., 99.), 5.).len(), 0);
    }

    #[test]
    fn test_query_single_point_inside_radius() {
        let mut qt = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
        let point = Point::new(5., 5.);
        qt.insert(&point).expect("Could not insert point");
        let result = qt.query(&Point::new(6., 5.), 2.);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], point);
    }

    #[test]
    fn test_query_one_point_inside_radius() {
        let mut qt = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
        let point = Point::new(5., 5.);
        qt.insert(&point).expect("Could not insert point");
        qt.insert(&Point::new(10., 20.))
            .expect("Unable to insert second point");
        let result = qt.query(&Point::new(6., 5.), 2.);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], point);
    }

    #[test]
    fn test_query_outside_radius() {
        let mut qt = Quadtree::new(&Rectangle::new(10., 10., 10., 10.));
        let point = Point::new(5., 5.);
        qt.insert(&point).expect("Could not insert point");
        qt.insert(&Point::new(10., 20.))
            .expect("Unable to insert second point");
        let result = qt.query(&Point::new(1., 1.), 2.);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_capacity_subdivisions() {
        let mut qt = Quadtree::with_capacity(&Rectangle::new(10., 10., 10., 10.), 1);
        qt.insert(&Point::new( 1., 1.)).expect("Cannot insert pt1");
        qt.insert(&Point::new( 9., 1.)).expect("Cannot insert pt2");
        qt.insert(&Point::new( 1., 11.)).expect("Cannot insert pt3");

        assert!(qt.children.is_some());
        assert_eq!(qt.points.len(), 1);
        assert_eq!(qt.children.as_ref().unwrap().ne.points.len(), 1);
        assert_eq!(qt.children.as_ref().unwrap().se.points.len(), 1);
    }

    #[test]
    fn test_query_with_depth() {
        let mut qt = Quadtree::with_capacity(&Rectangle::new(10., 10., 10., 10.), 1);
        qt.insert(&Point::new( 1., 1.)).expect("Cannot insert pt1");
        qt.insert(&Point::new( 9., 1.)).expect("Cannot insert pt2");
        qt.insert(&Point::new( 1., 11.)).expect("Cannot insert pt3");

        let result = qt.query(&Point::new(10., 10.), 5.);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_query_no_depth() {
        let mut qt = Quadtree::with_capacity(&Rectangle::new(10., 10., 10., 10.), 10);
        qt.insert(&Point::new( 1., 1.)).expect("Cannot insert pt1");
        qt.insert(&Point::new( 9., 1.)).expect("Cannot insert pt2");
        qt.insert(&Point::new( 1., 11.)).expect("Cannot insert pt3");

        let result = qt.query(&Point::new(10., 10.), 5.);
        assert_eq!(result.len(), 3);
    }
}
