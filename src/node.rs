use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum NodeExtra {
    RowIndex(usize), // The node is an inner node, representing part of an action.
    Count(usize),  // The node is a header for a constraints.
    Root           // Root node.
}

#[derive(Debug)]
pub struct Node {
    pub up: Weak<RefCell<Node>>,
    pub down: Weak<RefCell<Node>>,
    pub left: Weak<RefCell<Node>>,
    pub right: Weak<RefCell<Node>>,
    at_self: Weak<RefCell<Node>>, //Probably dont need this

    pub column_index: Option<usize>,
    header: Weak<RefCell<Node>>,
    extra: NodeExtra
}

pub type StrongNode = Rc<RefCell<Node>>;
pub type WeakNode = Weak<RefCell<Node>>;

impl Node {
    pub fn new_header(col: Option<usize>) -> StrongNode {
        Self::new(col, None, NodeExtra::Count(0))
    }

    pub fn new_inner(header: &StrongNode, row: usize) -> StrongNode {
        Self::new(header.borrow().column_index, Some(&Rc::downgrade(&header)), NodeExtra::RowIndex(row))
    }

    pub fn new_root() -> StrongNode {
        Self::new(None, None, NodeExtra::Root)
    }

    fn new(col: Option<usize>, header: Option<&WeakNode>, e: NodeExtra) -> StrongNode {
        let an_owned_node = Rc::new(RefCell::new(Node {
            up: Weak::new(), down: Weak::new(),
            left: Weak::new(), right: Weak::new(),
            at_self: Weak::new(),
            header: Weak::new(),
            column_index: col, extra: e
        }));

        {
            let mut mutable_reference_to_owned_node = (*an_owned_node).borrow_mut();
            let a_weak_node = Rc::downgrade(&an_owned_node);

            mutable_reference_to_owned_node.up = a_weak_node.clone();
            mutable_reference_to_owned_node.down = a_weak_node.clone();
            mutable_reference_to_owned_node.left = a_weak_node.clone();
            mutable_reference_to_owned_node.right = a_weak_node.clone();
            mutable_reference_to_owned_node.at_self = a_weak_node.clone();

            mutable_reference_to_owned_node.header = match header {
                Some(node) => node.clone(),
                None => a_weak_node.clone()
            }
        }

        return an_owned_node
    }

    pub fn inc_count(&mut self) {
        let c = match self.extra {
            NodeExtra::Count(i) => i,
            _ => return
        };
        self.extra = NodeExtra::Count(c+1);
    }

    pub fn get_count(&self) -> usize {
        let count = match self.extra {
            NodeExtra::Count(i) => i,
            _ => return 999999999
        };

        return count;
    }

    pub fn get_row(&self) -> Option<usize> {
        match self.extra {
            NodeExtra::RowIndex(i) => Some(i),
            _ => None
        }
    }

    pub fn pretty_print(&self) {
        let node = self;

        println!("Node {{ up: {:?}, down: {:?}, left: {:?}, right: {:?}, at_self: {:?}, column: {:?}, header: {:?}, extra: {:?} }}",
                 node.up.upgrade().unwrap(), node.down.upgrade().unwrap(), node.left.upgrade(), node.right.upgrade(), node.at_self.upgrade(),
                 node.column_index, node.header.upgrade(), node.extra)
    }

    /**
     * Insert node prepended to the left of root
     * This will result in root being the last node in the link
     * root<->x -----> x<->node<->root
     */
    pub fn link_left(&mut self, root: &StrongNode, node: &WeakNode) -> () {
        let node_ref = self;
        {
            node_ref.right = Rc::downgrade(&root);
            node_ref.left = root.borrow_mut().left.clone();
        }
        {
            let mut mutable_root = root.borrow_mut();
            mutable_root.left = (*node).clone();
        }
        {

            let x_node = node_ref.left.upgrade().unwrap();
            x_node.borrow_mut().right = (*node).clone();
        }
    }

    pub fn link_down(&mut self, node: &WeakNode) -> () {
        let root: &StrongNode = &self.header.upgrade().unwrap();
        let node_ref = self;

        {
            node_ref.down = Rc::downgrade(&root);
            node_ref.up = root.borrow_mut().up.clone();
        }
        {
            let mut root_ref = root.borrow_mut();
            root_ref.up = (*node).clone();
        }
        {
            let x_node = node_ref.up.upgrade().unwrap();
            x_node.borrow_mut().down = (*node).clone();
        }
    }

}

impl Drop for Node{
    fn drop(&mut self) {
        let row = match self.extra {
            NodeExtra::RowIndex(i) => i,
            _ => return,
        };

        println!("We have a dropper!");
        println!("Row: {}", row);
        println!("Column: {}", self.column_index.unwrap());
    }
}

