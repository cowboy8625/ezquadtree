use crate::{Rectangle, Serialize, Deserialize};

pub trait Vector<Rhs = Self>: Clone + PartialEq + std::fmt::Debug {
    fn as_point(&self) -> Option<(u32, u32)>;
    fn eq_point(&self, other: &Rhs) -> bool;
    fn eq_item(&self, other: &Rhs) -> bool;
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuadTree<T: Vector> {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<T>,
    children: Option<[Box<QuadTree<T>>; 4]>,
}

impl<'a, T: Vector> QuadTree<T> {
    /// Create a new QuadTree object with a boundary and a capacity.
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::with_capacity(capacity as usize),
            children: None,
        }
    }

    // When Nodes(QuadTree) capacity is reached, subdivide is call to create
    // children.of Node(QuadTree).
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


    /// checks to see if location has item not equal to replace.
    pub fn replace(&mut self, item: &T) -> bool {
        if !self.boundary.contains(item) {
            return false;
        }
        if let Some(idx) = self.points.iter().position(|x| x.eq_point(item) && !x.eq_item(item) ) {
            self.points.remove(idx);
            self.points.push(item.clone());
            return true;
        }

        self.children
            .iter_mut()
            .flatten()
            .any(|child| child.replace(item))
    }

    /// Will not overwrite same location.
    pub fn insert(&mut self, item: &T) -> bool {
        if !self.boundary.contains(item) {
            return false;
        }

        if self.points.len() < self.capacity as usize && !self.points.iter().any(|i| i.eq_point(item)) {
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

    /// Removes Location
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

    /// Not yet implemented.
    pub fn query_mut<F: FnMut(&mut T)>(&mut self, _range: &Rectangle, _func: &mut F) {
        todo!();
    }

    /// Can pull out Points from a Rectangle area.
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

        if let Some(c) = self.children.as_ref() { c.iter().for_each(|c| c.query(Some(&range), func)) }
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

    /// Return `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks to see if item is in QuadTree.
    pub fn contains(&self, item: &T) -> bool {
        if self.points.contains(item) {
            return true;
        }
        if let Some(child) = &self.children {
            return child.iter().any(|ch| ch.contains(item));
        }
        false
    }
    // pub fn iter(&self) -> Iter<'_, T> {
    //     self.into_iter()
    // }
}


/*
impl<T: Vector> Iterator for &QuadTree<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
*/

// impl<'a, T: Vector> IntoIterator for &'a QuadTree<T> {
//     type Item = &'a T;
//     type IntoIter = Iter<'a, Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         Iter {
//             tree: QuadTree<
//         }
//     }
// }
//
// pub struct Iter<'a, QuadTree<T>> {
//     tree: QuadTree<T>,
// }
//
// impl<'a, T: Vector> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
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
