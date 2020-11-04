// quadtree

pub trait Vector: Clone + PartialEq {
    fn point(&self) -> (u32, u32);
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

    fn contains<T>(&self, item: &T) -> bool where T: Vector {
        let (x, y) = item.point();
        x >= self.x
            && x <= self.x + self.w
            && y >= self.y
            && y <= self.y + self.h
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

    fn contains<T>(&self, item: T) -> bool where T: Vector {
        // check if the point is in the circle by checking if the euclidean distance of
        // the point and the center of the circle if smaller or equal to the radius of
        // the circle
        let (x, y) = item.point();
        let d = (x - self.x).pow(2) + (y - self.y).pow(2);
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

#[derive(Debug)]
pub struct QuadTree<T: Vector> {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<T>,
    children: Option<[Box<QuadTree<T>>; 4]>,
}

impl<T: Vector> QuadTree<T> {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::with_capacity(capacity as usize),
            children: None,
        }
    }

    fn subdivide(boundary: Rectangle, capacity: usize) -> [Box<QuadTree<T>>; 4] {
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

    pub fn insert(&mut self, item: &T) -> bool {
        if !self.boundary.contains(item) {
            return false;
        }

        if self.points.len() < self.capacity as usize && !self.points.contains(item) {
            self.points.push(item.clone());
            return true;
        }
        if self.points.len() == self.capacity {
            let (b, c) = (self.boundary, self.capacity);
            return self.children
                .get_or_insert_with(move || Self::subdivide(b, c))
                .iter_mut()
                .any(|c| c.insert(item));
        }
        false 
    }

    pub fn remove(&mut self, item: &T) -> bool {
        if self.points.contains(item) {
            self.points = self.points.iter().filter(|p| *p != item).map(Clone::clone).collect();
            return true;
        }
        if let Some(c) = &mut self.children {
            for child in c.iter_mut() {
                if child.remove(item) {
                    return true;
                }
            }
        }
        false
    }


    pub fn query(&self, range: &Rectangle, found: &mut Vec<T>) {
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

    pub fn contains(&self, item: &T) -> bool {
        if self.points.contains(item) {
            return true;
        }
        if let Some(child) = &self.children {
            return child.iter().any(|ch| ch.contains(item));
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Foo {
        x: u32,
        y: u32,
    }

    impl Foo {
        fn new(x: u32, y: u32) -> Self {
            Self { x, y }
        }
    }

    impl Vector for Foo {
        fn point(&self) -> (u32, u32) {
            (self.x, self.y)
        }
    }

    #[test]
    fn quadtree_insert_query() {

        let a = Foo::new(0, 0);
        let b = Foo::new(10, 10);


        let mut found = Vec::new();
        let (w, h) = (40, 40);
        let bb = Rectangle::new(0, 0, w, h);
        let mut qt = QuadTree::new(bb, 4);

        qt.insert(&a);
        qt.insert(&b);
        qt.query(&Rectangle::new(0, 0, w, h), &mut found);

        assert_eq!(found, vec![a, b]);
    }
}
