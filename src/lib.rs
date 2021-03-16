fn find_leaf(i: usize) -> (usize, usize, usize, usize) {
    (4*i+1, 4*i+2, 4*i+3, 4*i+4)
}

fn find_parent(i: usize) -> usize {
    (i-1)/4
}

#[derive(Debug)]
pub struct QuadTree<D, P> {
    pub boundary: Shape,
    pub id: Vec<usize>,
    pub loc: Vec<P>,
    pub data: Vec<D>,
}

impl<'a, D, P> QuadTree<D, P> {
    pub fn new(boundary: Shape) -> Self {
        Self {
            boundary,
            id: Vec::new(),
            loc: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn replace(&mut self, _loc: P) -> Option<P> {
        todo!()
    }

    pub fn insert(&mut self, _loc: P) -> bool {
        todo!()
    }

    pub fn remove(&mut self, _loc: P) -> bool {
        todo!()
    }

    pub fn query<F: FnMut(&P)>(&mut self, _range: &Rectangle) {
        todo!()
    }

    pub fn query_mut<F: FnMut(&mut P)>(&mut self, _range: &Rectangle, _func: &mut F) {
        todo!();
    }

    pub fn contains(&self) -> bool {
        // New type enum?
        // enum QuadTreeContents<D, P> {
        //      Id(usize),
        //      Loc(P),
        //      Data(D),
        todo!()
    }

    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id.len() == 0
    }

    pub fn iter(&'a self) {
        todo!()
    }

    pub fn iter_mut() {
        todo!()
    }

    pub fn into_iter(&'a self) {
        todo!()
    }
}

pub struct Rectangle;
pub struct Circle;

// Want to make this a trait
#[derive(Debug)]
pub enum Shape {
    Rect(Rectangle),
    Circle(Circle),
}


// pub trait ShapeTrait {
//     fn contains<P>(&self, pos: P) -> bool;
//     fn intersects<P>(&self, pos: P) -> bool;
// }

#[cfg(test)]
mod test {
}
