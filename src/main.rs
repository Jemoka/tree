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

    // Get the value given an index
    pub fn nth(&self, n:usize) -> Option<T> {
        if n >= self.arena.len() { return None };

        // Create the visited array
        let mut visited = vec![false; self.arena.len()];
        let mut num_visited = 0usize;

        // Create a stack and keep track of the current node
        let mut stack:Vec<usize> = vec![0];
        let mut current = 0;

        // DFS!
        while num_visited < n {
            // Track and increment number of visited
            num_visited += 1;
            current = stack.pop().unwrap();
            visited[current] = true;

            // Push the left to be visited unto the stack if not visited
            if let Some(l) = self.arena[current].left {
                if !visited[l] {
                    stack.push(l);
                }
            }

            // Push the right to be visited unto the stack if not visited
            if let Some(l) = self.arena[current].right {
                if !visited[l] {
                    stack.push(l);
                }
            }
        }

        return Some(self.arena[current].value.clone());
    }
    

    // Finds the global root index
    pub fn root(&self) -> usize {
        let mut canidate = &self.arena[0];

        while canidate.parent != None {
            canidate = &self.arena[canidate.parent.unwrap()];
        }

        return canidate.index;
    }

    // Insert
}

pub struct AVLTreeNode<'a, T:Clone> {
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,

    pub index: usize,

    pub height: u32,
    pub value: T,

    pub container: &'a mut AVLTree<'a, T>
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
    // let mut test = AVLTree::<u32>::new();
}
