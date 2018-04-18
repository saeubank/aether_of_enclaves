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
mod game;
mod creature;
mod tile;
mod test;
mod ship;
mod misc;
mod item;

use piston_window::*;
use game::Game;
use find_folder::Search;
use std::collections::HashMap;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("AOE", (500, 500))
        .exit_on_esc(true)
        .build()
        .unwrap();

    // window.set_max_fps(60);
    // window.ups(60);

    // Collect the graphics ("textures").
    let assets = Search::ParentsThenKids(3, 3).for_folder("images").unwrap();
    let sky = Texture::from_path(
        &mut window.factory,
        assets.join("sky.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let boards = Texture::from_path(
        &mut window.factory,
        assets.join("boards.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let mc = Texture::from_path(
        &mut window.factory,
        assets.join("player.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let wheel = Texture::from_path(
        &mut window.factory,
        assets.join("wheel.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let bisket = Texture::from_path(
        &mut window.factory,
        assets.join("bisket.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let mut textures: HashMap<String, G2dTexture> = HashMap::new();
    textures.insert("sky".to_string(), sky);
    textures.insert("boards".to_string(), boards);
    textures.insert("mc".to_string(), mc);
    textures.insert("wheel".to_string(), wheel);
    textures.insert("bisket".to_string(), bisket);

    let dirs = ["N", "W", "S", "E", "NE", "NW", "SE", "SW"];

    // Import all player sprites
    for j in 0..dirs.len() {
        for i in 1..4 {
            let filename = format!("{}{}{}{}{}", "mc_", dirs[j], "_", i.to_string(), ".png");
            let mut map_name = format!("{}{}{}{}", "mc_", dirs[j], "_", i.to_string());
            let sprite = Texture::from_path(
                &mut window.factory,
                assets.join(&filename),
                Flip::None,
                &TextureSettings::new(),
            ).unwrap();
            textures.insert(map_name, sprite);
            // TODO clean this shit up lol
            if i == 2 {
                let sprite_2 = Texture::from_path(
                    &mut window.factory,
                    assets.join(&filename),
                    Flip::None,
                    &TextureSettings::new(),
                ).unwrap();
                let mut double = format!("{}{}{}", "mc_", dirs[j], "_0");
                textures.insert(double, sprite_2);
            }
        }
    }

    let mut game = Game::new();
    game.run(&mut window, &textures);
}
