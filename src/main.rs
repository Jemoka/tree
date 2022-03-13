// Debug facilities
use derivative::Derivative;

// Shared pointer facilities
use std::rc::Rc;
use std::cell::RefMut;
use std::cell::RefCell;

// Partial equivalence failities
use std::cmp::max;
use std::cmp::PartialOrd;

//// AVL Tree ////
// The User-Facing AVL Tree. It actually just an wrapper that manipulates an AVLTreeArena. 
#[derive(Debug)]
pub struct AVLTree<'a, T:Clone+PartialOrd+std::fmt::Debug> {
    arena: &'a mut AVLTreeArena<T>
}

impl<'a, T:Clone+PartialOrd+std::fmt::Debug> AVLTree<'a, T> {
    // Creates a new AVL tree with the info
    pub fn new(val:T) -> Self {
        // Get a rc reference of the arena
        let arena_ref = AVLTreeArena::<T>::new(val);
        // Get the raw pointer to the arena
        let arena_pointer_raw = arena_ref.as_ptr();

        unsafe {
            AVLTree { arena: &mut *arena_pointer_raw }
        }
    }

    // Get the value given an index
    pub fn take(&self, n:usize) -> Option<Vec<T>> { self.arena.take(n) }

    // Insert a value
    pub fn insert(&mut self, val:T) -> Option<AVLTreeNode<T>> { self.arena.insert(val) }

    // Get the size of the tree
    pub fn size(&self) -> usize { self.arena.size() }
}

// AVLTreeArena: used to store pointer objects, memory manegement facilities, etc.
// also is where most of the logic goes in this setup.
// Nodes have Counted References of Mutable Reference Cells of this store shared across
// all of them to maintain conchordiance.
#[derive(Debug, Clone)]
struct AVLTreeArena<T:Clone+PartialOrd+std::fmt::Debug> {
    store: Vec<AVLTreeNode<T>>
}

impl<'a, T:Clone+PartialOrd+std::fmt::Debug> AVLTreeArena<T> {
    pub fn new(val:T) -> Rc<RefCell<AVLTreeArena<T>>> {
        // Create a counted reference of our newly minted tree object
        let tree_rc = Rc::new(RefCell::new(AVLTreeArena { store: vec![] }));

        // Create the root node, and clone the Rc pointer into the object
        let root_node =
            AVLTreeNode {
                left:None,
                right:None,
                parent:None,
                index: 0,
                height: 0,
                value: val.clone(),
                container: tree_rc.clone()
            };

        // Borrow the pointer as mutable and push our root node
        tree_rc.borrow_mut().store.push(root_node);

        // Return the actual tree
        return tree_rc;
    }

    // Get the value given an index
    pub fn take(&self, n:usize) -> Option<Vec<T>> {
        if n > self.store.len() { return None };

        // Create the visited array
        let mut visited = vec![false; self.store.len()];
        let mut results:Vec<T> = vec![];

        // Create a stack and keep track of the current node
        let root = self.root();
        let mut stack:Vec<usize> = vec![root];

        // Set the current node to the root node
        let mut current;

        // Get the previous height of the stack
        let mut prev_height = 0;

        // DFS!
        while results.len() < n && stack.len() > 0 {

            current = stack.pop().unwrap();
            let height = stack.len();

            // Track and increment number of visited
            // if popping results in a shorter stack
            // we are tracking it as we are backing up
            if height < prev_height {
                results.push(self.store[current].value.clone());
            }
            prev_height = height;

            visited[current] = true;

            // Push the left to be visited unto the stack if not visited
            if let Some(l) = self.store[current].left {
                if !visited[l] {
                    stack.push(l);
                }
            }

            // Push the right to be visited unto the stack if not visited
            if let Some(l) = self.store[current].right {
                if !visited[l] {
                    stack.push(l);
                }
            }
        }

        return Some(results);
    }

    // Finds the global root index
    pub fn root(&self) -> usize {
        let mut canidate = &self.store[0];

        while canidate.parent != None {
            canidate = &self.store[canidate.parent.unwrap()];
        }

        return canidate.index;
    }

    // Finds the length of the store
    pub fn size(&self) -> usize {
        return self.store.len();
    }

    // Insert
    // Get the value given an index
    pub fn insert(&mut self, val:T) -> Option<AVLTreeNode<T>> {
        // Create the visited array
        let mut visited = vec![false; self.store.len()];

        // Create a stack and keep track of the current node
        let root = self.root();
        let mut stack:Vec<usize> = vec![root];
        let mut current;

        // get the new index (length of existing)
        let new_index = self.store.len();

        // DFS!
        // we break explicitly when the adding is done
        loop {
            // Track and increment number of visited
            current = stack.pop().unwrap();
            visited[current] = true;

            // Check whether to check left or right node
            if self.store[current].value <= val {
                // check right and append if exists
                // if not, insert and we did it!
                if let Some(l) = self.store[current].right {
                    if !visited[l] {stack.push(l);}
                } else {
                    self.store.push(AVLTreeNode {
                        left:None,
                        right:None,
                        parent:Some(current),
                        index: new_index,
                        height: 1,
                        value: val.clone(),
                        container: self.store[current].container.clone()
                    });
                    self.store[current].right = Some(new_index);
                    break;
                }
            } else {
                // check left and append if exists
                // if not, insert and we did it!
                if let Some(l) = self.store[current].left {
                    if !visited[l] {stack.push(l);}
                } else {
                    self.store.push(AVLTreeNode {
                        left:None,
                        right:None,
                        parent:Some(current),
                        index: new_index,
                        height: 1,
                        value: val.clone(),
                        container: self.store[current].container.clone()
                    });
                    self.store[current].left = Some(new_index);
                    break;
                }

            }
        }

        // Re-calculate height upwards as well as rotate as needed
        let mut current = new_index;

        // as long as our current node has a parent
        // we process its parent
        while let Some(p) = self.store[current].parent {
            current = p;

            // we get left and right heights
            let left_height = match self.store[current].left { Some(i) => (self.store[i]).height,
                                                                None => 0 };
            let right_height = match self.store[current].right { Some(i) => (self.store[i]).height,
                                                                    None => 0 };

            // we update its height
            let new_height = max(left_height, right_height)+1;
            self.store[current].height = new_height;

            // we now perform rotations as needed
            if left_height > right_height && left_height-right_height > 1 {
                // Need to rotate right!
                self.store[current].rotate_right();
            } else if left_height < right_height && right_height-left_height > 1 {
                // Need to rotate left!
                self.store[current].rotate_left();
            }

        }


        dbg!(self.store.clone());

        // we can safely .clone() the added node here as it only countains
        // indexes and a pointer to things, which is not that bad
        return Some(self.store[current].clone());
    }
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct AVLTreeNode<T:Clone+PartialOrd+std::fmt::Debug> {
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,

    index: usize,

    height: u32,
    pub value: T,

    #[derivative(Debug="ignore")]
    container: Rc<RefCell<AVLTreeArena<T>>>
}

