use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureType};
use input_handler::InputHandler;

#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    Main,
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
    fn display(&mut self, e: Event, window: &mut PistonWindow) {
        let assets = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        let ref font = assets.join("Inconsolata-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        window.draw_2d(&e, |context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
            match self.game_state {
                GameState::Main => {
                    // Draw Player.
                    let red = [1.0, 0.0, 0.0, 1.0];
                    let player_image = [self.player.x, self.player.y, 15.0, 15.0];
                    rectangle(red, player_image, context.transform, graphics);
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
    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    // make it so that when key is pressed velocity is changed once
                    // then make it so that when key is released velocity is changed back
                    // make sure to watch out for bug if speed is chnaged while button was pressed
                    // but also make sure to keep velocity of what person is on
                    // maybe can be solved by no allowing for speed to be changed while moving (keydown)
                    self.input_hnd.handle_input(
                        args.state,
                        args.button,
                        &mut self.player,
                        &mut self.game_state,
                    );

                    // if arges.state == ButtonState::Release
                }

                // add lag handler here
                Event::Loop(Loop::Update(_args)) => {
                    self.player.update_position();
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(e, window);
                }
                _ => {}
            }
        }
    }

    // TODO add grapics for things here?
    // fn get_graphics() {
    //     let assets = Search::ParentsThenKids(3,3).for_folder("images").unwrap();
    //     Graphics {
    //         grass: Texture::from_path(
    //             &mut window.factory,
    //             assets.join("grass.png"),
    //             Flip::None,
    //             &TextureSettings::new()
    //         ).unwrap()
    //     }
    // }
}
