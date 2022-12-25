use std::{cell::RefCell, cmp, rc::Rc};

use super::TreeNode;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => return 0,
        Some(node) => {
            let node = node.borrow_mut();
            match (node.left.clone(), node.right.clone()) {
                (None, None) => return 0,
                (Some(left), None) => return 1 + min_depth(Some(left)),
                (None, Some(right)) => return 1 + min_depth(Some(right)),
                (Some(left), Some(right)) => {
                    return 1 + cmp::min(min_depth(Some(left)), min_depth(Some(right)))
                }
            }
        }
    }
}
