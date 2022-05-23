/*
    AVL Tree in rust.
    Author: Emil Hultcrantz <emilhul@kth.se>
    Inspired by https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/
*/

use std::cmp::{max, Ordering};
use std::mem::{replace, swap};


#[derive(Debug, PartialEq, Clone)]
/// A single node in the AVL Tree.
pub struct AVLNode<T: Ord> {
    value: T,
    left: AVLTree<T>,
    right: AVLTree<T>,
    height: usize,
}

impl <T: Ord> AVLNode<T> {
    /// Creates a new AVLNode with given value T 
    fn new(value: T) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }
    
    /*
            r                        L
           / \     Right Rotate     / \
          L   R       ———>         LL  r
         / \                          / \
        LL LR                        LR  R
        
        r = root
        L = left
        LL = left left
        LR = left right
        R = right
    */
    /// Rotate tree right around node
    fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left = self.left
                                            .as_mut()
                                            .unwrap();
        
        // Take left side of tree
        let left_right = left.right.take();
        let left_left = left.left.take();

        // Put LL in correct place
        let mut new_right_tree = replace(&mut self.left, left_left);

        // Swap value of r and L. To avoid actually moving the root
        swap(&mut self.value, &mut new_right_tree
                                            .as_mut()
                                            .unwrap()
                                            .value);
        
        // Take right side
        let right = self.right.take();

        // Set node new right to new_right_tree now containing the root value
        // Then put right and left_right as it's children 
        let new_right = new_right_tree
                                                .as_mut()
                                                .unwrap();
        new_right.left = left_right;
        new_right.right = right;

        // Lastly put the right node of self, with value of left to the new_right_tree
        self.right = new_right_tree;
        
        // Calculate new height of right side.
        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        // Update own height
        self.update_height();

        true
    }

        /*
            r                        R
           / \     Left Rotatse     / \
          L   R       ———>         r  RR
             / \                  / \
            RL RR                L  RL
        
        r = root
        L = left
        R = right
        RL = right left
        RR = right right
    */
    /// Rotate tree left around node
    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right = self.right
                                            .as_mut()
                                            .unwrap();
        
        // Take right side of tree
        let right_right = right.right.take();
        let right_left = right.left.take();

        // Put RR in correct place
        let mut new_left_tree = replace(&mut self.right, right_right);
        
        // Swap value of r and R. To avoid actually moving the root
        swap(&mut self.value, &mut new_left_tree
                                            .as_mut()
                                            .unwrap()
                                            .value);
        
        // Take left side
        let left = self.left.take();

        // Set node new left to new_left_tree now containing the root value
        // Then put right and left_right as it's children 
        let new_left = new_left_tree
                                                .as_mut()
                                                .unwrap();
        new_left.left = left;
        new_left.right = right_left;

        // Lastly put the right node of self, with value of left to the new_right_tree
        self.left = new_left_tree;
        
        // Calculate new height of right side.
        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        // Update own height
        self.update_height();

        true
    }

    /// Rebalnce the tree by rotating it appropiately
    fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();

                true
            },
            2 => {
                let left_node = self.left.as_mut().unwrap();
                
                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self. rotate_right();

                true
            },
            _ => false,
            
        }
    }

    /// Height of left side
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |node | node.height)
    }

    /// Height of right side
    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |node | node.height)
    }

    /// Difference in hight between both sides
    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }
} 
 
type AVLTree<T> = Option<Box<AVLNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
/// An ordered set based on a AVL Tree.
pub struct AVLTreeSet<T: Ord> {
    root: AVLTree<T>,
}

