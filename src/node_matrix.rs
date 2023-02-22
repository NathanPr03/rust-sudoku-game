use std::env::current_exe;
use std::rc::Rc;
use crate::{ColumnIterator, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};
use crate::iter::RowIterator;

use crate::node::{Node, StrongNode};

pub struct NodeMatrix {
    root_node: StrongNode,
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
        let mut solution: Vec<StrongNode> = Vec::new();
        {
            let borrowed_root = self.root_node.borrow_mut();
            if borrowed_root.right.upgrade().unwrap().borrow_mut().column_index
                == borrowed_root.column_index {
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
        NodeMatrix::cover(&column_node);

        println!("{}", count);

        for node in column_iterator {
            let mut temp_node = node.clone();
            solution.push(temp_node.upgrade().unwrap());

            let row_iterator = RowIterator::new(&node);
            for row_node in row_iterator {
                temp_node = row_node.clone();
            }

            self.solve(count + 1);

            column_node = temp_node.upgrade().unwrap().borrow_mut().header.upgrade().unwrap();

            let row_iterator_reverse = RowIterator::new(&temp_node);
            for node_from_row in row_iterator_reverse.rev() {
                NodeMatrix::uncover(&node_from_row.upgrade().unwrap());
                solution.pop();
            }

        }
        NodeMatrix::uncover(&column_node);
    }


    pub fn choose_column(&mut self) -> StrongNode
    {
        let mut lowest_count = EXACT_COVER_MATRIX_ROWS as usize;
        let mut index_of_populus_column = 0;

        for i in 0..self.column_nodes.len() {
            let node = self.column_nodes[i].clone();
            let current_nodes_count = node.borrow_mut().get_count();
            if current_nodes_count < lowest_count {
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
            borrowed_column_header.left.upgrade().unwrap().borrow_mut().right = borrowed_column_header.right.clone();
            borrowed_column_header.right.upgrade().unwrap().borrow_mut().left = borrowed_column_header.left.clone();
        }

        let column_iterator = ColumnIterator::new(&column_header);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);

            for node_from_row in row_iterator {
                let raw_node_from_row = node_from_row.upgrade().unwrap();
                raw_node_from_row.borrow_mut().remove_node_from_column();

                let column_header_of_given_node = raw_node_from_row.borrow_mut().header.clone();
                println!("prev {}", column_header_of_given_node.upgrade().unwrap().borrow_mut().get_count());
                column_header_of_given_node.upgrade().unwrap().borrow_mut().decrement_count();
                println!("after {}", column_header_of_given_node.upgrade().unwrap().borrow_mut().get_count());
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

                let column_header_of_given_node = raw_node_from_row.borrow_mut().header.clone();

                column_header_of_given_node.upgrade().unwrap().borrow_mut().increment_count();
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