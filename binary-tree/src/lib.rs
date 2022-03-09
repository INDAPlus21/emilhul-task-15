
pub struct RedBlackTree<T> {
    root: TreeNode<T>,
    size: usize,
}

impl <T> RedBlackTree<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    pub fn new(data: T, key: usize) -> Self {
        let root = TreeNode::new(data, key, Color::Black);
        let size = 1;
        Self {root, size}
    }

    pub fn search(&mut self, key: usize) -> T {
        match self.common_search(key) {
            (_node , None) => _node.data,
            _ => panic!("No node found for given key!")
        }
    }

    pub fn insert(&mut self, data: T, key: usize) {
        let (mut parent_node, dir) = match self.common_search(key) {
            (_node, Some(_dir)) => (_node, _dir),
            _ => panic!("Key already exists in tree. Does not accept duplicates!")
        };
        let mut node = TreeNode::new(data, key, Color::Red);
        match dir {
            Direction::Left => parent_node.left = Some(Box::new(node)),
            Direction::Right => parent_node.right = Some(Box::new(node)),
        }
    }

    fn common_search(&mut self, key: usize) -> (TreeNode<T>, Option<Direction>)  {
        let mut node = self.root.clone();
        let mut dir: Option<Direction> = None;
        while node.key != key {
            if node.key > key {
                match node.left {
                    Some(_node) => node = *_node,
                    None => dir = Some(Direction::Left),
                }
            } else {
                match node.right {
                    Some(_node) => node = *_node,
                    None => dir = Some(Direction::Right),
                }
            }
        }
        (node, dir)
    }
}

#[derive(Clone)]
struct TreeNode<T> {
    color: Color,
    data: T,
    key: usize,
    parent: Option<Box<TreeNode<T>>>,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct NodePtr<T>(*mut TreeNode<T>);

impl <T> TreeNode<T> {
    fn new(data: T, key: usize, color: Color,) -> Self {
        Self {color, data, key, parent: None, left: None, right: None}
    }
}

#[derive(Clone)]
enum Color {
    Black, Red,
}

enum Direction {
    Right, Left,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let t = RedBlackTree::<String>::new(format!("10"), 10);
        let t_val = t.root.data;
        assert_eq!(t_val, format!("10"))
    }
}