impl<T: Ord> Default for AVLTreeSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl <T: Ord> AVLTreeSet<T> {
    /// Creates a new, empty AVLTreeSet.
    /// 
    /// Does not allocate anything on its own.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Adds a value to the set.
    /// 
    /// If the set did not have an equal element present, true is returned.
    /// 
    /// IF the set did have an equal element present, false is returned, and the enntry is nor updated.
    pub fn insert(&mut self, value: T) -> bool {
        let mut current_tree = &mut self.root;
        let mut prev_ptrs = Vec::<*mut AVLNode<T>>::new();
        
        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);
            match current_node.value.cmp(&value) {
                Ordering::Greater => current_tree = &mut current_node.left,
                Ordering::Equal => return false,
                Ordering::Less => current_tree = &mut current_node.right, 
            }
        }

        *current_tree = Some(Box::new(AVLNode::new(value)));

        for ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    /// Returns true if set contains an element equal to the value.
    pub fn contains(&self, value: &T) -> bool {
        let mut current_tree = &self.root;
        
        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Greater => current_tree = &current_node.left,
                Ordering::Equal => return true,
                Ordering::Less => current_tree = &current_node.right, 
            }
        }
        false
    }

    /// Gets an iterator that visits the elements in the AVLTree in ascending order.
    pub fn iter(&self) -> impl Iterator<Item = &'_ T> + '_ {
        self.node_iter().map(|_node| &_node.value)
    }

    /// Removes and returns the element in the set, if any, that is equal to the value.
    pub fn take(&mut self, value: &T) -> Option<T> {
        let mut current_tree = &mut self.root;
        let mut prev_ptrs = Vec::<*mut AVLNode<T>>::new();
        let mut target_value = None;
        
        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Greater => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.left;
                }
                Ordering::Equal => {
                    target_value = Some(&mut **current_node);
                    break;
                }
                Ordering::Less => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.right;
                }
            }
        }

        // If target_value is none menas the element is not in the tree
        target_value.as_ref()?;

        let target_node = target_value.unwrap();

        // Take value. Returning the value of the node and deleting it
        let taken_value = if target_node.left.is_none() || target_node.right.is_none() {
            // If node has one or zero children
            // If one child replace the target node with its child
            if let Some(_left) = target_node.left.take() {
                replace(target_node, *_left).value
            } else if let Some(_right) = target_node.right.take() {
                replace(target_node, *_right).value
            } else {
                // Zero children we need to get parent node
                if let Some(prev_ptr) = prev_ptrs.pop() {
                    let prev_node = unsafe { &mut *prev_ptr };
                    
                    // Check which of parent's children is target_node
                    // Take that node
                    let _value = if let Some(ref _left) = prev_node.left {
                        if _left.value == target_node.value {
                            prev_node.left.take().unwrap().value
                        } else {
                            prev_node.right.take().unwrap().value
                        }
                    } else {
                        prev_node.right.take().unwrap().value
                    };

                    // Update and rebalance parent
                    prev_node.update_height();
                    prev_node.rebalance();

                    _value
                } else {
                    // No parent means we pnly hace root
                    // Take root node
                    self.root.take().unwrap().value
                }
            }
        } else { 
            // If node has two children:
            // Begin at right child.
            // Traverse left children until reaching leftmost child.
            // Replace target with leftmost child.
            // Replace leftmost with its right child if it has one.
            // Update nodes
            /*
                t                        RL
               / \   Delete with two    /  \
              L   R      children      L    R
                 / \       ———>            / \
                RL RR                    RLR  RR
                  \            
                   RLR
            
            t = target
            L = left
            R = right
            RL = right left
            RR = right right
            RLR = right left righ
            */
            
            // Start at right node of target
            let right_tree = &mut target_node.right;
            if right_tree.as_ref().unwrap().left.is_none() {
                // If there is not a left child of the right child of target
                let mut right_node = right_tree.take().unwrap();
                
                // Replace target with right node
                let _value = replace(&mut target_node.value, right_node.value);

                // Replace right node with its right child if any
                let _ = replace(&mut target_node.right, right_node.right.take());

                // Update node
                target_node.update_height();
                target_node.rebalance();

                _value
            } else {
                // If right child has a left child
                let mut next_tree = right_tree;
                let mut _prev_ptrs = Vec::<*mut AVLNode<T>>::new();
    
                // While there are children to the left
                while let Some(_next_left) = next_tree {
                    if _next_left.left.is_some() {
                        _prev_ptrs.push(&mut **_next_left);
                    }
                    next_tree = &mut _next_left.left;
                }
    
                // Get the parent node. Which is at top of pointer stack
                let parent_left = unsafe { &mut *_prev_ptrs.pop().unwrap() };
                let mut leftmost = parent_left.left.take().unwrap();
    
                // Replace target node with this leftmost child.
                // Since it is easier to just switch value we do that instead.
                let _value = replace(&mut target_node.value, leftmost.value);
    
                // Replace the spot where leftmost was with its right child.
                let _ = replace(&mut parent_left.left, leftmost.right.take());
    
                // Update the nodes
                parent_left.update_height();
                parent_left.rebalance();
    
                for ptr in _prev_ptrs.into_iter().rev() {
                    let node = unsafe { &mut *ptr };
                    node.update_height();
                    node.rebalance();
                }
                
                target_node.update_height();
                target_node.rebalance();
    
                _value
            }
        };

        // Update the nodes
        for ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *ptr };
            node.update_height();
            node.rebalance();
        }

        Some(taken_value)
    }

    /// An iterator over the nodes instead of the values they contain
    fn node_iter(&self) -> impl Iterator<Item = &'_ AVLNode<T>> + '_ {
        AVLTreeSetNodeIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

impl<T: Ord> FromIterator<T> for AVLTreeSet<T> {
    /// Create an AVLTreeSet from an iterator.s
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
    }
}

