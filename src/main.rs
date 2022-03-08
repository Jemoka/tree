use std::cmp::max;

//// Basics ////

// Tree rotation trait
trait Rotateable:Sized {
    fn rotate_left(self) -> Result<Self, ()>;
    fn rotate_right(self) -> Result<Self, ()>;
}

// Tree operation trait
trait Tree<T> {
    fn get_nth(&self, nth:u32) -> T;
    fn insert(&mut self); 
    fn delete(&mut self); 
}


//// AVL Tree ////

// Generic Representation of an AVL tree node
#[derive(Clone)]
struct AVLTree<T:Clone> {
    left: Box<Option<AVLTree<T>>>,
    right: Box<Option<AVLTree<T>>>,
    height: u32,
    value: T
}

// Implement Rotation
impl <T:Clone> Rotateable for AVLTree<T> {
    fn rotate_left(self) -> Result<Self, ()> {
        match *self.right {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return Err(())},

            // If it does, we rotate
            Some(mut parent) => {
                // Get the new heights of both left and right objects
                let child_right_height = match &*parent.left {
                    Some(pl) => pl.height,
                    None => 0
                };
                let child_left_height = match &*self.left {
                    Some(pl) => pl.height,
                    None => 0
                };
                // Get the child's height
                let child_height = max(child_left_height, child_right_height);
                
                // set a copy of the right node as parent
                parent.left = Box::new(Some(
                    AVLTree { left: self.left,
                              right: parent.left,
                              height: child_height,
                              value: self.value }
                ));

                // Get the height of the right child of parent
                let parent_right_height = match &*parent.right {
                    Some(pl) => pl.height,
                    None => 0
                };

                // set the parent's new height
                parent.height = max(child_height, parent_right_height);
                
                // return new parent
                return Ok(parent);
            }
        }
    }

    fn rotate_right(self) -> Result<Self, ()> {
        match *self.left {
            // If it does not exist, return.
            // we can't actually rotate
            None => {return Err(())},

            // If it does, we rotate
            Some(mut parent) => {
                // Get the new heights of both left and right objects
                let child_right_height = match &*parent.right {
                    Some(pl) => pl.height,
                    None => 0
                };
                let child_left_height = match &*self.right {
                    Some(pl) => pl.height,
                    None => 0
                };
                // Get the child's height
                let child_height = max(child_left_height, child_right_height);

                // set as the parent
                parent.right = Box::new(Some(
                    AVLTree { left: parent.right,
                              right: self.right,
                              height: child_height,
                              value: self.value }
                ));

                // Get the height of the left child of parent
                let parent_left_height = match &*parent.left {
                    Some(pl) => pl.height,
                    None => 0
                };

                // set the parent's new height
                parent.height = max(child_height, parent_left_height);
                
                return Ok(parent);
            }
        }
    }
}

fn main() {
    

    println!("Hello, world!");
}
