use crate::utils::{check_in_range, clear_screen};

const DESK_SIZE: usize = 10;
const MAX_SHIP_LEN: usize = 4;

pub struct Game;

impl Game {
    pub fn run() {
        let player1 = input!("Player 1: Please enter your name:", String);
        let mut player1 = Player::new(&player1);
        player1.create_ships();

        let player2 = input!("Player 2: Please enter your name:", String);
        let mut player2 = Player::new(&player2);
        player2.create_ships();

        loop {
            player1.make_shot(&player2);
            if player1.is_win("") { break; }
            player2.make_shot(&player1);
            if player2.is_win("") { break; }
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    field: [[usize; DESK_SIZE]; DESK_SIZE],
    monitor: [[usize; DESK_SIZE]; DESK_SIZE],
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            field: [[0; DESK_SIZE]; DESK_SIZE],
            monitor: [[0; DESK_SIZE]; DESK_SIZE],
        }
    }

    fn create_ships(&mut self) {
        let mut deck: usize = MAX_SHIP_LEN;

        while deck >= 1 {
            println!("\n{}, place your {}-th desk ship on the board:\n", self.name, deck);
            self.draw_field();

            let x = input!("Enter x coordinate:", usize);
            let y = input!("Enter y coordinate:", usize);
            let direction = input!("Choose direction:\n\
            1. Vertical\n\
            2. Horizontal", usize);

            if !check_in_range(direction, 0, 3) {
                println!("Direction is invalid! Try again.");
                continue;
            }

            if !self.is_available(x, y, deck, direction) {
                println!("This place is not available! Try again.");
                continue;
            }
            for i in 0..deck {
                if direction == 1 {
                    self.field[x][y + i] = 1;
                } else {
                    self.field[x + i][y] = 1;
                }
            }
            deck -= 1;
            clear_screen();
        }
    }

    fn draw_field(&self) {
        println!("  0 1 2 3 4 5 6 7 8 9");
        for i in 0..DESK_SIZE {
            print!("{} ", i);
            for j in 0..DESK_SIZE {
                if self.field[j][i] == 0 {
                    print!("- ");
                } else {
                    print!("X ");
                }
            }
            print!("\n");
        }
    }

    fn make_shot(&mut self, other: &Player) {
        loop {
            println!("{}, make your turn.", self.name);
            println!("  0 1 2 3 4 5 6 7 8 9");
            for i in 0..DESK_SIZE {
                print!("{} ", i);
                for j in 0..DESK_SIZE {
                    if self.monitor[j][i] == 0 { print!("- "); } else if self.monitor[j][i] == 1 { print!("o "); } else { print!("x "); }
                }
                print!("\n");
            }

            let x = input!("Enter x coordinate for shot:", usize);
            let y = input!("Enter y coordinate for shot:", usize);

            if other.field[x][y] == 1 {
                println!("Hit! Make your shot again.");
                self.monitor[x][y] = 2;
                if self.is_win(self.name.as_str()) { break; }
            } else {
                self.monitor[x][y] = 1;
                println!("Miss! Your enemy turns.");
                clear_screen();
                break;
            }
            clear_screen();
        }
    }

    fn is_win(&self, name: &str) -> bool {
        let mut counter = 0;
        for i in 0..DESK_SIZE {
            for j in 0..DESK_SIZE {
                if self.monitor[i][j] == 2 { counter += 1 }
            }
        }

        if counter >= 10 {
            println!("{} win!", name);
            return true;
        }
        false
    }

    fn is_available(&self, x: usize, y: usize, deck: usize, direction: usize) -> bool {
        let mut deck = deck;
        let near = if x == 0 || y == 0 { 0 } else { 1usize };
        // out of field`s border
        if direction == 1 {
            if y + deck > DESK_SIZE { return false; }
        }
        if direction == 2 {
            if x + deck > DESK_SIZE { return false; }
        }

        // neighbours ships check
        while deck != 0 {
            for i in 0..deck {
                let mut xi = 0;
                let mut yi = 0;

                if direction == 1 { yi = i; } else { xi = i; }

                if x + near + xi < DESK_SIZE && (x + near + xi) as isize >= 0 {
                    if self.field[x + near + xi][y + yi] != 0 { return false; }
                }
                if x - near + xi < DESK_SIZE && (x - near + xi) as isize >= 0 {
                    if self.field[x - near + xi][y + yi] != 0 { return false; }
                }

                if y + near + yi < DESK_SIZE && (y + near + yi) as isize >= 0 {
                    if self.field[x + xi][y + near + yi] != 0 { return false; }
                }
                if y - near + yi < DESK_SIZE && (y - near + yi) as isize >= 0 {
                    if self.field[x + xi][y - near + yi] != 0 { return false; }
                }
            }
            deck -= 1;
        }
        true
    }
}