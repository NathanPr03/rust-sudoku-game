# Rust Sudoku Game :)
This repository contains a sudoku game written in rust. In order to generate random boards, 
I use Donald Knuth's dancing links to solve a sudoku board

## How to set up and run 

### Unix
- Install `rustup` by executing `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Clone this repository with the command `git clone https://github.com/NathanPr03/rust-sudoku-game.git`
- Open a terminal session and navigate to the base directory of this project
- Run `cargo run`

### Windows
- Install `rustup` from this link https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
- Clone this repository with the command `git clone https://github.com/NathanPr03/rust-sudoku-game.git`
- Open a cmd line session and navigate to the base directory of this project
- Run `cargo run`

## Some links I found useful while creating this project
 - https://www.kth.se/social/files/58861771f276547fe1dbf8d1/HLaestanderMHarrysson_dkand14.pdf
 - https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm
 - https://github.com/masonium/exact-cover-rs/blob/master/src/cover.rs
 - https://ferrous-systems.com/blog/dlx-in-rust/
 - https://code.google.com/archive/p/narorumo/wikis/SudokuDLX.wiki
 - https://github.com/KarlHajal/DLX-Sudoku-Solver/blob/master/DLXSudokuSolver.cpp#L69
 - https://arxiv.org/pdf/cs/0011047.pdf

