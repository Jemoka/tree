use std::cmp::max;

//// AVL Tree ////
// Generic Representation of an AVL tree node

pub struct AVLTree<'a, T:Clone> {
    arena: Vec<AVLTreeNode<'a, T>>
}

impl<'a, T:Clone> AVLTree<'a, T> {
    pub fn new() -> AVLTree<'a, T>{
        AVLTree { arena: vec![] }
    }
}

pub struct AVLTreeNode<'a, T:Clone> {
    pub left: Option<usize>,
    pub right: Option<usize>,
    parent: Option<usize>,

    index: usize,

    height: u32,
    pub value: T,

    container: &'a mut AVLTree<'a, T>
}

impl<'a, T:Clone> AVLTreeNode<'a, T> {

    // Left rotation
    pub fn rotate_left(&mut self) {
        // create a borrow who has a nonmutable view of the
        // arena inside. This is to appease the borrow checker
        // and race-condition-de-possibleifier of Rust
        let arena = &mut self.container.arena;

        match self.right {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return ()},

            // If it does, we rotate
            Some(parent_index) => {
                // Get the parent node
                let parent = &arena[parent_index];

                // Get the new heights of both left and right objects
                let child_right_height = match parent.left {
                    Some(pl) => (&arena[pl]).height,
                    None => 0
                };
                let child_left_height = match self.left {
                    Some(pl) => (&arena[pl]).height,
                    None => 0
                };
                // Get the child's height and parent
                let child_height = max(child_left_height, child_right_height);
                let old_parent_index = self.parent;

                // slide the left element of new parent to the right of self
                self.right = parent.left;

                // update the height
                self.height = child_height;

                // update the parent ID of current node
                self.parent = Some(parent.index);

                // Get the height of the right child of parent
                let parent_right_height = match parent.right {
                    Some(pl) => arena[pl].height,
                    None => 0
                };

                // We finally set all the values. We need to explicitly
                // index here because the previous parent borrow is
                // immutable

                // set the left node of the parent to be self
                arena[parent_index].left  = Some(self.index);

                // set the parent's new height
                arena[parent_index].height = max(child_height, parent_right_height);

                // set the parent's parent
                arena[parent_index].parent = old_parent_index;
            }
        }

    }

    // Right rotation
    pub fn rotate_right(&mut self) {
        // create a borrow who has a nonmutable view of the
        // arena inside. This is to appease the borrow checker
        // and race-condition-de-possibleifier of Rust
        let arena = &mut self.container.arena;

        match self.left {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return ()},

            // If it does, we rotate
            Some(parent_index) => {
                // Get the parent node
                let parent = &arena[parent_index];

                // Get the new heights of both left and right objects
                let child_right_height = match parent.right {
                    Some(pl) => (&arena[pl]).height,
                    None => 0
                };
                let child_left_height = match self.right {
                    Some(pl) => (&arena[pl]).height,
                    None => 0
                };
                // Get the child's height and parent
                let child_height = max(child_left_height, child_right_height);
                let old_parent_index = self.parent;

                // slide the left element of new parent to the right of self
                self.left = parent.right;

                // update the height
                self.height = child_height;

                // update the parent ID of current node
                self.parent = Some(parent.index);

                // Get the height of the right child of parent
                let parent_left_height = match parent.left {
                    Some(pl) => arena[pl].height,
                    None => 0
                };

                // We finally set all the values. We need to explicitly
                // index here because the previous parent borrow is
                // immutable

                // set the left node of the parent to be self
                arena[parent_index].right = Some(self.index);

                // set the parent's new height
                arena[parent_index].height = max(child_height, parent_left_height);

                // set the parent's parent
                arena[parent_index].parent = old_parent_index;
            }
        }

    }
}

fn main() {
    let test = AVLTree::<u32>::new();
}
