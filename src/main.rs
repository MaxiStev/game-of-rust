use std::io;
use std::io::Write;

mod input;
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

fn main() {
    println!("bidde nummer weite eingeben");
    let stdin = io::stdin();
    let width: usize;
    {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        width = buffer.trim().parse::<usize>().unwrap_or(8);
    }
    println!("bidde nummer höhe eingeben");
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
            "c" => flip(&stdin, &mut game),
            _ => {game.step();}
        }
    }
}

fn flip(stdin: &io::Stdin, game: &mut game::Game) {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let list = input::parse(buffer);
    for pair in list.iter() {
        //println!("{:?}", pair);
        for x in pair.x.start..pair.x.end {
            for y in pair.y.start..pair.y.end {
                println!("{x},{y}");
                game.flip(x, y);
            }
        }
    }
}
