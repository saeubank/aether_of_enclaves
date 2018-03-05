extern crate find_folder;
extern crate noise;
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
extern crate rand;
mod input_handler;
mod game;
mod creature;
// mod graphics;

use piston_window::{PistonWindow, WindowSettings};
use game::Game;

mod map;

fn main() {
    let temp = map::Island::new();
    // println!("{:?}", temp.getTiles());
    for i in temp.tiles {
        for j in i {
            match j {
                map::TileType::Air => print!("+"),
                map::TileType::Grass => print!("."),
                map::TileType::Water => print!("~"),
            }
        }
        println!("");
    }
    let mut window: PistonWindow = WindowSettings::new("AOE", (500, 500))
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new();
    game.run(&mut window);
}
