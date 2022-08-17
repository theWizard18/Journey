mod lib;

pub use crate::lib::Combatant;

fn main() {
    print!("{}", termion::clear::All);
    let player = Combatant::new("Namir");
    player.details();
}
