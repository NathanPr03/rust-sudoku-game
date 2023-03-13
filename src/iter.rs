use crate::node::WeakNode;
use crate::StrongNode;

#[derive(Debug)]
pub struct ColumnIterator {
    next_up: StrongNode,
    next_down: StrongNode,
}

#[derive(Debug)]
pub struct ColumnIteratorInclusive {
    next_up: StrongNode,
    next_down: StrongNode,
}

impl ColumnIterator {
    pub fn new(column_header: &StrongNode) -> ColumnIterator {
        ColumnIterator {
            next_up: column_header.clone(),
            next_down: column_header.clone(),
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

impl Iterator for ColumnIteratorInclusive {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next: WeakNode = self.next_down.borrow().down.clone();
        self.next_down = weak_next.upgrade().unwrap();

        if self.next_down.borrow().get_row() != self.next_up.borrow().get_row() {
            Some(weak_next.clone())
        } else if self.next_down.borrow().get_row() == self.next_up.borrow().get_row() {
            Some(self.next_down.borrow().up.clone())
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct RowIterator {
    next_left: StrongNode,
    next_right: StrongNode,
}

impl RowIterator {
    pub fn new(node: &WeakNode) -> RowIterator {
        let raw_node = node.upgrade().unwrap();
        RowIterator {
            next_left: raw_node.clone(),
            next_right: raw_node.clone(),
        }
    }
}

impl Iterator for RowIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next: WeakNode = self.next_right.borrow().right.clone();
        self.next_right = weak_next.upgrade().unwrap();

        if self.next_right.borrow().column_index != self.next_left.borrow().column_index {
            Some(weak_next.clone())
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for RowIterator {
    fn next_back(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.next_left.borrow().left.clone();
        self.next_left = weak_next.upgrade().unwrap();

        if self.next_right.borrow().column_index != self.next_left.borrow().column_index {
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
