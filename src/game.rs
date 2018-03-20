use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureType};
use input_handler::InputHandler;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    InGame,
    InMenu,
}

/**
	Implementation of the Game object.

	@field u_i The user interface.
*/
pub struct Game {
    input_hnd: InputHandler,
    player: Creature,
    game_state: GameState,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {
        Game {
            input_hnd: InputHandler::new(),
            player: Creature::new(CreatureType::Player),
            game_state: GameState::Title,
        }
    }

    // The function that draws stuff to the screen
    // @param e The graphics event that is for drawing.
    // @param window The PistonWindow that is drawn to.
    fn display(&mut self, e: Event, window: &mut PistonWindow, textures: &HashMap<&str, G2dTexture>) {
        let assets = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        let ref font = assets.join("Inconsolata-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        window.draw_2d(&e, |context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
            match self.game_state {
                GameState::InGame => {
                    image(textures.get("mc").unwrap(), context.transform.scale(1.25,1.25).trans(self.player.x, self.player.y), graphics);
                }
                GameState::Title => {
                    let transform = context.transform.trans(10.0, 100.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                        .draw(
                            "Press Return to begin :)",
                            &mut glyphs,
                            &context.draw_state,
                            transform,
                            graphics,
                        )
                        .unwrap();
                }
                GameState::InMenu => {
                    let transform = context.transform.trans(10.0, 100.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                        .draw(
                            "This is the menu :)",
                            &mut glyphs,
                            &context.draw_state,
                            transform,
                            graphics,
                        )
                        .unwrap();
                }
            }
        });
    }

    // The game loop.
    // @param window The PistonWindow that is drawn to.
    pub fn run(&mut self, window: &mut PistonWindow, textures: HashMap<&str, G2dTexture>) {
        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    self.input_hnd.handle_input(
                        args.state,
                        args.button,
                        &mut self.player,
                        &mut self.game_state,
                    );
                }

                // TODO Add lag handler here
                Event::Loop(Loop::Update(_args)) => {
                    self.player.update_position();
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(e, window, &textures);
                }
                _ => {}
            }
        }
    }
}

// fn get_graphics(window: &mut PistonWindow) -> HashMap<&str, G2dTexture> {
//     let assets = Search::ParentsThenKids(3, 3).for_folder("images").unwrap();
//     let sky = Texture::from_path(
//             &mut window.factory,
//             assets.join("sky.png"),
//             Flip::None,
//             &TextureSettings::new()
//             ).unwrap();
//     let boards = Texture::from_path(
//             &mut window.factory,
//             assets.join("boards.png"),
//             Flip::None,
//             &TextureSettings::new()
//             ).unwrap();
//     let mc = Texture::from_path(
//             &mut window.factory,
//             assets.join("player.png"),
//             Flip::None,
//             &TextureSettings::new()
//             ).unwrap();
//     let mut textures = HashMap::new();
//     textures.insert("sky", sky);
//     textures.insert("boards", boards);
//     textures.insert("mc", mc);
//     textures
// }



