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
                // set as the right node as parent
                parent.left = Box::new(Some(
                    AVLTree { left: self.left,
                              right: parent.left,
                              height: self.height,
                              value: self.value }
                ));
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
                // set as the parent
                parent.right = Box::new(Some(
                    AVLTree { left: parent.right,
                              right: self.right,
                              height: self.height,
                              value: self.value }
                ));
                return Ok(parent);
            }
        }
    }
}



fn main() {
    

    println!("Hello, world!");
}
