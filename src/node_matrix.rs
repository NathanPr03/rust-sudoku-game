use std::rc::Rc;
use crate::{ColumnIterator, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};
use crate::iter::RowIterator;

use crate::node::{Node, StrongNode};

pub struct NodeMatrix {
    pub root_node: StrongNode,
    column_nodes: Vec<StrongNode>,
    rows: Vec<Vec<StrongNode>>
}

impl NodeMatrix {
    pub fn new() -> NodeMatrix {
        return NodeMatrix {
            root_node: Node::new_root(),
            column_nodes: Vec::new(),
            rows: Vec::new()
        }
    }

    pub fn get_column_nodes(&self) -> &Vec<StrongNode>
    {
        return &self.column_nodes;
    }

    pub fn get_rows(&self) -> &Vec<Vec<StrongNode>>
    {
        return &self.rows;
    }

    pub fn arrange_matrix(
        &mut self,
        cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
    ) -> ()
    {
        let mut column_nodes: Vec<StrongNode> = Vec::new();

        for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
            let column_header: StrongNode = Node::new_header(Some(column_index as usize));
            column_header.borrow_mut().link_left(&self.root_node);
            column_nodes.push(column_header);
        }

        let mut all_rows: Vec<Vec<StrongNode>> = Vec::new();

        for row_index in 0..EXACT_COVER_MATRIX_ROWS {
            let mut a_row: Vec<StrongNode> = Vec::new();
            for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
                if cover_matrix[row_index as usize][column_index as usize] == 1 {
                    let header_node: &StrongNode = &(column_nodes[column_index as usize]);

                    let node: StrongNode = Node::new_inner(header_node, row_index as usize);

                    node.borrow_mut().link_down();

                    header_node.borrow_mut().increment_count();

                    a_row.push(node);
                }
            }

            all_rows.push(a_row);
        }

        for row in &all_rows.clone() {
            let length_of_vec: usize = row.len();
            for i in 0..length_of_vec {
                let previous_node = &row[get_previous_index(i, length_of_vec)];
                let next_node = &row[get_next_index(i, length_of_vec)];

                row[i].borrow_mut().left = Rc::downgrade(previous_node);
                row[i].borrow_mut().right = Rc::downgrade(next_node);
            }
        }

        self.rows = all_rows;
        self.column_nodes = column_nodes;
    }

    pub fn solve(&mut self, count: u32) {
        println!("Count is: {}", count);
        let mut solution: Vec<StrongNode> = Vec::new();
        {
            let borrowed_root = &self.root_node.borrow_mut();

            dbg!(&borrowed_root.right.upgrade().unwrap().borrow_mut().column_index);
            dbg!(&borrowed_root.right.upgrade().unwrap().borrow_mut().extra);
            dbg!(&borrowed_root.extra);

            let breakpoint = 2;

            if borrowed_root.right.upgrade().unwrap().borrow_mut().extra == borrowed_root.extra {
                //Print solution
                let hi = 2;
                println!("FOUND!");
                return;
            }
        }

        let mut column_node = self.choose_column();
        {
            dbg!(column_node.clone());
        }
        let column_iterator = ColumnIterator::new(&column_node);

        for mut node in column_iterator {
            solution.push(node.upgrade().unwrap());

            let row_iterator = RowIterator::new(&node);
            for row_node in row_iterator {
                dbg!(row_node.upgrade().unwrap());
                let header = row_node.upgrade().unwrap().borrow_mut().header.upgrade().unwrap();
                dbg!(header.clone());
                NodeMatrix::cover(&header);
            }
            let borrowed_root = self.root_node.borrow_mut().right.upgrade().unwrap().borrow_mut().column_index.unwrap();
            if borrowed_root == 247 {
                dbg!(self.root_node.borrow_mut().right.upgrade().unwrap().borrow_mut().column_index.unwrap());
                dbg!(self.root_node.borrow_mut().right.upgrade().unwrap().borrow_mut().right.upgrade().unwrap().borrow_mut().column_index.unwrap());
                let breake = 2;
            }
            self.solve(count + 1);

            node = Rc::downgrade(&solution[count as usize]);
            column_node = node.upgrade().unwrap().borrow_mut().header.clone().upgrade().unwrap();

            let row_iterator_reverse = RowIterator::new(&node);
            for new_node in row_iterator_reverse.rev() {
                let header = &new_node.upgrade().unwrap().borrow_mut().header.upgrade().unwrap();
                NodeMatrix::uncover(header);
            }
        }

        NodeMatrix::uncover(&column_node);

        dbg!(solution);
        return;
    }


    pub fn choose_column(&mut self) -> StrongNode
    {
        let mut lowest_count = EXACT_COVER_MATRIX_ROWS as usize;
        let mut index_of_populus_column = 0;

        for i in 0..self.column_nodes.len() {
            let node = self.column_nodes[i].clone();
            let current_nodes_count = node.borrow_mut().get_count();
            if current_nodes_count < lowest_count && current_nodes_count > 0 {
                lowest_count = current_nodes_count;
                index_of_populus_column = i;
            }
        }

        return self.column_nodes[index_of_populus_column].clone();
    }

    pub fn cover(column_header: &StrongNode) -> ()
    {
        {
            let borrowed_column_header = column_header.borrow_mut();
            {
                dbg!(&borrowed_column_header);
            }
            borrowed_column_header.left.upgrade().unwrap().borrow_mut().right = borrowed_column_header.right.clone();
            borrowed_column_header.right.upgrade().unwrap().borrow_mut().left = borrowed_column_header.left.clone();
        }

        let column_iterator = ColumnIterator::new(&column_header);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);

            for node_from_row in row_iterator {
                let raw_node_from_row = node_from_row.upgrade().unwrap();

                let header = raw_node_from_row.borrow_mut().header.upgrade().unwrap();
                if raw_node_from_row.borrow_mut().column_index.unwrap() == 247
                {
                    let bpoint = 3;
                }

                raw_node_from_row.borrow_mut().remove_node_from_column();
            }
        }
    }

    pub fn uncover(column_header: &StrongNode) -> ()
    {
        let column_iterator = ColumnIterator::new(&column_header);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);

            for node_from_row in row_iterator {
                let raw_node_from_row = node_from_row.upgrade().unwrap();

                raw_node_from_row.borrow_mut().reinsert_node_into_column();
            }
        }
        column_header.borrow_mut().left.upgrade().unwrap().borrow_mut().right = Rc::downgrade(column_header);
        column_header.borrow_mut().right.upgrade().unwrap().borrow_mut().left = Rc::downgrade(column_header);
    }
}

// This will return a modular index, giving us circular links
fn get_previous_index(current_index: usize, length: usize) -> usize {
    return if current_index == 0 {
        // 0 indexed
        length - 1
    } else {
        current_index - 1
    };
}

// This will return a modular index, giving us circular links
fn get_next_index(current_index: usize, length: usize) -> usize {
    return if current_index == length - 1 {
        0
    } else {
        current_index + 1
    }
}