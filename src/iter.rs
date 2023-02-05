use crate::node::WeakNode;
use crate::OwnedNode;

#[derive(Debug)]
pub struct ColumnIterator {
    next_up: OwnedNode,
    next_down: OwnedNode
}


impl ColumnIterator {
    pub fn new(c: &OwnedNode) -> ColumnIterator {
        ColumnIterator {
            next_up: c.clone(),
            next_down: c.clone()
        }
    }
}

impl Iterator for ColumnIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next: WeakNode = self.next_down.borrow().down.clone();
        self.next_down = weak_next.upgrade().unwrap();

        if self.next_down.borrow().get_row() != self.next_up.borrow().get_row() {
            Some(weak_next.clone())
        } else {
            None
        }
    }
}

// Dont know if we need this yet
impl DoubleEndedIterator for ColumnIterator {
    fn next_back(&mut self) -> Option<WeakNode> {
        let ref weak_next: WeakNode = self.next_up.borrow().up.clone();
        self.next_up = weak_next.upgrade().unwrap();

        if self.next_down.borrow().get_row() != self.next_up.borrow().get_row() {
            Some(weak_next.clone())
        } else {
            None
        }
    }
}
