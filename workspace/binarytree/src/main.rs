enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord + std::fmt::Display> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(
                    TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty
                    }
                ));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
    /// Inorder traversal
    fn print(&self) {
        match *self {
            BinaryTree::Empty => return (),
            BinaryTree::NonEmpty(ref node) => {
                //   node: &Box<TreeNode<T>>
                //  *node: Box<TreeNode<T>>
                // **node: TreeNode<T>
                // (**node).element: T
                // (**node).left: BinaryTree<T>
                // (**node).right: BinaryTree<T>
                // below is maybe implicit dereferencing?
                node.left.print();
                println!("{} ", node.element);
                node.right.print();
            }
        }
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add(5);
    tree.add(3);
    tree.add(7);
    tree.add(10);
    tree.print();
}
