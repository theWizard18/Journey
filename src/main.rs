mod lib;

pub use crate::lib::Player;

fn main() {
    print!("{}", termion::clear::All);
    let player = Player::new("Namir");
    player.details();
}
