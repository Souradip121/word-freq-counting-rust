mod reader;
mod counter;
mod display;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- \"your text here\"");
        println!("       cargo run -- file.txt");
        return;
    }

    let input = &args[1];
    let text = reader::read(input);
    let scoreboard = counter::count(&text);
    display::display(scoreboard);
}