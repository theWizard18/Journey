use std::io::{self, Write};

struct Game{}
impl Game {
    fn input(prompt: String) {
        print!("{prompt}");
        io::stdout().flush().expect("failed to flush the prompt");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to get input");
    }
}
