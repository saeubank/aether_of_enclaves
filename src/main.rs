/**
    Aether of Enclaves will be a 32-bit exploration game, in which the user 
    controls a main character and an airship and travels through the sky - 
    picking up crew members, discovering new islands, interacting with NPCs, 
    and exploring.

    V 0.0.0

    2018 Samuel Eubanks, McKenzie Weller
*/

extern crate piston;
extern crate piston_window;
use piston_window::*;
mod input_handler;
mod user_interface;
mod game;
mod player;

fn main() {
    let mut game = game::Game::new();

    game.run();
}
