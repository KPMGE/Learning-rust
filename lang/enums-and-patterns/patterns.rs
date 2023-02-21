// patterns in rust are really useful, we can use either the match 
// expression or the if-let syntax. Also, one important point is that, 
// when we match an expression with an enum and that enum has an underlying 
// value attached to it, we consume the value, meaning we take ownership over 
// the value inside the enum, if we don't wanna take ownership but borrow a reference
// instead, we should use the 'ref' and 'ref mut' keywords

enum BinaryTree<T> {
  Empty,
  NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
  value: T,
  left: BinaryTree<T>,
  right: BinaryTree<T>
}

impl<T: Ord + std::fmt::Display> BinaryTree<T> {
  fn insert(&mut self, value: T) {
    match *self {
      BinaryTree::Empty => {
        *self = BinaryTree::NonEmpty(
          Box::new(
            TreeNode {
              value: value,
              right: BinaryTree::Empty,
              left: BinaryTree::Empty,
            }
          )
        )
      }
      // we use 'ref mut' in here, cuz we don't wanna consume the node, instead, we wanna borrow a mutable reference to it
      BinaryTree::NonEmpty(ref mut node) => { 
        if node.value <= value {
          node.left.insert(value);
        } else {
          node.right.insert(value);
        }
      }
    }
  }

  fn display_in_order(&self) {
    if let BinaryTree::NonEmpty(node) = &*self {
      node.right.display_in_order();
      print!("{} ", node.value);
      node.left.display_in_order();
    }
  }
} 

fn main() {
  let mut tree = BinaryTree::Empty;

  tree.insert(2);
  tree.insert(10);
  tree.insert(0);
  tree.insert(200);
  tree.insert(5);

  tree.display_in_order();
}
