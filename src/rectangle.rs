use super::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle { x, y, w, h }
    }
}

impl Rectangle {
    pub fn contains(self, point: &Point) -> bool {
				!(point.x <= self.x - self.w ||
				 point.x > self.x + self.w ||
				 point.y <= self.y - self.h ||
				 point.y > self.y + self.h)
    }
}

#[cfg(test)]
mod tests {
		use super::*;

		#[test]
		fn test_contains() {
				let rect = Rectangle::new(0., 0., 50., 50.);
				assert!(rect.contains(&Point::new(10., 10.)));
				assert!(rect.contains(&Point::new(-10., -10.)));
				assert!(!rect.contains(&Point::new(100., 0.)));
				assert!(!rect.contains(&Point::new(0., 100.)));
				assert!(!rect.contains(&Point::new(100., 100.)));
				assert!(!rect.contains(&Point::new(-100., 100.)));
		}
}