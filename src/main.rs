use std::io;
use std::io::Write;

mod game;
fn draw(stdout: &mut io::Stdout, field: &Vec<Vec<bool>>) {
    for row in field {
        for col in row {
            write!(stdout, "{} ", {if *col {"X"} else {"0"}}).unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
    println!("-----------");
}

// fn step(field: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
//     let height = field.len();
//     let width = field[0].len();
//     let mut next_field = vec![vec![false; width]; height];
//     for row in 0usize..height {
//         for col in 0usize..width {
//             let mut count = 0;
//             let prev_row = row.checked_sub(1).unwrap_or(height -1);
//             let next_row = if row +1 >= height {0} else {row +1};
//             let prev_col = col.checked_sub(1).unwrap_or(width -1);
//             let next_col = if col +1 >= width {0} else {col +1};
//             if field[prev_row][prev_col] {count+=1};
//             if field[prev_row][col] {count+=1};
//             if field[prev_row][next_col] {count+=1};
//             if field[row][prev_col] {count+=1};
//             if field[row][next_col] {count+=1};
//             if field[next_row][prev_col] {count+=1};
//             if field[next_row][col] {count+=1};
//             if field[next_row][next_col] {count+=1};

//             next_field[row][col] =
//             if field[row][col] {
//                 if count == 2 || count == 3 {
//                     true
//                 } else {
//                     false
//                 }
//             } else if count == 3 {
//                 true
//             }
//             else {
//                 false
//             };
//         }
//     }
//     next_field
// }

fn main() {
    println!("bidde nummer weite eingeben");
    let stdin = io::stdin();
    let width: usize;
    {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        width = buffer.trim().parse::<usize>().unwrap_or(8);
    }
    println!("bidde höhe weite eingeben");
    let height: usize;
    {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        height = buffer.trim().parse::<usize>().unwrap_or(8);
    }
    println!("{} {}", height, width);
    let mut game: game::Game = game::Game::new(height, width);

    let mut stdout = io::stdout();

    let mut buffer: String;
    loop {
        draw(&mut stdout, &game.get_field());
        buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "q" => break,
            "c" => swap(&stdin, &mut game),
            _ => {game.step();}
        }
    }
}

fn swap(stdin: &io::Stdin, game: &mut game::Game) {
    let row: usize;
    let col: usize;
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    row = buffer.trim().parse::<usize>().unwrap();
    buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    col = buffer.trim().parse::<usize>().unwrap();
    game.swap(row, col);
}
