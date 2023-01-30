// use std::rc::Rc;
//
// pub struct Node {
//     left_link: Rc<Option<Node>>,
//     right_link: Rc<Option<Node>>,
//     up_link: Rc<Option<Node>>,
//     down_link: Rc<Option<Node>>,
//     column_header_node: ColumnHeaderNode,
// }
//
// impl Node {
//     pub fn new(column_header_node: ColumnHeaderNode) -> Node {
//         return Node {
//             left_link: Rc::new(None),
//             right_link: Rc::new(None),
//             up_link: Rc::new(None),
//             down_link: Rc::new(None),
//             column_header_node,
//         }
//     }
// }
//
// pub struct ColumnHeaderNode {
//     size: usize,
//     index: usize,
//     left_link: Rc<Option<ColumnHeaderNode>>,
//     right_link: Rc<Option<ColumnHeaderNode>>,
//     up_link: Rc<Option<ColumnHeaderNode>>,
//     down_link: Rc<Option<ColumnHeaderNode>>,
// }
//
// impl ColumnHeaderNode {
//     pub fn new(size: usize, index: usize) -> ColumnHeaderNode {
//         return ColumnHeaderNode {
//             size,
//             index,
//             left_link: Rc::new(None),
//             right_link: Rc::new(None),
//             up_link: Rc::new(None),
//             down_link: Rc::new(None),
//         }
//     }
//
//     pub fn link_right_and_left(&mut self, node: ColumnHeaderNode) -> ColumnHeaderNode
//     {
//         let mut raw_right_link = match node.right_link.as_ref() {
//             Some(node) => node,
//             None => panic!("Panic Stations"),
//         };
//
//         let mut raw_right_left_link = match raw_right_link.right_link.as_ref() {
//             Some(node) => node,
//             None => panic!("Panic Stations"),
//         };
//
//         let mut raw_self_right_link = match self.right_link.as_ref() {
//             Some(node) => node,
//             None => panic!("Panic Stations")
//         };
//
//         raw_right_link = raw_self_right_link;
//         raw_right_left_link = &node;
//
//         let mut raw_left_link = match node.left_link.as_ref() {
//             Some(node) => node,
//             None => panic!("Panic Stations"),
//         };
//
//         raw_left_link = &node;
//         raw_self_right_link = &node;
//
//         return node;
//     }
// }

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
    up: Weak<RefCell<Node>>,
    down: Weak<RefCell<Node>>,
    left: Weak<RefCell<Node>>,
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
}

/**
 * Insert node in-between root and whatever is to the right of root
 * root<->x -----> root<->node<->x
*/
pub fn link_right(root: &OwnedNode, node: &WeakNode) -> () {
    let unwrapped_node = (*node).upgrade().unwrap();

    {
        //                   ↓--------------↰
        //root<->x -----> ↓<-root<-node->x->^
        //                ↳--------------^
        let mut node_ref = unwrapped_node.borrow_mut();
        node_ref.left = Rc::downgrade(&root);
        node_ref.right = root.borrow_mut().right.clone();
    }
    {
        //   ↓--------------↰         ↓--------------↰
        //↓<-root<-node->x->^ ------> root<->node->x-^
        //↳--------------^
        let mut mutable_root = root.borrow_mut();
        mutable_root.right = (*node).clone();
    }
    {
        //↓---------------↰
        //root<->node->x->^ ------> root<->node<->x
        let node_ref = unwrapped_node.borrow_mut();
        let x_node = node_ref.right.upgrade().unwrap();
        x_node.borrow_mut().left = (*node).clone();
    }
}

pub fn prepend_up(root: &OwnedNode, node: &WeakNode) -> () {
    let u = (*node).upgrade().unwrap();

    {
        let mut n = u.borrow_mut();
        n.down = Rc::downgrade(&root);
        n.up = root.borrow_mut().up.clone();
    }
    {
        let mut head = root.borrow_mut();
        head.up = (*node).clone();
    }
    {
        let pup = u.borrow_mut();
        let prev_up  = pup.up.upgrade().unwrap();
        prev_up.borrow_mut().down = (*node).clone();
    }
}