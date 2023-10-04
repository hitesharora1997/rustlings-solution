// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit = -16,
    Echo = -12,
    Move = -100,
    ChangeColor = -63,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
