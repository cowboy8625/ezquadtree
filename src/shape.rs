use crate::{Serialize, Deserialize, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
/// Rectangle is to represent a rectangular bounding box.
/// # Example
/// ```
/// # use ezquadtree::Rectangle;
/// # fn main() {
/// // for now it only takes a u32 but At some point will be a T.
/// let rect = Rectangle::new(0, 0, 40, 40);
/// # }
/// ```
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rectangle {
    /// Create a new Rectangle.
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Get the top left most x and y of rectangle.
    pub fn top_left_corner(&self) -> (u32, u32) {
        (self.x - self.w / 2, self.y - self.h / 2)
    }

    /// Checks to see if a a given Vector is in the QuadTree.
    pub fn contains<T>(&self, item: &T) -> bool where T: Vector {
        let (x, y) = Vector::as_point(item);
        x >= self.x
            && x < self.x + self.w
            && y >= self.y
            && y < self.y + self.h
    }

    /// Checks to see if any part of another Rectangle overlaps its self.
    pub fn intersects(&self, range: &Rectangle) -> bool {
        Self::range_intersects(self.get_range_x(), range.get_range_x())
            && Self::range_intersects(self.get_range_y(), range.get_range_y())
    }

    // returns a Range for X.
    fn get_range_x(&self) -> std::ops::Range<u32> {
        self.x..(self.x + self.w)
    }

    // returns a Range for Y.
    fn get_range_y(&self) -> std::ops::Range<u32> {
        self.y..(self.y + self.h)
    }

    // return true if ranges overlap.
    fn range_intersects(mut range1: std::ops::Range<u32>, mut range2: std::ops::Range<u32>) -> bool {
        if range1.start > range2.start {
            std::mem::swap(&mut range1, &mut range2);
        }
        range1.end > range2.start
    }
}

// circle struct for a circle shaped query
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Circle {
    x: u32,
    y: u32,
    r: u32,
    r_squared: u32,
}

#[allow(dead_code)]
impl Circle {
    pub fn _new(x: u32, y: u32, r: u32) -> Self {
        let r_squared = r * r;
        Self { x, y, r, r_squared }
    }

    pub fn contains<T>(&self, item: T) -> bool where T: Vector {
        // check if the point is in the circle by checking if the euclidean distance of
        // the point and the center of the circle if smaller or equal to the radius of
        // the circle
        let (x, y) = Vector::as_point(&item);
        let d = (x - self.x).pow(2) + (y - self.y).pow(2);
        d <= self.r_squared
    }

    pub fn intersects(&self, range: Rectangle) -> bool {
        let x_dist = ((range.x - self.x) as i32).abs();
        let y_dist = ((range.y - self.y) as i32).abs();

        // radius of the circle
        let r = self.r;

        let w = range.w;
        let h = range.h;

        let edges = (x_dist - w as i32).pow(2) + (y_dist - h as i32).pow(2);

        // no intersection
        if x_dist > (r + w) as i32 || y_dist > (r + h) as i32 {
            return false;
        }

        // intersection within the circle
        if x_dist <= w as i32 || y_dist <= h as i32 {
            return true;
        }

        // intersection on the edge of the circle
        edges <= self.r_squared as i32
    }
}
