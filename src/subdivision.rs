use super::rectangle::Rectangle;
use super::Quadtree;

#[derive(Debug)]
pub struct QuadtreeSubdivisions {
    pub nw: Box<Quadtree>,
    pub ne: Box<Quadtree>,
    pub se: Box<Quadtree>,
    pub sw: Box<Quadtree>,
}

impl QuadtreeSubdivisions {
    pub fn new(boundary: &Rectangle, capacity: u8) -> QuadtreeSubdivisions {
        let new_w = boundary.w / 2.;
        let new_h = boundary.h / 2.;

        let nw = Rectangle::new(boundary.x + new_w, boundary.y - new_h, new_w, new_h);
        let ne = Rectangle::new(boundary.x - new_w, boundary.y - new_h, new_w, new_h);
        let sw = Rectangle::new(boundary.x + new_w, boundary.y + new_h, new_w, new_h);
        let se = Rectangle::new(boundary.x - new_w, boundary.y + new_h, new_w, new_h);

        QuadtreeSubdivisions {
            nw: Box::new(Quadtree::with_capacity(&nw, capacity)),
            ne: Box::new(Quadtree::with_capacity(&ne, capacity)),
            sw: Box::new(Quadtree::with_capacity(&sw, capacity)),
            se: Box::new(Quadtree::with_capacity(&se, capacity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let boundary = Rectangle::new(100., 200., 300., 400.);
        let sd = QuadtreeSubdivisions::new(&boundary, 10u8);
        assert_eq!(sd.sw.boundary, Rectangle::new(250., 400., 150., 200.));
        assert_eq!(sd.se.boundary, Rectangle::new(-50., 400., 150., 200.));
        assert_eq!(sd.nw.boundary, Rectangle::new(250., 0., 150., 200.));
        assert_eq!(sd.ne.boundary, Rectangle::new(-50., 0., 150., 200.));
    }
}
