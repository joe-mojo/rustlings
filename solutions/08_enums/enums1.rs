#[derive(Debug)]
enum Message {
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
