use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    Solution::postorder_recursive(root, &mut result);
    result
  }

  fn postorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
      Some(node) => {
        Solution::postorder_recursive(node.borrow().left.clone(), result);
        Solution::postorder_recursive(node.borrow().right.clone(), result);
        result.push(node.borrow().val);
      }
      None => return,
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let input = vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      Some(5),
      Some(6),
      Some(7),
    ];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![4, 5, 2, 6, 7, 3, 1]);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![3, 2, 1]);
  }
}
