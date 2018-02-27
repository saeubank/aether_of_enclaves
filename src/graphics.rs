/*
    Not used currently.
*/

use piston_window::*;

pub struct Graphics {
    grass: piston_window::Texture,
}

impl Graphics {
    pub fn new(&mut window: piston_window::PistonWindow) -> Self {
        // let assets = "./assets/images";
        Graphics {
            grass: Texture::from_path(
                &mut window.factory,
                // assets + "/grass.png",
                "./assets/images/grass.png",
                Flip::None,
                &TextureSettings::new()
            ).unwrap()
        }
    }
}
