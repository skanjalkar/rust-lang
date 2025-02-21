use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct FindElements {
    values: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = HashSet::new();

        fn recover(node: Option<Rc<RefCell<TreeNode>>>, val: i32, values: &mut HashSet<i32>) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                n.val = val;
                values.insert(val);

                // Recover left child (2 * x + 1)
                recover(n.left.clone(), 2 * val + 1, values);
                // Recover right child (2 * x + 2)
                recover(n.right.clone(), 2 * val + 2, values);
            }
        }

        recover(root, 0, &mut values);
        FindElements { values }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

fn main() {
    // ["FindElements","find","find","find"]
    // [[[-1,-1,-1,-1,-1]],[1],[3],[5]]

    let root = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
    let find_elements = FindElements::new(root);
    assert_eq!(find_elements.find(1), true);
    assert_eq!(find_elements.find(3), true);
    assert_eq!(find_elements.find(5), true);
}
