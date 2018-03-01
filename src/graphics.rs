/*
    Not used currently.
*/

// use piston_window::{PistonWindow, Texture};
use piston_window::*;
use find_folder::Search;
// use gfx_core::Resources;

pub struct Graphics {
    // grass: Texture<Resources>,
    grass: Texture<>
}

impl Graphics {
    pub fn new(&mut window: &mut PistonWindow) -> Self {
        let assets = Search::ParentsThenKids(3,3).for_folder("images").unwrap();
        Graphics {
            grass: Texture::from_path(
                &mut window.factory,
                assets.join("grass.png"),
                Flip::None,
                &TextureSettings::new()
            ).unwrap()
        }
    }
}
