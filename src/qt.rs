use crate::{Rectangle, Serialize, Deserialize};

/// A trait to describe Vector x and y to QuadTree.
pub trait Vector<Rhs = Self>: Clone + PartialEq + std::fmt::Debug {
    /// Pulls point out of Type.
    fn as_point(&self) -> (u32, u32);
}


/// A growable data structure type, with a look up complexity of O(log n).
/// ```rust
/// use ezquadtree::{Vector, QuadTree, Rectangle};
/// #[derive(Debug, Clone)]
/// struct Foo {
///     item: String,
///     x: u32,
///     y: u32,
/// }
/// 
/// impl Foo {
///     fn new(x: u32, y: u32, item: &str) -> Self {
///         Self { item: item.to_string(), x, y }
///     }
/// }
/// 
/// impl Vector for Foo {
///     fn as_point(&self) -> (u32, u32) {
///         (self.x, self.y)
///     }
/// }
/// 
/// impl PartialEq for Foo {
///     fn eq(&self, other: &Foo) -> bool {
///         self.x == other.x && self.y == other.y
///     }
/// }
///
/// fn main() {
///     let old = Foo::new(5, 5, "old");
///     let new = Foo::new(5, 5, "new");
///
///     let (w, h) = (40, 40);
///     let bb = Rectangle::new(0, 0, w, h);
///     let mut qt = QuadTree::new(bb, 4);
///
///     qt.insert(&old);
///     qt.insert(&new);
///
///     let mut result = Vec::new();
///
///     qt.query(None, &mut |e| result.push(e.clone()));
///
///     assert_eq!(result, vec![old.clone()]);
///     assert_eq!(qt.len(), 1);
///
///     let return_of_replace = qt.replace(&new);
///
///     assert_eq!(Some(old.clone()), return_of_replace);
///     assert_eq!(qt.len(), 1);
///
///     qt.query(None, &mut |inner_item| {
///         assert_eq!(inner_item, &new);
///     });
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuadTree<T: Vector> {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<T>,
    children: Option<[Box<QuadTree<T>>; 4]>,
}

impl<'a, T: Vector> QuadTree<T> {
    /// Create a new QuadTree object with a boundary and a capacity.
    /// ```rust
    /// # use ezquadtree::{Rectangle, Vector, QuadTree};
    /// # #[derive(Debug, Clone)]
    /// # struct MyType {
    /// #     item: String,
    /// #     x: u32,
    /// #     y: u32,
    /// # }
    /// # 
    /// # impl MyType {
    /// #     fn new(x: u32, y: u32, item: &str) -> Self {
    /// #         Self { item: item.to_string(), x, y }
    /// #     }
    /// # }
    /// # 
    /// # impl Vector for MyType {
    /// #     fn as_point(&self) -> (u32, u32) {
    /// #         (self.x, self.y)
    /// #     }
    /// # }
    /// # 
    /// # impl PartialEq for MyType {
    /// #     fn eq(&self, other: &MyType) -> bool {
    /// #         self.x == other.x && self.y == other.y
    /// #     }
    /// # }
    /// # fn main() {
    /// let quadtree: QuadTree<MyType> = QuadTree::new(Rectangle::new(0, 0, 40, 40), 4);
    /// # }
    /// ```
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


    /// repleace if x and y are not the same and returns the T
    ///
    /// ```rust
    /// # use ezquadtree::{QuadTree, Vector, Rectangle};
    /// # #[derive(Debug, Clone)]
    /// # struct Foo {
    /// #    x: u32,
    /// #    y: u32,
    /// #    item: String,
    /// # }
    /// # impl Vector for Foo {
    /// #    fn as_point(&self) -> (u32, u32) {
    /// #        (self.x, self.y)
    /// #    }
    /// # }
    /// # impl PartialEq for Foo {
    /// #     fn eq(&self, other: &Foo) -> bool {
    /// #         self.x == other.x && self.y == other.y
    /// #     }
    /// # }
    /// # fn main() {
    /// #
    /// # let boundary = Rectangle::new(0, 0, 40, 40);
    /// # let mut quadtree = QuadTree::new(boundary, 4);
    /// # let item1 = Foo { x: 11, y: 5, item: "thing".to_string() };
    /// # quadtree.insert(&item1);
    /// let item = Foo { x: 10, y: 5, item: "thing".to_string() };
    /// quadtree.replace(&item);
    /// # quadtree.query(None, &mut |i| {
    /// #     assert_eq!(i, &item1);
    /// # });
    /// # }
    /// ```
    pub fn replace(&mut self, item: &T) -> Option<T> {
        if !self.boundary.contains(item) {
            return None;
        }
        if let Some(idx) = self.points.iter().position(|x| x.as_point() == item.as_point()) {
            let old_item = self.points.remove(idx);
            self.points.push(item.clone());
            return Some(old_item);
        }

        self.children
            .iter_mut()
            .flatten()
            .find_map(|child| child.replace(item))
    }

    /// Will not overwrite same location.
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

    /// Checks to see if item is in QuadTree from it's Vector x and y.
    pub fn contains(&self, item: &T) -> bool {
        if self.points.contains(item) {
            return true;
        }
        if let Some(child) = &self.children {
            return child.iter().any(|ch| ch.contains(item));
        }
        false
    }

    /// Not yet implemented.
    pub fn iter(&'a self) { // -> Iter<'a, T> {
        // Iter {
        //     tree: self,
        //     found: Vec::new(),
        //     index: 0,
        // }
        todo!();
    }

    /// Not yet implemented.
    pub fn iter_mut() {
        todo!();
    }

    /// Not yet implemented.
    pub fn into_iter() {
        todo!();
    }
}


/*
impl<T: Vector> Iterator for &QuadTree<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
*/

/*
impl<'a, T: Vector> IntoIterator for &'a QuadTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> where T: Vector + 'a {
    tree: &'a QuadTree<T>,
    found: Vec<&'a T>,
    index: usize,
    none_count: usize,
}

impl<'a, T: Vector> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
    }
}
*/
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
