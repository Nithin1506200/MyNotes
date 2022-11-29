use std::{cell::RefCell, cmp, rc::Rc};

use super::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => return 0,
        Some(node) => {
            let node = node.borrow();
            match (node.left.clone(), node.right.clone()) {
                (None, None) => return 1,
                (Some(left), None) => return 1 + max_depth(Some(left)),
                (None, Some(right)) => return 1 + max_depth(Some(right)),
                (Some(left), Some(right)) => {
                    return 1 + cmp::max(max_depth(Some(left)), max_depth(Some(right)))
                }
            }
        }
    }
}
