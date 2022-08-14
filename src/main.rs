mod lib;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;   
pub use crate::lib::Player;

fn main() {
    print!("{}", termion::clear::All);
    let player = Player::new("Namir");
    player.details();
}
