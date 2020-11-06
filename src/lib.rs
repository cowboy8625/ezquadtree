// quadtree

use serde::{Serialize, Deserialize};

pub trait Vector: Clone + PartialEq + std::fmt::Debug {
    fn as_point(&self) -> (u32, u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    fn _left(&self) -> u32 {
        self.x - self.w / 2
    }

    fn _right(&self) -> u32 {
        self.x + self.w / 2
    }

    fn _top(&self) -> u32 {
        self.y - self.h / 2
    }

    fn _bottom(&self) -> u32 {
        self.y + self.h / 2
    }

    fn contains<T>(&self, item: &T) -> bool where T: Vector {
        let (x, y) = Vector::as_point(item);
        x >= self.x
            && x < self.x + self.w
            && y >= self.y
            && y < self.y + self.h
    }

    fn intersects(&self, range: &Rectangle) -> bool {
        Self::range_intersects(self.get_range_x(), range.get_range_x())
            && Self::range_intersects(self.get_range_y(), range.get_range_y())
    }

    fn get_range_x(&self) -> std::ops::Range<u32> {
        self.x..(self.x + self.w)
    }

    fn get_range_y(&self) -> std::ops::Range<u32> {
        self.y..(self.y + self.h)
    }

    fn range_intersects(mut range1: std::ops::Range<u32>, mut range2: std::ops::Range<u32>) -> bool {
        if range1.start > range2.start {
            std::mem::swap(&mut range1, &mut range2);
        }
        range1.end > range2.start
    }
}

// circle struct for a circle shaped query
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    x: u32,
    y: u32,
    r: u32,
    r_squared: u32,
}

impl Circle {
    pub fn new(x: u32, y: u32, r: u32) -> Self {
        let r_squared = r * r;
        Self { x, y, r, r_squared }
    }

    fn _contains<T>(&self, item: T) -> bool where T: Vector {
        // check if the point is in the circle by checking if the euclidean distance of
        // the point and the center of the circle if smaller or equal to the radius of
        // the circle
        let (x, y) = Vector::as_point(&item);
        let d = (x - self.x).pow(2) + (y - self.y).pow(2);
        d <= self.r_squared
    }

    fn _intersects(&self, range: Rectangle) -> bool {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuadTree<T: Vector> {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<T>,
    children: Option<[Box<QuadTree<T>>; 4]>,
}

impl<'a, T: Vector> QuadTree<T> {
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

        let nw = Rectangle::new(x, y, w, h);
        let ne = Rectangle::new(x + w, y, w, h);
        let sw = Rectangle::new(x, y + h, w, h);
        let se = Rectangle::new(x + w, y + h, w, h);

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

    pub fn query_mut<F: FnMut(&mut T)>(&mut self, _range: &Rectangle, _func: &mut F) {
        todo!();
    }

    pub fn query<F: FnMut(&T)>(&self, range: Option<&Rectangle>, func: &mut F) {
        let range = range.unwrap_or(&self.boundary);
        if !range.intersects(&self.boundary) {
            return;
        }

        for p in &self.points {
            if range.contains(p) {
                func(p);
            }
        }

        self.children
            .as_ref()
            .map(|c| c.iter().for_each(|c| c.query(Some(&range), func)));
    }

    /// Return the total number of items in QuadTree
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

// impl<T: Vector> IntoIterator for QuadTree<T> {
//     type Item = T;
//     type IntoIter = IntoIter<T>;
//     fn into_iter(self) -> Self::IntoIter {
//     }
// }
//
// pub struct IntoIter<T: Vector> {
//     tree: QuadTree<T>,
// }
//
// impl<T: Vector> Iterator for IntoIter<T> {
//     type Item = i8;
//     fn next(&mut self) -> Option<i8> {
//     }
// }

/*
use std::iter::Iterator;
enum QuadTree {
    Internal(Box<[QuadTree; 4]>),
    Leaf(Option<u32>),
}

impl QuadTree {
    fn into_iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a u32> + 'a> {
        match self {
            QuadTree::Internal(children) => children.iter()
                .fold(
                    Box::new(std::iter::empty()) as Box<dyn Iterator<Item = _>>,
                    |i, c| Box::new(i.chain(c.into_iter()))
                    ),
            QuadTree::Leaf(points) => Box::new(points.iter())
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
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
        fn as_point(&self) -> (u32, u32) {
            (self.x, self.y)
        }
    }

    fn create_foo(range: std::ops::Range<u32>) -> Vec<Foo> {
        let mut foos = Vec::new();
        for i in range {
            foos.push(Foo::new(i, 0));
        }
        foos
    }

    fn insert_foo(qt: &mut QuadTree<Foo>, foos: &Vec<Foo>) {
        for f in foos.iter() {
            qt.insert(f);
        }
    }

    #[test]
    fn test_rectangle() {
        let r = Rectangle::new(10, 10, 50, 50);
        assert_eq!(r, Rectangle::new(10, 10, 50, 50));
    }

    #[test]
    fn test_circle() {
        let r = Circle::new(10, 10, 50);
        assert_eq!(r, Circle::new(10, 10, 50));
    }

    #[test]
    fn quadtree_insert_query() {

        let foos = create_foo(0..9);
        let mut result: Vec<Foo> = Vec::new();

        let (w, h) = (40, 40);
        let bb = Rectangle::new(0, 0, w, h);
        let mut qt = QuadTree::new(bb, 4);

        let bb = Rectangle::new(0, 0, w+10, h+10);

        insert_foo(&mut qt, &foos);

        qt.query(Some(&bb), &mut |e| result.push(*e));

        assert_eq!(result, foos);
    }

    #[test]
    fn test_len() {
        let foos = create_foo(0..9);

        let (w, h) = (40, 40);
        let bb = Rectangle::new(0, 0, w, h);

        let mut qt = QuadTree::new(bb.clone(), 4);

        insert_foo(&mut qt, &foos);

        assert_eq!(qt.len(), 9);
    }
}
