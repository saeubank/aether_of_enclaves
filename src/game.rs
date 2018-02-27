use piston_window::*;
use find_folder::*;
use player::Player;
use input_handler::InputHandler;

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
    player: Player,
    game_state: GameState,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {
        Game {
            input_hnd: InputHandler::new(),
            player: Player::new(),
            game_state: GameState::Title,
        }
    }

    // The function that draws stuff to the screen
    // @param e The graphics event that is for drawing.
    // @param window The PistonWindow that is drawn to.
    fn display(&mut self, e: Event, window: &mut PistonWindow) {
        let assets = Search::ParentsThenKids(3, 3)
            .for_folder("fonts").unwrap();
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
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                        "Press Return to begin :)",
                        &mut glyphs,
                        &context.draw_state,
                        transform, graphics
                    ).expect("duno what to put here. Title");
                }
                GameState::InMenu => {
                    let transform = context.transform.trans(10.0, 100.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                        "This is the menu :)",
                        &mut glyphs,
                        &context.draw_state,
                        transform, graphics
                    ).expect("duno what to put here. InMenu");
                }
                // _ => {}
            }
        });
    }

    // The game loop.
    // @param window The PistonWindow that is drawn to.
    pub fn run(&mut self, window: &mut PistonWindow) {

        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    if args.state == ButtonState::Press {
                        self.input_hnd.handle_input(
                            args.button,
                            &mut self.player,
                            &mut self.game_state,
                        );
                    }
                }

                Event::Loop(Loop::Update(_args)) => {
                    //Update Events
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(e, window);
                }
                _ => {}
            }
        }
    }
}
