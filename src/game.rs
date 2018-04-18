use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureState, CreatureType};
use std::collections::HashMap;
use ship::Ship;
use tile::*;
use misc::*;
use item::*;
use map::Map;

const IMAGE_SIZE: f64 = 32.0;
const ANIMATION_RATE: u32 = 4;

#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    InGame,
    InMenu,
}

/* pub enum MenuOption {
     Top,
     Inventory, 
     //Options,
}*/

/**
    Implementation of the Game object.

    @field input_hnd The Input Handler.
    @field player The main player.
    @field game_state The Game State (see above). 
*/
pub struct Game {
    pub player: Creature,
    pub ship: Ship,
    pub game_state: GameState,
    pub item_prototypes: HashMap<String, Item>,
    pub items_in_game: Vec<Item>,
    pub map: Map,
    pub frames_since_last_draw: u32,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {
        let ship_tiles: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 2, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        let mut prototypes: HashMap<String, Item> = HashMap::new();
        prototypes.insert(
            "bisket".to_string(),
            Item::new(ItemType::Food(FoodType::Bisket), 1, true, 1.0),
        );
        prototypes.insert(
            "sword".to_string(),
            Item::new(
                ItemType::Interactable(InteractableType::Sword),
                10,
                true,
                5.0,
            ),
        );
        prototypes.insert(
            "grune".to_string(),
            Item::new(ItemType::Resource(ResourceType::Grune), 1, true, 0.5),
        );
        prototypes.insert(
            "logs".to_string(),
            Item::new(ItemType::Resource(ResourceType::Logs), 5, true, 8.0),
        );

        Game {
            player: Creature::new(CreatureType::Player),
            ship: Ship::new(ship_tiles),
            game_state: GameState::Title,
            item_prototypes: prototypes,
            items_in_game: vec![],
            map: Map::new(),
            frames_since_last_draw: 0,
        }
    }

