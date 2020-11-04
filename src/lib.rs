// quadtree

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn top_left_corner(&self) -> (u32, u32) {
        (self.x - self.w / 2, self.y - self.h / 2)
    }

    fn left(&self) -> u32 {
        self.x - self.w / 2
    }

    fn right(&self) -> u32 {
        self.x + self.w / 2
    }

    fn top(&self) -> u32 {
        self.y - self.h / 2
    }

    fn bottom(&self) -> u32 {
        self.y + self.h / 2
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.w
            && point.y >= self.y
            && point.y <= self.y + self.h
    }

    fn intersects(&self, range: &Rectangle) -> bool {
        !(range.w - range.x > self.x + self.w
            || range.x + range.w < self.w - self.x
            || range.h - range.y > self.y + self.h
            || range.y + range.h < self.h - self.y)
    }
}

// circle struct for a circle shaped query
#[derive(Debug)]
struct Circle {
    x: u32,
    y: u32,
    r: u32,
    r_squared: u32,
}

impl Circle {
    fn new(x: u32, y: u32, r: u32) -> Self {
        let r_squared = r * r;
        Self { x, y, r, r_squared }
    }

    fn contains(&self, point: Point) -> bool {
        // check if the point is in the circle by checking if the euclidean distance of
        // the point and the center of the circle if smaller or equal to the radius of
        // the circle
        let d = (point.x - self.x).pow(2) + (point.y - self.y).pow(2);
        d <= self.r_squared
    }

    fn intersects(&self, range: Rectangle) -> bool {
        let xDist = ((range.x - self.x) as i32).abs();
        let yDist = ((range.y - self.y) as i32).abs();

        // radius of the circle
        let r = self.r;

        let w = range.w;
        let h = range.h;

        let edges = (xDist - w as i32).pow(2) + (yDist - h as i32).pow(2);

        // no intersection
        if xDist > (r + w) as i32 || yDist > (r + h) as i32 {
            return false;
        }

        // intersection within the circle
        if xDist <= w as i32 || yDist <= h as i32 {
            return true;
        }

        // intersection on the edge of the circle
        edges <= self.r_squared as i32
    }
}

// enum BountryBox {
//     Rect(Rectangle),
//     Circle(Circle),
// }

#[derive(Debug)]
pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,
    children: Option<[Box<QuadTree>; 4]>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::with_capacity(capacity as usize),
            children: None,
        }
    }

    fn subdivide(boundary: Rectangle, capacity: usize) -> [Box<QuadTree>; 4] {
        let x = boundary.x;
        let y = boundary.y;
        let w = boundary.w / 2;
        let h = boundary.h / 2;

        let ne = Rectangle::new(x + w, y - h, w, h);
        let nw = Rectangle::new(x - w, y - h, w, h);
        let se = Rectangle::new(x + w, y + h, w, h);
        let sw = Rectangle::new(x - w, y + h, w, h);

        [
            Box::new(QuadTree::new(nw, capacity)),
            Box::new(QuadTree::new(ne, capacity)),
            Box::new(QuadTree::new(sw, capacity)),
            Box::new(QuadTree::new(se, capacity)),
        ]
    }

    pub fn insert(&mut self, point: &Point) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }

        if self.points.len() < self.capacity as usize && !self.points.contains(point) {
            self.points.push(point.clone());
            return true;
        }
        if self.points.len() == self.capacity {
            let (b, c) = (self.boundary, self.capacity);
            return self.children
                .get_or_insert_with(move || Self::subdivide(b, c))
                .iter_mut()
                .any(|c| c.insert(point));
        }
        false 
    }

    pub fn remove(&mut self, point: &Point) -> bool {
        if self.points.contains(point) {
            self.points = self.points.iter().filter(|p| *p != point).map(Clone::clone).collect();
            return true;
        }
        if let Some(c) = &mut self.children {
            for child in c.iter_mut() {
                if child.remove(point) {
                    return true;
                }
            }
        }
        false
    }


    pub fn query(&self, range: &Rectangle, found: &mut Vec<Point>) {
        if !range.intersects(&self.boundary) {
            return;
        }

        for p in &self.points {
            if range.contains(p) {
                found.push(p.clone());
            }
        }

        self.children
            .as_ref()
            .map(|c| c.iter().for_each(|c| c.query(&range, found)));
    }

    pub fn len(&self) -> usize {
        self.points.len()
            + self
                .children
                .as_ref()
                .map(|c| c.iter().fold(0, |x, c| x + c.len()))
                .unwrap_or(0)
    }

    pub fn contains(&self, point: &Point) -> bool {
        if self.points.contains(point) {
            return true;
        }
        if let Some(child) = &self.children {
            return child.iter().any(|ch| ch.contains(point));
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadtree_insert_query() {
        let mut found = Vec::new();
        let (w, h) = (40, 40);
        let bb = Rectangle::new(0, 0, w, h);
        let mut qt = QuadTree::new(bb, 4);
        qt.insert(&Point::new(0, 0));
        qt.insert(&Point::new(w, h));
        qt.query(&Rectangle::new(0, 0, w, h), &mut found);
        dbg!(qt.boundary);
        assert_eq!(found, vec![Point::new(0, 0), Point::new(w, h)]);
    }
}
