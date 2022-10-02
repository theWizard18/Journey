mod game;
mod resources;

pub use crate::resources::combatant::Combatant;

fn main() {
    print!("{}", termion::clear::All);
    let player = Combatant::new("Namir");
    player.details();
}
