use super::*;


#[test]
fn test_rectangle() {
    let r = Rectangle::new(10, 10, 50, 50);
    assert_eq!(r, Rectangle::new(10, 10, 50, 50));
}

// #[test]
// fn test_circle() {
//     let r = Circle::new(10, 10, 50);
//     assert_eq!(r, Circle::new(10, 10, 50));
// }

#[test]
fn quadtree_insert_query() {

    let foos = create_foo(0..9);
    let mut result: Vec<Foo> = Vec::new();

    let (w, h) = (40, 40);
    let bb = Rectangle::new(0, 0, w, h);
    let mut qt = QuadTree::new(bb, 4);

    let bb = Rectangle::new(0, 0, w+10, h+10);

    insert_foo(&mut qt, &foos);

    qt.query(Some(&bb), &mut |e| result.push(e.clone()));

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

#[test]
fn test_insert_same_location() {
    let old = Foo::new(5, 5, "old");
    let new = Foo::new(5, 5, "new");

    let (w, h) = (40, 40);
    let bb = Rectangle::new(0, 0, w, h);

    let mut qt = QuadTree::new(bb, 4);

    qt.insert(&old);
    qt.insert(&new);
    qt.insert(&new);
    qt.insert(&new);
    qt.insert(&new);
    qt.insert(&new);
    qt.insert(&new);

    let mut result = Vec::new();

    qt.query(None, &mut |e| result.push(e.clone()));

    assert_eq!(result[0], old);
    assert_eq!(qt.len(), 1);
}

#[derive(Debug, Clone, PartialEq)]
struct Foo {
    other: String,
    x: u32,
    y: u32,
}

impl Foo {
    fn new(x: u32, y: u32, other: &str) -> Self {
        Self { other: other.to_string(), x, y }
    }
}

impl Vector for Foo {
    fn as_point(&self) -> Option<(u32, u32)> {
        Some((self.x, self.y))
    }
}

fn create_foo(range: std::ops::Range<u32>) -> Vec<Foo> {
    let mut foos = Vec::new();
    for i in range {
        foos.push(Foo::new(i, 0, "FOOOOOOO"));
    }
    foos
}

fn insert_foo(qt: &mut QuadTree<Foo>, foos: &Vec<Foo>) {
    for f in foos.iter() {
        qt.insert(f);
    }
}
