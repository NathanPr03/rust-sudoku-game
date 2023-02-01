use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type ColumnIndex = usize;
pub type RowIndex = usize;

#[derive(Debug)]
pub enum NodeExtra {
    Row(RowIndex), // The node is an inner node, representing part of an action.
    Count(usize),  // The node is a header for a constraints.
    Root           // Root node.
}

#[derive(Debug)]
pub struct Node {
    pub up: Weak<RefCell<Node>>,
    down: Weak<RefCell<Node>>,
    pub left: Weak<RefCell<Node>>,
    right: Weak<RefCell<Node>>,
    at_self: Weak<RefCell<Node>>,

    pub column: Option<ColumnIndex>,
    header: Weak<RefCell<Node>>,
    extra: NodeExtra
}


pub type OwnedNode = Rc<RefCell<Node>>;
pub type WeakNode = Weak<RefCell<Node>>;

impl Node {
    pub fn new_header(col: Option<usize>) -> OwnedNode {
        Self::new(col, None, NodeExtra::Count(0))
    }

    pub fn new_inner(header: &OwnedNode, row: usize) -> OwnedNode {
        Self::new(header.borrow().column, Some(&Rc::downgrade(&header)), NodeExtra::Row(row))
    }

    pub fn new_root() -> OwnedNode {
        Self::new(None, None, NodeExtra::Root)
    }

    fn new(col: Option<usize>, header: Option<&WeakNode>, e: NodeExtra) -> OwnedNode {
        let an_owned_node = Rc::new(RefCell::new(Node {
            up: Weak::new(), down: Weak::new(),
            left: Weak::new(), right: Weak::new(),
            at_self: Weak::new(),
            header: Weak::new(),
            column: col, extra: e
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
}

/**
 * Insert node prepended to the left of root
 * This will result in root being the last node in the link
 * root<->x -----> x<->node<->root
*/
pub fn link_left(root: &OwnedNode, node: &WeakNode) -> () {
    let unwrapped_node = (*node).upgrade().unwrap();
    {
        //                   ↓--------------↰
        //root<->x -----> ↓<-root<-node->x->^
        //                ↳--------------^
        let mut node_ref = unwrapped_node.borrow_mut();
        node_ref.right = Rc::downgrade(&root);
        node_ref.left = root.borrow_mut().left.clone();
    }
    {
        //   ↓--------------↰         ↓--------------↰
        //↓<-root<-node->x->^ ------> root<->node->x-^
        //↳--------------^
        let mut mutable_root = root.borrow_mut();
        mutable_root.left = (*node).clone();
    }
    {
        //↓---------------↰
        //root<->node->x->^ ------> root<->node<->x
        let node_ref = unwrapped_node.borrow_mut();
        let x_node = node_ref.left.upgrade().unwrap();
        x_node.borrow_mut().right = (*node).clone();
    }
}

pub fn link_down(root: &OwnedNode, node: &WeakNode) -> () {
    dbg!(root.borrow_mut().up.upgrade());

    let unwrapped_node = (*node).upgrade().unwrap();
    {
        let mut node_ref = unwrapped_node.borrow_mut();
        node_ref.down = Rc::downgrade(&root);
        node_ref.up = root.borrow_mut().up.clone();
    }
    {
        let mut root_ref = root.borrow_mut();
        root_ref.up = (*node).clone();
    }
    {
        let node_ref = unwrapped_node.borrow_mut();
        let x_node = node_ref.up.upgrade().unwrap();
        x_node.borrow_mut().down = (*node).clone();
    }
}