// quadtree
#[cfg(test)]
mod test;

mod shape;
mod qt;

use serde::{Serialize, Deserialize};
pub use shape::Rectangle;
pub use qt::{QuadTree, Vector};