    // The function that draws stuff to the screen
    // @param e The graphics event for drawing.
    // @param window The PistonWindow that is drawn to.
    // @param textures A HashMap of texture graphics.
    fn display(
        &mut self,
        e: Event,
        window: &mut PistonWindow,
        textures: &HashMap<String, G2dTexture>,
    ) {
        // Font locating.
        let assets = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        let ref font = assets.join("Inconsolata-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        let window_size = window.draw_size();

        window.draw_2d(&e, |context, graphics| {
            let w_width = window_size.width as f64;
            let w_height = window_size.height as f64;
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
            match self.game_state {
                GameState::InGame => {
                    let trans_x = w_width / 2.0 - self.player.x;
                    let trans_y = w_height / 2.0 - self.player.y;

                    image(
                        textures.get("sky").unwrap(),
                        context.transform.scale(w_width, w_height),
                        graphics,
                    );

                    for i in 0..self.map.tiles.len() {
                        if i as f64 * IMAGE_SIZE + IMAGE_SIZE > self.player.x - w_height / 2.0
                            && i as f64 * IMAGE_SIZE < self.player.x + w_height / 2.0
                        {
                            for j in 0..self.map.tiles[i].len() {
                                if j as f64 * IMAGE_SIZE + IMAGE_SIZE
                                    > self.player.y - w_width / 2.0
                                    && j as f64 * IMAGE_SIZE < self.player.y + w_width / 2.0
                                {
                                    match self.map.tiles[i][j].material {
                                        TileMaterial::Water => {
                                            image(
                                                textures.get("water").unwrap(),
                                                context
                                                    .transform
                                                    .trans(
                                                        i as f64 * IMAGE_SIZE,
                                                        j as f64 * IMAGE_SIZE,
                                                    )
                                                    .trans(trans_x, trans_y),
                                                graphics,
                                            );
                                        }
                                        TileMaterial::Stone => {
                                            image(
                                                textures.get("floor_stone").unwrap(),
                                                context
                                                    .transform
                                                    .trans(
                                                        i as f64 * IMAGE_SIZE,
                                                        j as f64 * IMAGE_SIZE,
                                                    )
                                                    .trans(trans_x, trans_y),
                                                graphics,
                                            );
                                        }
                                        TileMaterial::Grass => {
                                            image(
                                                textures.get("floor_grass").unwrap(),
                                                context
                                                    .transform
                                                    .trans(
                                                        i as f64 * IMAGE_SIZE,
                                                        j as f64 * IMAGE_SIZE,
                                                    )
                                                    .trans(trans_x, trans_y),
                                                graphics,
                                            );
                                        }
                                        TileMaterial::Dirt => {
                                            image(
                                                textures.get("floor_dirt").unwrap(),
                                                context
                                                    .transform
                                                    .trans(
                                                        i as f64 * IMAGE_SIZE,
                                                        j as f64 * IMAGE_SIZE,
                                                    )
                                                    .trans(trans_x, trans_y),
                                                graphics,
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    for i in 0..self.ship.tiles.len() {
                        for j in 0..self.ship.tiles[i].len() {
                            match self.ship.tiles[i][j].material {
                                TileMaterial::Wood => {
                                    image(
                                        textures.get("boards").unwrap(),
                                        context
                                            .transform
                                            .trans(
                                                self.ship.x + i as f64 * IMAGE_SIZE,
                                                self.ship.y + j as f64 * IMAGE_SIZE,
                                            )
                                            .trans(trans_x, trans_y),
                                        graphics,
                                    );
                                }
                                TileMaterial::Wheel => {
                                    image(
                                        textures.get("boards").unwrap(),
                                        context
                                            .transform
                                            .trans(
                                                self.ship.x + i as f64 * IMAGE_SIZE,
                                                self.ship.y + j as f64 * IMAGE_SIZE,
                                            )
                                            .trans(trans_x, trans_y),
                                        graphics,
                                    );
                                    image(
                                        textures.get("wheel").unwrap(),
                                        context
                                            .transform
                                            .trans(
                                                self.ship.x + i as f64 * IMAGE_SIZE,
                                                self.ship.y + j as f64 * IMAGE_SIZE,
                                            )
                                            .trans(trans_x, trans_y),
                                        graphics,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }

                    for i in 0..self.items_in_game.len() {
                        // let item_x = self.items_in_game[i].x;
                        // let item_y = self.items_in_game[i].y;
                        // if self.is_on_ship(item_x, item_y) {
                        //     self.items_in_game[i].x += self.ship.self_vel_x;
                        //     self.items_in_game[i].y += self.ship.self_vel_y;
                        // }
                        match self.items_in_game[i].item_type {
                            ItemType::Food(FoodType::Bisket) => {
                                image(
                                    textures.get("bisket").unwrap(),
                                    context
                                        .transform
                                        .trans(self.items_in_game[i].x, self.items_in_game[i].y)
                                        .trans(trans_x, trans_y),
                                    graphics,
                                );
                            }
                            _ => {}
                        }
                    }

                    // Player animation

                    let mut dir: String = "".to_string();

                    // TODO fuckin fix this mess
                    if self.player.self_vel_y != 0.0 || self.player.self_vel_x != 0.0 {
                        if self.player.directions.len() > 1 {
                            let dir_1 = direction_to_string(self.player.directions[0]);
                            let dir_2 = direction_to_string(self.player.directions[1]);

                            if dir_1 == "N".to_string() || dir_2 == "N".to_string() {
                                if dir_1 == "E".to_string() || dir_2 == "E".to_string() {
                                    dir = "NE".to_string();
                                } else {
                                    dir = "NW".to_string();
                                }
                            } else if dir_1 == "S".to_string() || dir_2 == "S".to_string() {
                                if dir_1 == "E".to_string() || dir_2 == "E".to_string() {
                                    dir = "SE".to_string();
                                } else {
                                    dir = "SW".to_string();
                                }
                            }
                        } else if self.player.directions.len() == 1 {
                            dir = direction_to_string(self.player.directions[0]);
                            self.player.dir = self.player.directions[0];
                        } else {
                            dir = direction_to_string(self.player.dir);
                        }
                    } else {
                        // Not moving
                        dir = direction_to_string(self.player.dir);
                        self.player.sprite_index = 2;
                    }

                    // Draw the player texture at player's x and y position.
                    image(
                        textures
                            .get(&format!(
                                "{}{}{}{}",
                                "mc_",
                                dir,
                                "_",
                                self.player.sprite_index.to_string()
                            ))
                            .unwrap(),
                        context.transform.trans(w_width / 2.0, w_height / 2.0),
                        graphics,
                    );

                    // TODO framerate stuff

                    if self.frames_since_last_draw > ANIMATION_RATE {
                        self.frames_since_last_draw = 0;
                        self.player.sprite_index = (self.player.sprite_index + 1) % 4;
                    }
                    self.frames_since_last_draw += 1;

                    // End player animation
                }

                GameState::Title => {
                    let title_img = textures.get("title_img").unwrap();

                    // For scaling / positioning text.
                    let title_txt = textures.get("title_text").unwrap();
                    let mut scale;
                    if w_height < w_width { 
                        scale = w_height / title_txt.get_width() as f64; 
                    }
                    else { 
                        scale = w_width / title_txt.get_width() as f64; 
                    }
                    let new_size = scale * title_txt.get_width() as f64;

                    image(
                        title_img,
                        context.transform.scale(w_width / title_img.get_width() as f64, w_height / title_img.get_height() as f64),
                        graphics,
                    );
                    image(
                        title_txt,
                        context.transform.
                            trans(w_width / 2.0 - new_size / 2.0, w_height / 2.0 - new_size / 2.0).
                            scale(scale, scale),
                        graphics,
                    );

                }
                GameState::InMenu => {
                    //let MenuOption m = MenuOption::Main;
                    //while(m != MenuOption::Exit) {
                    let transform = context.transform.trans(100.0, 100.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                        .draw(
                            "Inventory",
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
    pub fn run(&mut self, window: &mut PistonWindow, textures: &HashMap<String, G2dTexture>) {
        self.player.x = self.ship.x + ((self.ship.width / 2.0) * IMAGE_SIZE);
        self.player.y = self.ship.y + ((self.ship.height / 2.0) * IMAGE_SIZE);

        // Temporary
        self.items_in_game.push(
            self.item_prototypes
                .get("bisket")
                .unwrap()
                .generate_clone(self.ship.x + 128.0, self.ship.y + 128.0),
        );
        self.items_in_game[0].x_vel = self.ship.self_vel_x;
        self.items_in_game[0].y_vel = self.ship.self_vel_y;

        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    self.handle_input(args.state, args.button);
                }

                // TODO Add lag handler here
                Event::Loop(Loop::Update(_args)) => {
                    // Collision detection.
                    // TODO Create separate function.
                    self.player.other_vel_x = self.ship.self_vel_x;
                    self.player.other_vel_y = self.ship.self_vel_y;
                    self.player.update_position_other();
                    let x = self.player.x_to_be_location();
                    let y = self.player.y_to_be_location();
                    if self.is_on_ship(x, y) {
                        self.player.update_position_self();
                    }
                    self.ship.update_position();
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(e, window, textures);
                }
                _ => {}
            }
        }
    }

    // Checks whether a specific x,y position is on the ship.
    // @param x Some x coordinate.
    // @param y Some y coordinate.
    fn is_on_ship(&mut self, x: f64, y: f64) -> bool {
        // Check edges.
        let is_in_x =
            x >= self.ship.x && x + IMAGE_SIZE <= self.ship.x + self.ship.width * IMAGE_SIZE;
        let is_in_y =
            y >= self.ship.y && y + IMAGE_SIZE <= self.ship.y + self.ship.height * IMAGE_SIZE;
        if is_in_x && is_in_y {
            // Check specific tiles.
            let ship_tile_x = (x - self.ship.x) / IMAGE_SIZE;
            let ship_tile_y = (y - self.ship.y) / IMAGE_SIZE;
            if !self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.floor() as usize].passable
                || !self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.ceil() as usize]
                    .passable
                || !self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.floor() as usize]
                    .passable
                || !self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.ceil() as usize]
                    .passable
            {
                return false;
            }
            return true;
        }
        false
    }

    // Input Handling below.

    // @param state The ButtonState.
    // @param button The input button arguments.
    // @param player The player.
    // @param game_state The current Game State.
    fn handle_input(&mut self, state: ButtonState, button: Button) {
        use self::Key::*;
        match button {
            Button::Keyboard(key) => match key {
                // Action button.
                Return => self.execute_action(state, None),
                // Menu toggle.
                Tab => self.execute_open_menu(state, None),
                // Move.
                W | A | S | D => self.execute_move(state, Some(key)),
                V => self.execute_change_control_state(state, None),
                _ => {}
            },
            _ => {}
        }
    }

    // Opens menu.
    fn execute_open_menu(&mut self, state: ButtonState, _key: Option<Key>) {
        if state == ButtonState::Press {
            match self.game_state {
                GameState::InGame => {
                    println!("Menu opened.");
                    self.game_state = GameState::InMenu;
                }
                GameState::InMenu => {
                    println!("Menu closed.");
                    self.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }

    // Action button is pressed.
    fn execute_action(&mut self, state: ButtonState, _key: Option<Key>) {
        if state == ButtonState::Press {
            match self.game_state {
                GameState::Title => {
                    println!("Changing state to InGame.");
                    self.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }

    // Moves creature / ship.
    fn execute_move(&mut self, state: ButtonState, key: Option<Key>) {
        match self.player.creature_state {
            CreatureState::Normal => {
                self.player.handle_input(state, key);
                self.player.update_self_velocity();
            }
            CreatureState::ControllingShip => {
                self.ship.handle_input(state, key);
                self.ship.update_self_velocity();
            }
        }
    }

    // Change of player's control state.
    fn execute_change_control_state(&mut self, state: ButtonState, key: Option<Key>) {
        if state == ButtonState::Press {
            match self.player.creature_state {
                CreatureState::Normal => {
                    self.player.creature_state = CreatureState::ControllingShip;
                }
                _ => {
                    self.player.creature_state = CreatureState::Normal;
                }
            }
        }
    }
}
