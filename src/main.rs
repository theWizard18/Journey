mod lib;

pub use crate::lib::Player;

fn main() {
    let player = Player::new("Namir");
    println!("{}", player);
}
