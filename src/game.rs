use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureType};
use input_handler::InputHandler;
use std::collections::HashMap;
use ship::Ship;
use tile::*;

const WIDTH: f64 = 500.0;
const HEIGHT: f64 = 500.0;
const IMAGE_SIZE: f64 = 32.0;


#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    InGame,
    InMenu,
}

/**
    Implementation of the Game object.

    @field input_hnd The Input Handler.
    @field player The main player.
    @field game_state The Game State (see above). 
*/
pub struct Game {
    input_hnd: InputHandler,
    player: Creature,
    ship: Ship,
    game_state: GameState,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {

        let ship_tiles: Vec<Vec<i32>> = 
        vec![
        vec![0,0,1,1,1,0,0],
        vec![0,1,1,1,1,1,0],
        vec![0,1,1,2,1,1,0],
        vec![1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1],
        ];

        Game {
            input_hnd: InputHandler::new(),
            player: Creature::new(CreatureType::Player),
            ship: Ship::new(ship_tiles),
            game_state: GameState::Title,
        }
    }

    // The function that draws stuff to the screen
    // @param e The graphics event for drawing.
    // @param window The PistonWindow that is drawn to.
    // @param textures A HashMap of texture graphics.
    fn display(&mut self, e: Event, window: &mut PistonWindow, textures: &HashMap<&str, G2dTexture>) {
        // Font locating.
        let assets = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        let ref font = assets.join("Inconsolata-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        window.draw_2d(&e, |context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
            match self.game_state {
                GameState::InGame => {
                    image(textures.get("sky").unwrap(), context.transform.scale(WIDTH,HEIGHT), graphics);
                    for i in 0..self.ship.tiles.len() {
                        for j in 0..self.ship.tiles[i].len() {
                            match self.ship.tiles[i][j].material {
                                TileMaterial::Wood => {
                                    image(textures.get("boards").unwrap(), context.transform.trans(self.ship.x + i as f64 * IMAGE_SIZE, self.ship.y + j as f64 * IMAGE_SIZE), graphics);
                                },
                                TileMaterial::Wheel => {
                                    image(textures.get("boards").unwrap(), context.transform.trans(self.ship.x + i as f64 * IMAGE_SIZE, self.ship.y + j as f64 * IMAGE_SIZE), graphics);
                                    image(textures.get("wheel").unwrap(), context.transform.trans(self.ship.x + i as f64 * IMAGE_SIZE, self.ship.y + j as f64 * IMAGE_SIZE), graphics);
                                }
                                _ => {}
                            }
                        }
                    }
                    // Draw the player texture at player's x and y position.
                    image(textures.get("mc").unwrap(), context.transform.trans(self.player.x - IMAGE_SIZE/2.0, self.player.y - IMAGE_SIZE/2.0), graphics);
                }
                GameState::Title => {
                    let transform = context.transform.trans(WIDTH/2.0, HEIGHT/2.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                        .draw(
                            "Press Enter to begin.",
                            &mut glyphs,
                            &context.draw_state,
                            transform,
                            graphics,
                        )
                        .unwrap();
                }
                GameState::InMenu => {
                    let transform = context.transform.trans(WIDTH/2.0, HEIGHT/2.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                        .draw(
                            "This is the menu.",
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

    // The game loop. Displays the screen and updates events.
    // @param window The PistonWindow that is drawn to.
    // @param textures HashMap of graphics textures.
    pub fn run(&mut self, window: &mut PistonWindow, textures: HashMap<&str, G2dTexture>) {
        self.player.x = self.ship.x + ((self.ship.width / 2.0) * IMAGE_SIZE);
        self.player.y = self.ship.y + ((self.ship.height / 2.0) * IMAGE_SIZE);
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



