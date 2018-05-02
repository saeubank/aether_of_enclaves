/**
    Aether of Enclaves is an exploration game, in which the user controls a main character 
    and travels through the sky with an airship to pick up crew members,
    discover islands, interact with NPCs, and explore.

    2018 Samuel Eubanks, McKenzie Weller
*/
extern crate find_folder;
extern crate noise;
extern crate piston_window;
extern crate rand;
mod game;
mod creature;
mod tile;
mod test;
mod ship;
mod misc;
mod map;
mod item;
mod constants;

use piston_window::*;
use game::Game;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("AOE", (650, 650))
        .exit_on_esc(true)
        .build()
        .expect("Error building window");

    let mut window = window.ups(60).ups_reset(0);

    let mut game = Game::new(&mut window);
    game.run(&mut window);
}
