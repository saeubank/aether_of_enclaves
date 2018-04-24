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
mod map;
mod item;

use piston_window::*;
use game::Game;
use find_folder::Search;
use std::collections::HashMap;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("AOE", (650, 650))
        .exit_on_esc(true)
        .build()
        .expect("Error building window");

    window.set_max_fps(60);
    let mut window = window.ups(60);

    // Collect the graphics ("textures").
    let assets = Search::ParentsThenKids(3, 3).for_folder("images").expect("Error finding folder");
    let image_names = [
            "err",
            "sky",
            "floor_boards",
            "player",
            "wheel",
            "bisket",
            "floor_stone",
            "floor_grass",
            "floor_dirt",
            "water",
            "title_no_text",
            "title_text",
        ];

    let mut textures: HashMap<String, G2dTexture> = HashMap::new();

        for image_name in image_names.into_iter() {
            let filename = image_name.to_owned().to_owned() + ".png";
            let img = Texture::from_path(
                &mut window.factory,
                assets.join(filename.clone()),
                Flip::None,
                &TextureSettings::new(),
            ).expect(&format!("Not found: {:?}", filename));

            textures.insert(image_name.to_string(), img);
        }

    // Import all player sprites
    let dirs = ["N", "W", "S", "E", "NE", "NW", "SE", "SW"];
    for j in 0..dirs.len() {
        for i in 0..4 {
            let filename = format!("{}{}{}{}{}", "mc_", dirs[j], "_", i.to_string(), ".png");
            let mut map_name = format!("{}{}{}{}", "mc_", dirs[j], "_", i.to_string());
            let sprite = Texture::from_path(
                &mut window.factory,
                assets.join(&filename),
                Flip::None,
                &TextureSettings::new(),
            ).expect(&format!("Not found: {:?}", filename));
            textures.insert(map_name, sprite);
        }
    }

    let mut game = Game::new();
    game.run(&mut window, &textures);
}