#[derive(Debug)]
pub struct AVLTreeSetNodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AVLNode<T>>,
    current_tree: &'a AVLTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AVLTreeSetNodeIter<'a, T> {
    type Item = &'a AVLNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(prev_nodes) => {
                        self.current_tree = &prev_nodes.right;

                        return  Some(prev_nodes);
                    }
                },
                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }
                    
                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;

                        return Some(current_node);
                    }

                    self.current_tree = &None;

                    return Some(current_node);
                }
            }
        }
    }
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, TestResult};
    use std::collections::BTreeSet;

    impl<T: Arbitrary + Ord> Arbitrary for AVLTreeSet<T> {
        fn arbitrary(g: &mut Gen) -> Self {
            let vec: Vec<T> = Arbitrary::arbitrary(g);
            vec.into_iter().collect()
        }
    
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let vec: Vec<T> = self.iter().cloned().collect();
            Box::new(vec.shrink().map(|v| v.into_iter().collect::<Self>()))
        }
    }

    #[quickcheck]
    fn rotate_left_and_rotate_right_identity(set: AVLTreeSet<u8>) -> TestResult {
        if set.root.is_none() {
            return TestResult::discard();
        }

        let mut rotated_set = set.clone();
        let root_node = rotated_set.root.as_mut().unwrap();

        if root_node.rotate_left() {
            root_node.rotate_right();
        } else {
            root_node.rotate_right();
            root_node.rotate_left();
        }

        TestResult::from_bool(rotated_set == set)
    }


    #[quickcheck]
    fn rotate_right_tilts_balance_factor(xs: Vec<u32>) -> TestResult {
        let mut set = xs.iter().cloned().collect::<AVLTreeSet<_>>();

        if !set.root.is_some() {
            return TestResult::discard();
        }

        let root_node = set.root.as_mut().unwrap();
        let balance_factor = root_node.balance_factor();

        if !root_node.rotate_right() {
            return TestResult::discard();
        }

        let tilted_factor = root_node.balance_factor();

        TestResult::from_bool(balance_factor - tilted_factor >= 2)
    }

    #[quickcheck]
    fn rotate_left_tilts_balance_factor(xs: Vec<u32>) -> TestResult {
        let mut set = xs.iter().cloned().collect::<AVLTreeSet<_>>();

        if !set.root.is_some() {
            return TestResult::discard();
        }

        let root_node = set.root.as_mut().unwrap();
        let balance_factor = root_node.balance_factor();

        if !root_node.rotate_left() {
            return TestResult::discard();
        }

        let tilted_factor = root_node.balance_factor();

        TestResult::from_bool(balance_factor - tilted_factor <= -2)
    }

    #[quickcheck]
    fn rotate_right_preserves_order(btree: BTreeSet<u8>) -> TestResult {
        let mut set = btree.iter().cloned().collect::<AVLTreeSet<_>>();

        if !set.root.is_some() {
            return TestResult::discard();
        }

        if !set.root.as_mut().unwrap().rotate_right() {
            return TestResult::discard();
        }

        TestResult::from_bool(set.iter().eq(btree.iter()))
    }

    #[quickcheck]
    fn rotate_left_preserves_order(btree: BTreeSet<u8>) -> TestResult {
        let mut set = btree.iter().cloned().collect::<AVLTreeSet<_>>();

        if !set.root.is_some() {
            return TestResult::discard();
        }

        if !set.root.as_mut().unwrap().rotate_left() {
            return TestResult::discard();
        }

        TestResult::from_bool(set.iter().eq(btree.iter()))
    }

    #[quickcheck]
    fn node_height(set: AVLTreeSet<u16>) -> bool {
        set.node_iter()
            .all(|_node| 
                _node.height == 1 + max(_node.left_height(), _node.right_height()) 
            )
    }

    #[quickcheck]
    fn node_balance(set: AVLTreeSet<u16>) -> bool {
        set.node_iter()
            .all(|_node|
                _node.balance_factor().abs() < 2
            
            )
    }

    #[quickcheck]
    fn take_balanced_nodes(xs: Vec<usize>) -> bool {
        let odds = xs
            .iter()
            .cloned()
            .filter(|x| x % 2 == 1)
            .collect::<Vec<_>>();
        let mut set = xs.iter().cloned().collect::<AVLTreeSet<_>>();

        for odd in odds {
            set.take(&odd);
        }

        let x = set.node_iter().all(|_node| 
            _node.balance_factor().abs() < 2
        );
        x
    }

    #[quickcheck]
    fn take_height_nodes(xs: Vec<isize>) -> bool {
        let negatives = xs.iter().cloned().filter(|&x| x < 0).collect::<Vec<_>>();
        let mut set = xs.iter().cloned().collect::<AVLTreeSet<_>>();

        for negative in negatives {
            set.take(&negative);
        }

        let x = set.node_iter().all( |_node| 
            _node.height == 1 + max(_node.left_height(), _node.right_height())
        );
        x
    }

    #[quickcheck]
    fn take_iterator_parity(xs: Vec<i16>) -> bool {
        let fives = xs
                            .iter()
                            .cloned()
                            .filter(|x| x % 5 == 0)
                            .collect::<Vec<_>>();
        let mut avl_set = xs.iter().cloned().collect::<AVLTreeSet<_>>();
        let mut btree_set = xs.iter().cloned().collect::<BTreeSet<_>>();

        for five in fives {
            assert_eq!(avl_set.take(&five), btree_set.take(&five));
        }

        avl_set.iter().eq(btree_set.iter())
    }

    #[quickcheck]
    fn take_parity(xs: Vec<usize>) -> bool {
        let odds = xs
                                .iter()
                                .cloned()
                                .filter(|_x| _x % 2 == 1)
                                .collect::<Vec<_>>();
        let mut avl_set = odds.iter().cloned().collect::<AVLTreeSet<_>>(); 
        let mut btree_set = odds.iter().cloned().collect::<BTreeSet<_>>();

        xs.iter().all(|_x| avl_set.take(_x) == btree_set.take(_x))
    }

    #[quickcheck]
    fn contains_parity(xs: Vec<isize>) -> bool {
        let evens = xs
                                .iter()
                                .cloned()
                                .filter(|_x| _x % 2 == 0)
                                .collect::<Vec<_>>();
        let avl_set = evens.iter().cloned().collect::<AVLTreeSet<_>>();
        let btree_set = evens.iter().cloned().collect::<BTreeSet<_>>();

        xs.iter().all(|_x| avl_set.contains(_x) == btree_set.contains(_x))
    }

    #[quickcheck]
    fn iterator_parity(xs: Vec<usize>) -> bool {
        let avl_set = xs.iter().cloned().collect::<AVLTreeSet<_>>();
        let btree_set = xs.iter().cloned().collect::<BTreeSet<_>>();

        avl_set.iter().eq(btree_set.iter())
    }

    #[quickcheck]
    fn insert_parity(mut btree_set: BTreeSet<u8>, x:u8) -> bool {
        let mut avl_set = btree_set.iter().cloned().collect::<AVLTreeSet<_>>();
        avl_set.insert(x) == btree_set.insert(x)
    }
    
    #[test]
    fn iter_insert() {
        let mut set = AVLTreeSet::new();

        for i in (1..4 as usize).rev() {
            set.insert(i);
        }

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}