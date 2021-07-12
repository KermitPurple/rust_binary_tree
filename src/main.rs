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

    fn print_lnr(&self) {
        self._print_lnr();
        println!();
    }

    fn _print_lnr(&self) {
        match self {
            Tree(Some(node)) => {
                node.left._print_lnr();
                print!("{} ", node.val);
                node.right._print_lnr();
            },
            Tree(None) => (),
        }
    }

    fn print_fs(&self){
        self._print_fs(0);
    }

    fn _print_fs(&self, depth: usize) {
        if let Tree(Some(node)) = self {
            print!("{}", "  ".repeat(depth));
            println!("{}", node.val);
            node.left._print_fs(depth + 1);
            node.right._print_fs(depth + 1);
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

fn make_tree_vals(max: i32) -> Vec<i32>{
    let mut result: Vec<i32> = vec![];
    plus_minus(max / 2, max / 4, &mut result);
    result
}

fn plus_minus(cur: i32, delta: i32, vec: &mut Vec<i32>){
    vec.push(cur);
    if delta == 0 {
        return
    }
    plus_minus(cur + delta, delta / 2, vec);
    plus_minus(cur - delta, delta / 2, vec);
}

fn main() {
    println!("Hello, world!");
    let t = Tree::from_vec(make_tree_vals(100));
    t.print_lnr();
    t.print_fs();
    println!("{}", t.contains(&5));
    println!("{:?}", make_tree_vals(10));
}
