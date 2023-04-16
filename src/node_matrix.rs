use crate::iter::RowIterator;
use crate::{
    four_by_four_cover_matrix, ColumnIterator,
};
use std::rc::Rc;

use crate::node::{Node, StrongNode};

pub struct NodeMatrix {
    pub root_node: StrongNode,
    column_nodes: Vec<StrongNode>,
    rows: Vec<Vec<StrongNode>>,
    pub actual_solution: Vec<StrongNode>,
    potential_solution: Vec<StrongNode>,
    solution_found: bool,
    eighty_recursion_times: usize,
}

impl NodeMatrix {
    pub fn new() -> NodeMatrix {
        return NodeMatrix {
            root_node: Node::new_root(),
            column_nodes: Vec::new(),
            rows: Vec::new(),
            actual_solution: Vec::new(),
            potential_solution: Vec::new(),
            solution_found: false,
            eighty_recursion_times: 0
        };
    }

    pub fn get_column_nodes(&self) -> &Vec<StrongNode> {
        return &self.column_nodes;
    }

    pub fn get_rows(&self) -> &Vec<Vec<StrongNode>> {
        return &self.rows;
    }

    pub fn arrange_matrix(
        &mut self,
        cover_matrix: &Vec<Vec<usize>>,
    ) -> () {
        let cover_matrix_rows = cover_matrix.len();
        let cover_matrix_columns = cover_matrix[0].len();

        let mut column_nodes: Vec<StrongNode> = Vec::new();

        for column_index in 0..cover_matrix_columns {
            let column_header: StrongNode = Node::new_header(Some(column_index as usize));
            column_header.borrow_mut().link_left(&self.root_node);
            column_nodes.push(column_header);
        }

        let mut all_rows: Vec<Vec<StrongNode>> = Vec::new();

        for row_index in 0..cover_matrix_rows {
            let mut a_row: Vec<StrongNode> = Vec::new();
            for column_index in 0..cover_matrix_columns {
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

    pub fn search(&mut self, k: u32, cover_matrix_rows: usize) {
        if self.solution_found {
            return;
        }
        // println!("Count is: {}", k);
        {
            let borrowed_root = &mut self.root_node.borrow();

            if borrowed_root.right.upgrade().unwrap().borrow().extra == borrowed_root.extra {
                self.actual_solution = self.potential_solution.clone();
                self.solution_found = true;
                return;
            }
        }

        let column_node = self.choose_column(cover_matrix_rows);

        NodeMatrix::cover(&column_node);
        let column_iterator = ColumnIterator::new(&column_node);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);
            self.potential_solution.push(node.upgrade().unwrap());

            for row_node in row_iterator {
                let header = row_node
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .header
                    .upgrade()
                    .unwrap();
                NodeMatrix::cover(&header);
            }
            self.search(k + 1, cover_matrix_rows);

            self.potential_solution.pop();

            let row_iterator_reverse = RowIterator::new(&node);

            for new_node in row_iterator_reverse.rev() {
                let header = &new_node
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .header
                    .upgrade()
                    .unwrap();

                NodeMatrix::uncover(header);
            }
        }

        NodeMatrix::uncover(&column_node);
        {
            // TODO: This shit is a horrible hack, hope I dont need it
            if k >= 80
            {
                self.eighty_recursion_times += 1;
            }

            if self.eighty_recursion_times > 2
            {
                self.solution_found = true;
            }
        }

        return;
    }

    fn print_matrix_at_given_point(&self) {
        // let mut cover_matrix = [
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0]
        // ];

        let mut cover_matrix = four_by_four_cover_matrix();
        let row_iterator = RowIterator::new(&Rc::downgrade(&self.root_node));

        for column_node in row_iterator {
            // dbg!(column_node.upgrade().unwrap());
            let column_iterator = ColumnIterator::new(&column_node.upgrade().unwrap());
            for node in column_iterator {
                let upgraded_node = node.upgrade().unwrap();
                let column = upgraded_node.borrow_mut().column_index.unwrap();
                let row = upgraded_node.borrow_mut().get_row().unwrap();

                cover_matrix[row][column] = 1;
            }
        }

        for i in 0..cover_matrix.len() {
            for j in 0..cover_matrix[1].len() {
                print!("|{}", cover_matrix[i][j]);
            }
            print!("|");
            println!();
        }

        println!("--------------");
    }

    pub fn choose_column(&mut self, cover_matrix_rows: usize) -> StrongNode {

        let mut lowest_count = cover_matrix_rows;

        let downgraded_root = Rc::downgrade(&self.root_node);
        let row_iterator = RowIterator::new(&downgraded_root);

        let mut column_with_least_nodes = downgraded_root;

        for node in row_iterator {
            let current_nodes_count = node.upgrade().unwrap().borrow_mut().get_count();
            if current_nodes_count < lowest_count && current_nodes_count > 0 {
                lowest_count = current_nodes_count;
                column_with_least_nodes = node;
            }
        }

        return column_with_least_nodes.upgrade().unwrap();
    }

    pub fn cover(column_header: &StrongNode) -> () {
        {
            let borrowed_column_header = column_header.borrow_mut();

            borrowed_column_header
                .left
                .upgrade()
                .unwrap()
                .borrow_mut()
                .right = borrowed_column_header.right.clone();
            borrowed_column_header
                .right
                .upgrade()
                .unwrap()
                .borrow_mut()
                .left = borrowed_column_header.left.clone();
        }

        let column_iterator = ColumnIterator::new(&column_header);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);

            for node_from_row in row_iterator {
                let raw_node_from_row = node_from_row.upgrade().unwrap();

                raw_node_from_row.borrow_mut().remove_node_from_column();
            }
        }
    }

    pub fn uncover(column_header: &StrongNode) -> () {
        let column_iterator = ColumnIterator::new(&column_header);

        for node in column_iterator {
            let row_iterator = RowIterator::new(&node);

            for node_from_row in row_iterator {
                let raw_node_from_row = node_from_row.upgrade().unwrap();

                raw_node_from_row.borrow_mut().reinsert_node_into_column();
            }
        }
        // dbg!(column_header.borrow_mut().left.upgrade().unwrap());
        column_header
            .borrow_mut()
            .left
            .upgrade()
            .unwrap()
            .borrow_mut()
            .right = Rc::downgrade(column_header);
        column_header
            .borrow_mut()
            .right
            .upgrade()
            .unwrap()
            .borrow_mut()
            .left = Rc::downgrade(column_header);
        // dbg!(column_header.borrow_mut().right.upgrade().unwrap());
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
    };
}
