mod game;
mod resources;

pub use crate::resources::{combatant::Combatant, party::Party};

fn main() {
    print!("{}", termion::clear::All);
    let mut party = Party::new(vec![], vec![]);
    let player = Combatant::new("Namir");
    party.recruit(player);
}
