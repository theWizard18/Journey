mod lib;

pub use crate::lib::Player;

fn main() {
    let player = Player::new("Namir");
    player.show_status();
}
