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
    alive: i32
}

pub struct Board {
    contents: [[Cell;80];40]
}

impl Board {
    pub fn create() -> Board {
        /* Create a board populated full of cells */

        let cells =  [ [Cell{alive: 0}; 80]; 40 ];

        Board {contents: cells}

    }

    fn init_cells(&mut self) {
        let options = [1, 0, 0, 1, 0, 0];

        for x in 0..self.contents.len() {
            let y_value = self.contents[x];
            for y in 0..y_value.len() {

                // randomly choose if the initial state of each cell is alive
                let random_age = rand::thread_rng().choose(&options);
                self.contents[x][y].alive = *random_age.unwrap();
            }
        }
    }

    fn display(&self, delay:u64, story:&[&str; 4]) -> () {
        /* print the board to screen */

        Board::delay(delay);
        Board::clear();


        // format and print out all the rows fo the board
        for x in 0..self.contents.len() {
            let y_value = self.contents[x];

            // collect all the values and provide any other visuals
            let mut row = "".to_owned();
            for y in 0..y_value.len() {
                match self.contents[x][y].alive {
                    0 => row.push_str(" "),
                    1...5 => row.push_str(story[0]),  // new born
                    5...10 => row.push_str(story[1]), // middle age
                    11...18 => row.push_str(story[2]), // oooold
                    _ => row.push_str(story[3])
                };
            }
            println!("{}", row);
        }
    }

    fn clear() {
        /* clear current screen */
        if cfg!(target_os = "windows") {
            // This doesn't seem to work as expected on Windows 10.
            // std::process::Command::new("cls").status().expect("Failed to clear the screen.");

            for _ in 0..10 {
                println!("\n\n\n\n\n\n\n\n\n\n");
            }

        } else {
            std::process::Command::new("clear").status().expect("Failed to clear the screen.");
        }

       // println!("{}c", 27 as char);
    }

    fn count_living_neighbours(&self, x:usize, y:usize) -> i32 {
        let mut result = 0;

        // Deal with previous row
        if x > 0 {
            if y > 0 {
                if self.contents[x-1][y-1].alive > 0 {result += 1};
            }

            if self.contents[x-1][y].alive > 0 {result += 1}

            if y+1 < self.contents[x-1].len() {
                if self.contents[x-1][y+1].alive > 0 {result += 1};
            }
        }

        // current row
        if y > 0 {
            if self.contents[x][y-1].alive > 0 {result += 1};
        }

        // skip current cell

        if y+1 < self.contents[x].len() {
            if self.contents[x][y+1].alive > 0 {result += 1};
        }

        // next row
        if x+1 < self.contents.len() {
            if y > 0 {
                if self.contents[x+1][y-1].alive > 0 {result += 1};
            }

            if self.contents[x+1][y].alive > 0 {result += 1};

            if y+1 < self.contents[x+1].len() {
                if self.contents[x+1][y+1].alive > 0 {result += 1};
            }
        }

        result
    }

    fn tick(&self) -> Board {
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

                let alive_for = self.contents[i][j].alive;

                let result = self.count_living_neighbours(i, j);
                match result {
                    0 | 1 => new_board.contents[i][j].alive = 0,
                    2 => {
                        if alive_for > 0 {
                            new_board.contents[i][j].alive = alive_for + 1
                        } else {
                            new_board.contents[i][j].alive = alive_for
                        }
                    },
                    3 => new_board.contents[i][j].alive = alive_for + 1,
                    _ => new_board.contents[i][j].alive = 0
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
    let mut board = Board::create();
    board.init_cells();
    // uncomment to pause before starting for animated gif recording
    //Board::delay(8000);

    let stories = [
                    ["🤕", "😵", "💀", "👻"],
                    ["🥚", "🐣", "🐥", "🐓"],
                    ["░", "▒", "▓", "█"],
                    ["°", "°", "°", "°"],
                    ["∙", "∙", "∙", "∙"]
        ];

    let story = rand::thread_rng().choose(&stories);

    for index in 0..5000 {
        board.display(85, story.unwrap());
        println!("Iteration: {}", index);
        board = board.tick();

    }
}
