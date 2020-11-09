//! quadtree
//!
//! Easy Quad Tree (ezquadtree)
//!
//! This is the first Implementation of a quad tree I have made so there is some room for
//! improvement.
//!
//! A [QuadTree](https://www.i-programmer.info/programming/theory/1679-quadtrees-and-octrees.html) uses the Hilbert curve and can be explained [here](http://blog.notdot.net/2009/11/Damn-Cool-Algorithms-Spatial-indexing-with-Quadtrees-and-Hilbert-Curves)
//! ```rust
//! use ezquadtree::{Vector, QuadTree, Rectangle};
//! #[derive(Debug, Clone)]
//! struct Foo {
//!     item: String,
//!     x: u32,
//!     y: u32,
//! }
//! 
//! impl Foo {
//!     fn new(x: u32, y: u32, item: &str) -> Self {
//!         Self { item: item.to_string(), x, y }
//!     }
//! }
//! 
//! impl Vector for Foo {
//!     fn as_point(&self) -> (u32, u32) {
//!         (self.x, self.y)
//!     }
//! }
//! 
//! impl PartialEq for Foo {
//!     fn eq(&self, other: &Foo) -> bool {
//!         self.x == other.x && self.y == other.y
//!     }
//! }
//!
//! fn main() {
//!     let old = Foo::new(5, 5, "old");
//!     let new = Foo::new(5, 5, "new");
//!
//!     let (w, h) = (40, 40);
//!     let bb = Rectangle::new(0, 0, w, h);
//!     let mut qt = QuadTree::new(bb, 4);
//!
//!     qt.insert(&old);
//!     qt.insert(&new);
//!
//!     let mut result = Vec::new();
//!
//!     qt.query(None, &mut |e| result.push(e.clone()));
//!
//!     assert_eq!(result, vec![old.clone()]);
//!     assert_eq!(qt.len(), 1);
//!
//!     let return_of_replace = qt.replace(&new);
//!
//!     assert_eq!(Some(old.clone()), return_of_replace);
//!     assert_eq!(qt.len(), 1);
//!
//!     qt.query(None, &mut |inner_item| {
//!         assert_eq!(inner_item, &new);
//!     });
//! }
//! ```

#[cfg(test)]
mod test;

mod shape;
mod qt;

use serde::{Serialize, Deserialize};
pub use shape::Rectangle;
pub use qt::{QuadTree, Vector};
