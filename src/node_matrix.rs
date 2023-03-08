use std::cell::{BorrowError, RefCell};
use std::rc::{Rc, Weak};
use crate::{BOARD_SIZE, BOARD_SIZE_SQUARED, ColumnIterator, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, four_by_four_cover_matrix, ninebyninecovermatrix};
use crate::iter::RowIterator;
use crate::ninebyninecovermatrix::nine_by_nine_cover_matrix;

use crate::node::{Node, StrongNode, WeakNode};

pub struct NodeMatrix {
    pub root_node: StrongNode,
    column_nodes: Vec<StrongNode>,
    rows: Vec<Vec<StrongNode>>,
    pub solution: Vec<StrongNode>
}

impl NodeMatrix {
    pub fn new() -> NodeMatrix {
        return NodeMatrix {
            root_node: Node::new_root(),
            column_nodes: Vec::new(),
            rows: Vec::new(),
            solution: Vec::new()
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
        // self.print_matrix_at_given_point();

        println!("Count is: {}", count);
        {
            let borrowed_root = &mut self.root_node.borrow();

            if borrowed_root.right.upgrade().unwrap().borrow().extra == borrowed_root.extra {
                println!("SOLUTION FOUND!!!");
                self.convert_matrix_to_sudoku_grid();
                // self.print_matrix_solution();
                return;
            }
        }

        let mut column_node = self.choose_column();
        // dbg!(&column_node);

        NodeMatrix::cover(&column_node);
        let column_iterator = ColumnIterator::new(&column_node);

        // self.print_matrix_at_given_point();

        for mut node in column_iterator {
            let row_iterator = RowIterator::new(&node);
            self.solution.push(node.upgrade().unwrap());

            for row_node in row_iterator {
                // dbg!(&row_node.upgrade().unwrap());
                let header = row_node.upgrade().unwrap().borrow_mut().header.upgrade().unwrap();
                NodeMatrix::cover(&header);

                // self.print_matrix_at_given_point();
            }

            // self.print_matrix_solution();
            self.solve(count + 1);

            self.solution.pop();

            // dbg!(node.upgrade().unwrap());
            let row_iterator_reverse = RowIterator::new(&node);

            for new_node in row_iterator_reverse.rev() {
                let header = &new_node.upgrade().unwrap().borrow_mut().header.upgrade().unwrap();

                // dbg!(header.clone());
                NodeMatrix::uncover(header);

                // self.print_matrix_at_given_point();
            }
        }

        NodeMatrix::uncover(&column_node);
        // self.print_matrix_at_given_point();
        // // dbg!(&self.solution);

        return;
    }

    fn convert_matrix_to_sudoku_grid(&self)
    {
        let mut board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
            [[0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0]];

        for node in self.solution.clone() {
            let exact_cover_row_index = node.borrow().get_row().unwrap();
            let row = (exact_cover_row_index) / BOARD_SIZE_SQUARED as usize;
            let column = ((exact_cover_row_index) % BOARD_SIZE_SQUARED as usize ) / BOARD_SIZE as usize;
            let mut value = (exact_cover_row_index) % BOARD_SIZE as usize;

            if value == 0 {
                value = BOARD_SIZE as usize;
            }

            board[row][column] = value;
        }

        for i in 0..BOARD_SIZE * 2 + 1 {
            print!("-");
        }
        println!();
        for i in 0..board.len() {
            for j in 0..board[1].len() {
                print!("|");
                print!("{}", board[i][j]);
            }
            print!("|");
            println!();
            for i in 0..BOARD_SIZE * 2 + 1 {
                print!("-");
            }
            println!();
        }

    }

    fn print_matrix_at_given_point(&self)
    {
        // let mut cover_matrix = [
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0],
        //     [0,0,0,0,0,0,0]
        // ];

        let mut cover_matrix = nine_by_nine_cover_matrix();
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

    pub fn print_matrix_solution(&self)
    {
        let mut cover_matrix = four_by_four_cover_matrix();

        for node in self.solution.clone() {
            let col_index = node.borrow_mut().column_index.unwrap();
            let row_index = node.borrow_mut().get_row().unwrap();

            cover_matrix[row_index][col_index] = 1;
        }

        println!("---------------------------------------------------------------------");
        for i in 0..cover_matrix.len() {
            for j in 0..cover_matrix[1].len() {
                print!("{}", cover_matrix[i][j]);
            }
            print!("|");
            println!();
            println!("---------------------------------------------------------------------")
        }
    }

    pub fn choose_column(&mut self) -> StrongNode
    {
        let mut lowest_count = EXACT_COVER_MATRIX_ROWS as usize;

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
        // dbg!(column_header.borrow_mut().left.upgrade().unwrap());
        column_header.borrow_mut().left.upgrade().unwrap().borrow_mut().right = Rc::downgrade(column_header);
        column_header.borrow_mut().right.upgrade().unwrap().borrow_mut().left = Rc::downgrade(column_header);
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
    }
}