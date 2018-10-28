/*
The Rules
The simulation starts with an initial pattern of cells on the grid and computes successive
generations of cells according to the following rules:

1. A location that has zero or one neighbors will be empty in the next generation. If
a cell was in that location, it dies of loneliness.

2. A location with two neighbors is stable—that is, if it contained a cell, it still
contains a cell. If it was empty, it's still empty.

3. A location with three neighbors will contain a cell in the next generation. If it
was unoccupied before, a new cell is born. If it currently contains a cell, the cell
remains. Good times.

4. A location with four or more neighbors will be empty in the next generation. If
there was a cell in that location, it dies of overcrowding.

5. The births and deaths that transform one generation to the next must all take
effect simultaneously. Thus, when computing a new generation, new births and
deaths in that generation don’t impact other births and deaths in that generation.
To keep the two generations separate, you will need to work on two versions of
the grid—one for the current generation, and a second that allows you to
compute and store the next generation without changing the current one.
Check your understanding of these rules
*/

extern crate rand;
use rand::Rng;
use std::{thread, time};

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    alive: bool
}

pub struct Board {
    contents: [[Cell;20];20]
}

impl Board {
    pub fn create() -> Board {
        /* Create a board populated full of cells */

        let options = [true, false, false];

        let cells =  [ [Cell{alive: false}; 20]; 20 ];

        let mut board = Board {contents: cells};

        for x in 0..board.contents.len() {
            let y_value = board.contents[x];
            for y in 0..y_value.len() {

                // randomly choose if the initial state of each cell is alive
                let random_bool = rand::thread_rng().choose(&options);
                board.contents[x][y].alive = *random_bool.unwrap();
            }
        }

        board
    }

    fn display(&self) -> () {
        /* print the board to screen */

        Board::clear();

        // format and print out all the rows fo the board
        for x in 0..self.contents.len() {
            let y_value = self.contents[x];
            // collect all the values and provide any other visuals
            let mut row = "".to_owned();
            for y in 0..y_value.len() {
                match self.contents[x][y].alive {
                    true => row.push_str("█"),
                    false => row.push_str(" "),
                };
                row.push_str(" | ");
            }
            println!("| {}\n", row);
        }
    }

    fn clear() {
        // clear current screen
        // println!("\x1B[m");
        // println!("{}[2J", 27 as char);
        println!("{}c", 27 as char);
    }

    fn count_living_neighbours(&self, x:usize, y:usize) -> i32 {
        let mut result = 0;


        // Deal with previous row
        if x > 0 {
            if y > 0 {
                if self.contents[x-1][y-1].alive == true {result += 1};
            }

            if self.contents[x-1][y].alive == true {result += 1}

            if y+1 < self.contents[x-1].len() {
                if self.contents[x-1][y+1].alive == true {result += 1};
            }
        }

        // current row
        if y > 0 {
            if self.contents[x][y-1].alive == true {result += 1};
        }
        if y+1 < self.contents[x].len() {
            if self.contents[x][y+1].alive == true {result += 1};
        }

        // next row
        if x+1 < self.contents.len() {
            if y > 0 {
                if self.contents[x+1][y-1].alive == true {result += 1};
            }

            if self.contents[x+1][y].alive == true {result += 1}

            if y+1 < self.contents[x+1].len() {
                if self.contents[x+1][y+1].alive == true {result += 1};
            }
        }

        result
    }

    pub fn tick(&self) -> Board {
        /*
        Trigger the rules of the board for one tick
            - for each cell location:
            --> 0 || 1 neighbors = contents of cell killed if present
            --> 2 neighbors = no change
            --> 3 neighbors = cell contents born if not already present
            --> >= 4 neighbors = cell contents is removed if present
        */

        let mut new_board = Board::create();

        // 1. get cell
        // 2. test all neighbors for alive count
        // 3. create new board with new info --
        // 4. and replace
        for i in 0..self.contents.len() {
            for j in 0..self.contents[i].len() {

                let result = self.count_living_neighbours(i, j);
                match result {
                    0 | 1 => new_board.contents[i][j].alive = false,
                    2 => new_board.contents[i][j].alive = self.contents[i][j].alive,
                    3 => new_board.contents[i][j].alive = true,
                    _ => new_board.contents[i][j].alive = false
                }
                // println!("Alive: {}, {}: {}", i, j, result);
            }
        }

        new_board
    }


    fn delay(millis:u64) {
        let millis = time::Duration::from_millis(millis);
        thread::sleep(millis);
    }

}

fn main() {
    let mut b = Board::create();
    Board::delay(4000);
    for index in 0..1000 {
        b.display();
        println!("Iteration: {}", index);
        Board::delay(100);
        b = b.tick();
    }
}