impl<T:Clone+PartialOrd+std::fmt::Debug> AVLTreeNode<T> {

    // Left rotation
    pub fn rotate_left(&mut self) {
        // create a borrow who has a nonmutable view of the
        // store inside. This is to appease the borrow checker
        // and race-condition-de-possibleifier of Rust
        let mut container_tree:RefMut<AVLTreeArena<T>> = self.container.borrow_mut();
        let store = &mut container_tree.store;

        match self.right {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return ()},

            // If it does, we rotate
            Some(parent_index) => {
                // Get the parent node
                let parent = &store[parent_index];

                // Get the new heights of both left and right objects
                let child_right_height = match parent.left {
                    Some(pl) => (&store[pl]).height,
                    None => 0
                };
                let child_left_height = match self.left {
                    Some(pl) => (&store[pl]).height,
                    None => 0
                };
                // Get the child's height and parent
                let child_height = max(child_left_height, child_right_height);
                let old_parent_index = self.parent;

                // slide the left element of new parent to the right of self
                self.right = parent.left;

                // update the height
                self.height = child_height+1;

                // update the parent ID of current node
                self.parent = Some(parent.index);

                // Get the height of the right child of parent
                let parent_right_height = match parent.right {
                    Some(pl) => store[pl].height,
                    None => 0
                };

                // We finally set all the values. We need to explicitly
                // index here because the previous parent borrow is
                // immutable

                // set the left node of the parent to be self
                store[parent_index].left  = Some(self.index);

                // set the parent's new height
                store[parent_index].height = max(child_height, parent_right_height)+1;

                // set the parent's parent
                store[parent_index].parent = old_parent_index;
            }
        }

    }

    // Right rotation
    pub fn rotate_right(&mut self) {
        // create a borrow who has a nonmutable view of the
        // store inside. This is to appease the borrow checker
        // and race-condition-de-possibleifier of Rust
        let mut container_tree:RefMut<AVLTreeArena<T>> = self.container.borrow_mut();
        let store = &mut container_tree.store;

        match self.left {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return ()},

            // If it does, we rotate
            Some(parent_index) => {
                // Get the parent node
                let parent = &store[parent_index];

                // Get the new heights of both left and right objects
                let child_right_height = match parent.right {
                    Some(pl) => (&store[pl]).height,
                    None => 0
                };
                let child_left_height = match self.right {
                    Some(pl) => (&store[pl]).height,
                    None => 0
                };
                // Get the child's height and parent
                let child_height = max(child_left_height, child_right_height);
                let old_parent_index = self.parent;

                // slide the left element of new parent to the right of self
                self.left = parent.right;

                // update the height
                self.height = child_height+1;

                // update the parent ID of current node
                self.parent = Some(parent.index);

                // Get the height of the right child of parent
                let parent_left_height = match parent.left {
                    Some(pl) => store[pl].height,
                    None => 0
                };

                // We finally set all the values. We need to explicitly
                // index here because the previous parent borrow is
                // immutable

                // set the left node of the parent to be self
                store[parent_index].right = Some(self.index);

                // set the parent's new height
                store[parent_index].height = max(child_height, parent_left_height)+1;

                // set the parent's parent
                store[parent_index].parent = old_parent_index;
            }
        }

    }
}

fn main() {
    let mut test = AVLTree::<u32>::new(0);
    test.insert(1);
    test.insert(1);
    test.insert(2);

    dbg!(test.take(test.size()));
}
