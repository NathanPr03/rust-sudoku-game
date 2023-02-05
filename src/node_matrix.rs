use std::rc::Rc;
use crate::{EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};

use crate::node::{Node, StrongNode};

pub struct NodeMatrix {
    column_nodes: Vec<StrongNode>,
    rows: Vec<Vec<StrongNode>> // Dont really want this property but nodes will be dropped if we dont have it :(
}

impl NodeMatrix {
    pub fn new() -> NodeMatrix {
        return NodeMatrix {
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
        let special_header: StrongNode = Node::new_root();

        let mut column_nodes: Vec<StrongNode> = Vec::new();

        for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
            let column_header: StrongNode = Node::new_header(Some(column_index as usize));
            column_header.borrow_mut().link_left(&special_header, &Rc::downgrade(&column_header));
            column_nodes.push(column_header);
        }

        let mut all_rows: Vec<Vec<StrongNode>> = Vec::new();

        for row_index in 0..EXACT_COVER_MATRIX_ROWS {
            let mut a_row: Vec<StrongNode> = Vec::new();
            for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
                if cover_matrix[row_index as usize][column_index as usize] == 1 {
                    let header_node: &StrongNode = &(column_nodes[column_index as usize]);

                    let node: StrongNode = Node::new_inner(header_node, row_index as usize);

                    node.borrow_mut().link_down(&Rc::downgrade(&node));

                    header_node.borrow_mut().inc_count();

                    a_row.push(node);
                }
            }

            all_rows.push(a_row);
        }

        for row in all_rows.clone() {
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
}

// This will return a module index, giving us circular links
fn get_previous_index(current_index: usize, length: usize) -> usize {
    return if current_index == 0 {
        // 0 indexed
        length - 1
    } else {
        current_index
    };
}

// This will return a module index, giving us circular links
fn get_next_index(current_index: usize, length: usize) -> usize {
    return if current_index == length {
        0
    } else {
        current_index
    }
}