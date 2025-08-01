#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Move { x: i32, y: i32 },
    Resize { width: u32, height: u32 },
    Echo(String),
    ChangeColor { red: u8, green: u8, blue: u8 },
    Quit
}

fn main() {
    println!("{:?}", Message::Resize { width: 800, height: 600 });
    println!("{:?}", Message::Move { x: 10, y: 15 });
    println!("{:?}", Message::Echo("Hello world".to_string()));
    println!("{:?}", Message::ChangeColor { red: 255, green: 0, blue: 0 });
    println!("{:?}", Message::Quit);
}

// That was a pain in the arse because I was being stupid and put the main fn inside the enum block.
// I am an idiot. It took me a while to figure out why the compiler was complaining about the main function not being found.
// I should have just put it outside the enum block like a normal person. But hey, at least I learned something new today!