/**
    Aether of Enclaves will be a 32-bit exploration game, in which the user
    controls a main character and an airship and travels through the sky -
    picking up crew members, discovering new islands, interacting with NPCs,
    and exploring.

    V 0.0.0

    2018 Samuel Eubanks, McKenzie Weller
*/
extern crate find_folder;
extern crate noise;
extern crate piston;
extern crate piston_window;
extern crate rand;
mod input_handler;
mod game;
mod creature;
mod tile;
mod test;
mod ship;

// mod map;

use piston_window::*;
use game::Game;
use find_folder::Search;
use std::collections::HashMap;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("AOE", (500, 500))
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(60);

    let assets = Search::ParentsThenKids(3, 3).for_folder("images").unwrap();
    let sky = Texture::from_path(
            &mut window.factory,
            assets.join("sky.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();
    let boards = Texture::from_path(
            &mut window.factory,
            assets.join("boards.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();
    let mc = Texture::from_path(
            &mut window.factory,
            assets.join("player_64.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();
    let mut textures = HashMap::new();
    textures.insert("sky", sky);
    textures.insert("boards", boards);
    textures.insert("mc", mc);


    let mut game = Game::new();
    game.run(&mut window, textures);
}
