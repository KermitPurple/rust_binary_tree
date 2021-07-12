struct Tree<T>(Option<Box<TreeNode<T>>>);

impl<T> Tree<T>
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    fn new(val: T) -> Tree<T> {
        Tree(Some(Box::new(TreeNode::new(val))))
    }

    fn from_vec(vec: Vec<T>) -> Tree<T> {
        let mut root = Tree(None);
        for val in &vec {
            match root {
                Tree(None) => {
                    root = Tree::new(*val);
                }
                Tree(Some(_)) => root.push(*val),
            }
        }
        root
    }

    fn push(&mut self, val: T) {
        match self {
            Tree(None) => {
                *self = Tree::new(val);
            },
            Tree(Some(node)) => {
                if node.val < val {
                    node.right.push(val);
                } else {
                    node.left.push(val);
                }
            }
        };
    }

    fn print_val(&self) {
        match self {
            Tree(Some(x)) => println!("{}", x.val),
            Tree(None) => println!("None"),
        }
    }

    fn lnr(&self) {
        match self {
            Tree(Some(node)) => {
                node.left.lnr();
                print!("{} ", node.val);
                node.right.lnr();
            },
            Tree(None) => (),
        }
    }

    fn contains(&self, value: &T) -> bool {
        match self {
            Tree(Some(node)) => node.val == *value || node.left.contains(value) || node.right.contains(value),
            Tree(None) => false,
        }
    }
}

struct TreeNode<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            val,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let t = Tree::from_vec(vec![5, 3, 7, 4, 6]);
    t.lnr();
    println!("{}", t.contains(&5));
}
