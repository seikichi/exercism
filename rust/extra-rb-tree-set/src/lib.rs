// https://www.cs.princeton.edu/~rs/talks/LLRB/RedBlack.pdfcolor

#[derive(Debug)]
pub struct TreeSet<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    red: bool,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn flip_color(&mut self) {
        self.red = !self.red;
        if let Some(left) = &mut self.left {
            left.red = !left.red;
        }
        if let Some(right) = &mut self.right {
            right.red = !right.red;
        }
    }
}

fn is_red<T>(node: &Option<Box<Node<T>>>) -> bool {
    match node {
        None => false,
        Some(node) => node.red,
    }
}

fn rotate_left<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    let red = h.red;
    let mut x = h.right.unwrap();
    h.red = true;
    h.right = x.left.take();
    x.left = Some(h);
    x.red = red;
    x
}

fn rotate_right<T>(mut h: Box<Node<T>>) -> Box<Node<T>> {
    let red = h.red;
    let mut x = h.left.unwrap();
    h.red = true;
    h.left = x.right.take();
    x.right = Some(h);
    x.red = red;
    x
}

fn insert<T: Ord>(h: Option<Box<Node<T>>>, data: T) -> Option<Box<Node<T>>> {
    match h {
        None => Some(Box::new(Node {
            data,
            red: true,
            left: None,
            right: None,
        })),
        Some(mut h) => {
            if data < h.data {
                h.left = insert(h.left.take(), data);
            } else if data > h.data {
                h.right = insert(h.right.take(), data);
            }

            if is_red(&h.right) {
                h = rotate_left(h);
            }
            if is_red(&h.left) && is_red(&h.left.as_ref().unwrap().left) {
                h = rotate_right(h);
            }
            if is_red(&h.left) && is_red(&h.right) {
                h.flip_color();
            }
            Some(h)
        }
    }
}

impl<T: Ord> TreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        let mut result = 0;
        let mut stack = vec![&self.root];
        while let Some(next) = stack.pop() {
            if let Some(node) = next {
                result += 1;
                stack.push(&node.left);
                stack.push(&node.right);
            }
        }
        result
    }

    pub fn get(&self, data: &T) -> Option<&T> {
        let mut x = &self.root;
        while let Some(n) = x {
            if *data < n.data {
                x = &n.left;
            } else if *data > n.data {
                x = &n.right;
            } else {
                return x.as_ref().map(|x| &x.data);
            }
        }
        None
    }

    pub fn insert(&mut self, data: T) {
        let root = self.root.take();
        self.root = insert(root, data);
        if let Some(node) = &mut self.root {
            node.red = false;
        }
    }
}

// #![feature(box_syntax, box_patterns)]
// pub enum Color {
//     R,
//     B,
// }

// pub enum TreeSet<T> {
//     E,
//     N(Color, Box<TreeSet<T>>, T, Box<TreeSet<T>>),
// }

// impl<T: Ord> TreeSet<T> {
//     pub fn new() -> Self {
//         TreeSet::E
//     }

//     pub fn insert(&mut self, value: T) {
//         let old = std::mem::replace(self, TreeSet::E);
//         if let TreeSet::N(_, left, x, right) = old.ins(value) {
//             *self = TreeSet::N(Color::B, left, x, right);
//         }
//     }

//     pub fn len(&self) -> usize {
//         match self {
//             TreeSet::E => 0,
//             TreeSet::N(_, left, _, right) => 1 + left.len() + right.len(),
//         }
//     }

//     fn balance(self) -> Self {
//         match self {
//             TreeSet::N(
//                 Color::B,
//                 box TreeSet::N(Color::R, box TreeSet::N(Color::R, a, x, b), y, c),
//                 z,
//                 d,
//             )
//             | TreeSet::N(
//                 Color::B,
//                 box TreeSet::N(Color::R, a, x, box TreeSet::N(Color::R, b, y, c)),
//                 z,
//                 d,
//             )
//             | TreeSet::N(
//                 Color::B,
//                 a,
//                 x,
//                 box TreeSet::N(Color::R, box TreeSet::N(Color::R, b, y, c), z, d),
//             )
//             | TreeSet::N(
//                 Color::B,
//                 a,
//                 x,
//                 box TreeSet::N(Color::R, b, y, box TreeSet::N(Color::R, c, z, d)),
//             ) => TreeSet::N(
//                 Color::R,
//                 box TreeSet::N(Color::B, a, x, b),
//                 y,
//                 box TreeSet::N(Color::B, c, z, d),
//             ),
//             _ => self,
//         }
//     }

//     fn ins(self, x: T) -> TreeSet<T> {
//         match self {
//             TreeSet::E => TreeSet::N(Color::R, box TreeSet::E, x, box TreeSet::E),
//             TreeSet::N(color, a, y, b) => {
//                 if x < y {
//                     TreeSet::N(color, box a.ins(x), y, b).balance()
//                 } else if x > y {
//                     TreeSet::N(color, a, y, box b.ins(x)).balance()
//                 } else {
//                     TreeSet::N(color, a, y, b)
//                 }
//             }
//         }
//     }
// }
