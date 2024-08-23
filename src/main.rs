use game::Game;

pub mod component;
pub mod entity;
pub mod game;
pub mod system;
pub mod util;

fn main() {
    let mut game = Game::new();
    game.run();
}
