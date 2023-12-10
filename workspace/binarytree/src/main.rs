enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// impl<T> BinaryTree<T> {
    // IMPLEMENT add_left, add_right 
// }

use self::BinaryTree::*;
fn main() {
    let jupiter_tree = NonEmpty(Box::new(
        TreeNode {
            element: "Jupiter",
            left: Empty,
            right: Empty,
        }
    ));

    let mars_tree = NonEmpty(Box::new(
        TreeNode {
            element: "Mars",
            left: jupiter_tree,
            right: Empty,
        }
    ));
}
