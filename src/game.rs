//! This is the functionality for the game loop, handled using a Game object.
//! The object maintains updating and graphics rendering for all other components.

use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureState};
use texture::TextureSettings;
use std::collections::HashMap;
use ship::Ship;
use misc::*;
use item::*;
use map::Map;
use constants::*;
use tile::TileType;

#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    InGame,
    InMenu,
    GameOver,
}

enum PlayerLocation {
    OnShip,
    InWorld,
}

/**
    Implementation of the Game object.

    @field player The main player.
    @field ship The player's airship.
    @field game_state The Game State (see above). 
    @field player_location Player's worldly position (see above).
    @field item_prototypes Prototyping pattern for cloning items.
    @field items_in_game Set of all items in the game.
    @field map The world map.
    @glyphs Glyphs library for graphics.
    @textures HashMap of sprite / tile textures.
*/
pub struct Game {
    player: Creature,
    ship: Ship,
    game_state: GameState,
    player_location: PlayerLocation,
    item_prototypes: HashMap<String, Item>,
    items_in_game: Vec<Item>,
    map: Map,
    glyphs: Glyphs,
    textures: HashMap<String, G2dTexture>,
}

impl Game {
    /*
        Game constructor.
    */
    pub fn new(window: &mut PistonWindow) -> Self {
        let ship_tiles: Vec<Vec<i32>> = vec![ // Default ship.
            vec![0, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 2, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        let item_prototypes = generate_item_prototypes();
        let glyphs = generate_glyphs(window);
        let textures = generate_textures(window);

        Game {
            player: Creature::new(),
            ship: Ship::new(ship_tiles),
            game_state: GameState::Title,
            player_location: PlayerLocation::OnShip,
            item_prototypes: item_prototypes,
            items_in_game: vec![],
            map: Map::new(MAP_WIDTH, MAP_HEIGHT),
            glyphs: glyphs,
            textures: textures,
        }
    }

    /*
        Draws graphics to the window.
    
        @param e The graphics event for drawing.
        @param window The PistonWindow that is drawn to.
    */
    fn display(&mut self, e: &Event, window: &mut PistonWindow) {
        let window_size = window.draw_size(); // Updates screen upon resizing.

        window.draw_2d(e, |context, mut graphics| {
            let w_width = window_size.width as f64;
            let w_height = window_size.height as f64;
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen for new draw.
            match self.game_state {
                GameState::InGame => {
                    // Translations for objects around the player.
                    let trans_x = w_width / 2.0 - self.player.x;
                    let trans_y = w_height / 2.0 - self.player.y;

                    let sky_img_string = "sky";
                    // Sky background.
                    image(
                        self.textures
                            .get(sky_img_string)
                            .expect(&format!("Not found: {:?}", sky_img_string)),
                        context.transform.scale(w_width, w_height),
                        graphics,
                    );

                    self.map.draw(
                        &self.textures,
                        &context,
                        &mut graphics,
                        w_width,
                        w_height,
                        self.player.x,
                        self.player.y,
                        trans_x,
                        trans_y,
                    );

                    // Draw items.
                    for i in 0..self.items_in_game.len() {
                        if self.items_in_game[i].x - self.player.x
                            > -w_width / 2.0 - IMAGE_SIZE_SCALED
                            && self.items_in_game[i].x - self.player.x < w_width / 2.0
                            && self.items_in_game[i].y - self.player.y > -w_width / 2.0
                            && self.items_in_game[i].y - self.player.y < w_width / 2.0
                        {
                            self.items_in_game[i].draw(
                                &self.textures,
                                &context,
                                &mut graphics,
                                trans_x,
                                trans_y,
                            );
                        }

                    }

                    match self.player_location {
                        PlayerLocation::OnShip => self.ship.draw( // Don't draw ship if player isn't on board.
                            &self.textures,
                            &context,
                            &mut graphics,
                            trans_x,
                            trans_y,
                        ),
                        PlayerLocation::InWorld => {}
                    }

                    self.player
                        .draw(&self.textures, &context, &mut graphics, w_width, w_height);

                        // Draw health at top of screen.
                    for i in 0..self.player.health {
                        image(
                            textures
                                .get(IMG_HEART)
                                .expect(&format!("Not found: {:?}", IMG_HEART)),
                            context
                                .transform
                                .trans(25.0 + i as f64 * (IMAGE_SIZE_SCALED + 2.0), 25.0)
                                .scale(IMAGE_SCALE, IMAGE_SCALE),
                            graphics,
                        );
                    }

                    // End in-game graphics.
                }

                GameState::Title => {
                    // Draw title screen.
                    let img = IMG_TITLE_NO_TEXT;
                    let title_img = self.textures
                        .get(img)
                        .expect(&format!("Not found: {:?}", img));

                    // For scaling / positioning text.
                    let img = IMG_TITLE_TEXT;
                    let title_txt = self.textures
                        .get(img)
                        .expect(&format!("Not found: {:?}", img));
                    let mut scale;
                    if w_height < w_width {
                        scale = w_height / title_txt.get_width() as f64;
                    } else {
                        scale = w_width / title_txt.get_width() as f64;
                    }
                    let new_size = scale * title_txt.get_width() as f64;

                    image(
                        title_img,
                        context.transform.scale(
                            w_width / title_img.get_width() as f64,
                            w_height / title_img.get_height() as f64,
                        ),
                        graphics,
                    );
                    // Draw title text on background.
                    image(
                        title_txt,
                        context
                            .transform
                            .trans(
                                w_width / 2.0 - new_size / 2.0,
                                w_height / 2.0 - new_size / 2.0,
                            )
                            .scale(scale, scale),
                        graphics,
                    );
                }
                GameState::InMenu => {
                    let transform_y = 100.0;
                    let draw_text = ["Controls:",
                    "W/A/S/D: Movement",
                    "Tab: Enter/Exit this menu",
                    "E: Use item/Interact",
                    "Space: Pickup/Drop item"];
                    let font = 24;
                    for i in 0..draw_text.len() {
                    text(
                        [1.0; 4],
                        font,
                        draw_text[i],
                        &mut self.glyphs,
                        context.transform.trans(100.0, transform_y + i as f64 * font as f64),
                        graphics,
                    ).expect(&format!("Error drawing {}", draw_text[i]));
                    }
                }
                GameState::GameOver => {
                    let draw_text = "GAME OVER";
                    let font = 24;
                    let x = (font * (draw_text.len() as u32)) as f64 / 2.0;
                    let transform = context.transform.trans(w_width / 2.0 - x, w_height / 2.0);

                    text(
                        [1.0; 4],
                        font,
                        draw_text,
                        &mut self.glyphs,
                        transform,
                        graphics,
                    ).expect(&format!("Error drawing {}", draw_text));
                }
            }
        });
    }

    // The game loop. Displays the screen and updates events.
    // @param window The PistonWindow that is drawn to.
    // @param textures HashMap of graphics textures.
    pub fn run(&mut self, window: &mut PistonWindow) {
        self.ship.x = MAP_WIDTH as f64 * IMAGE_SIZE_SCALED / 2.0;
        self.ship.y = MAP_HEIGHT as f64 * IMAGE_SIZE_SCALED / 2.0;
        self.player.x = self.ship.x + ((self.ship.width / 2.0) * IMAGE_SIZE_SCALED);
        self.player.y = self.ship.y + ((self.ship.height / 2.0) * IMAGE_SIZE_SCALED);

        // Temporary
        self.items_in_game.push(
            self.item_prototypes
                .get("bisket")
                .unwrap()
                .generate_clone(self.ship.x + 128.0, self.ship.y + 128.0),
        );

        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    self.handle_input(&args.state, &args.button);
                }

                // TODO Add lag handler here
                Event::Loop(Loop::Update(_args)) => {
                    self.update();
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(&e, window);
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        if self.player.is_dead() {
            self.game_state = GameState::GameOver;
        }
        // Collision detection.
        // TODO Create separate function.
        if self.game_state == GameState::InGame {
            match self.player_location {
                PlayerLocation::OnShip => {
                    self.player.other_vel_x = self.ship.self_vel_x;
                    self.player.other_vel_y = self.ship.self_vel_y;
                    self.player.update_position_other();
                    let x = self.player.x_to_be_location();
                    let y = self.player.y_to_be_location();
                    if self.is_on_ship(x, y) {
                        self.player.update_position_self();
                        self.player.update_direction();
                    }
                    self.ship.update_position();
                }
                PlayerLocation::InWorld => {
                    let x = self.player.x_to_be_location();
                    let y = self.player.y_to_be_location();
                    if self.can_go_to(x, y) {
                        self.player.update_position_self();
                        self.player.update_direction();
                    }
                }
            }
        }
    }

    fn tiletype_under_player(&self) -> Option<TileType> {
        let x = self.player.x + IMAGE_SIZE_SCALED as f64 / 2.0;
        let y = self.player.y + IMAGE_SIZE_SCALED as f64 / 2.0;
        let iss = IMAGE_SIZE_SCALED as f64;
        match self.player_location {
            PlayerLocation::OnShip => {
                let is_in_x = x >= self.ship.x && x + iss <= self.ship.x + self.ship.width * iss;
                let is_in_y = y >= self.ship.y && y + iss <= self.ship.y + self.ship.height * iss;
                if is_in_x && is_in_y {
                    return Some(
                        self.ship.tiles[x.floor() as usize][y.floor() as usize]
                            .tile_type
                            .clone(),
                    );
                }
            }
            PlayerLocation::InWorld => {
                let is_in_x = x >= 0.0 && x + iss <= MAP_WIDTH as f64 * iss;
                let is_in_y = y >= 0.0 && y + iss <= MAP_HEIGHT as f64 * iss;
                if is_in_x && is_in_y {
                    return Some(
                        self.map.tiles[x.floor() as usize][y.floor() as usize]
                            .tile_type
                            .clone(),
                    );
                }
            }
        }
        None
    }

    fn can_go_to(&self, x: f64, y: f64) -> bool {
        let iss = IMAGE_SIZE_SCALED as f64;
        let is_in_x = x >= 0.0 && x + iss <= MAP_WIDTH as f64 * iss;
        let is_in_y = y >= 0.0 && y + iss <= MAP_HEIGHT as f64 * iss;
        if is_in_x && is_in_y {
            let x = x / iss;
            let y = y / iss;
            if self.map.tiles[x.floor() as usize][y.floor() as usize].passable
                && self.map.tiles[x.floor() as usize][y.ceil() as usize].passable
                && self.map.tiles[x.ceil() as usize][y.floor() as usize].passable
                && self.map.tiles[x.ceil() as usize][y.ceil() as usize].passable
            {
                return true;
            }
        }
        false
    }

    // Checks whether a specific x,y position is on the ship.
    // @param x Some x coordinate.
    // @param y Some y coordinate.
    fn is_on_ship(&self, x: f64, y: f64) -> bool {
        let ship_x = self.ship.x_to_be_location();
        let ship_y = self.ship.y_to_be_location();
        // Check edges.
        let is_in_x =
            x >= ship_x && x + IMAGE_SIZE_SCALED <= ship_x + self.ship.width * IMAGE_SIZE_SCALED;
        let is_in_y =
            y >= ship_y && y + IMAGE_SIZE_SCALED <= ship_y + self.ship.height * IMAGE_SIZE_SCALED;
        if is_in_x && is_in_y {
            // Check specific tiles.
            let ship_tile_x = (x - ship_x) / IMAGE_SIZE_SCALED;
            let ship_tile_y = (y - ship_y) / IMAGE_SIZE_SCALED;
            if self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.floor() as usize].passable
                && self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.ceil() as usize]
                    .passable
                && self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.floor() as usize]
                    .passable
                && self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.ceil() as usize]
                    .passable
            {
                return true;
            }
        }
        false
    }

    // Input Handling below.

    // @param state The ButtonState.
    // @param button The input button arguments.
    // @param player The player.
    // @param game_state The current Game State.
    fn handle_input(&mut self, state: &ButtonState, button: &Button) {
        use self::Key::*;
        match *button {
            Button::Keyboard(key) => match key {
                // Action button.
                Return => self.execute_main_menu(state),
                // Menu toggle.
                Tab => self.execute_open_menu(state),
                // Move.
                W | A | S | D => self.execute_move(state, &Some(key)),
                V => self.execute_action(state),
                L => {
                    if *state == ButtonState::Press {
                        self.player.take_damage(1)
                    }
                }

                F => {
                    self.change_player_location(state);
                    self.player.creature_state = CreatureState::Normal;
                }
                E => {
                    self.player.use_item();
                }
                Space => {
                    self.execute_player_hands(state);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn change_player_location(&mut self, state: &ButtonState) {
        if *state == ButtonState::Press {
            self.player_location = match self.player_location {
                PlayerLocation::OnShip => PlayerLocation::InWorld,
                PlayerLocation::InWorld => PlayerLocation::OnShip,
            }
        }
    }

    fn execute_player_hands(&mut self, state: &ButtonState) {
        if *state == ButtonState::Press {
            match self.player.inventory {
                Some(_) => {
                    let item = self.player.drop_item().expect("dropped empty inventory");
                    self.items_in_game.push(item);
                }
                None => {
                    let mut place = -1;
                    for i in 0..self.items_in_game.len() {
                        let diff_x = self.items_in_game[i].x - self.player.x;
                        let diff_y = self.items_in_game[i].y - self.player.y;
                        if diff_x < IMAGE_SIZE_SCALED && diff_x > -IMAGE_SIZE_SCALED
                            && diff_y < IMAGE_SIZE_SCALED
                            && diff_y > -IMAGE_SIZE_SCALED
                        {
                            place = i as i32;
                            break;
                        }
                    }
                    if place != -1 {
                        let item = self.items_in_game.remove(place as usize);
                        self.player.pickup_item(item);
                    }
                }
            }
        }
    }

    // Opens menu.
    fn execute_open_menu(&mut self, state: &ButtonState) {
        if *state == ButtonState::Press {
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

    fn execute_main_menu(&mut self, state: &ButtonState) {
        if *state == ButtonState::Press {
            match self.game_state {
                GameState::Title => {
                    println!("Changing state to InGame.");
                    self.game_state = GameState::InMenu;
                }
                _ => {}
            }
        }
    }

    // Moves creature / ship.
    fn execute_move(&mut self, state: &ButtonState, key: &Option<Key>) {
        if self.game_state == GameState::InGame {
            match self.player.creature_state {
                CreatureState::Normal => {
                    self.player.handle_input(&state, &key);
                    self.player.update_self_velocity();
                }
                CreatureState::ControllingShip => {
                    self.ship.handle_input(&state, &key);
                    self.ship.update_self_velocity();
                }
            }
        }
    }

    // Change of player's control state.
    fn execute_action(&mut self, state: &ButtonState) {
        if self.game_state == GameState::InGame {
            if *state == ButtonState::Press {
                self.player.change_control_state();
                self.ship.reset_dir();
            }
        }
    }
}

fn generate_item_prototypes() -> HashMap<String, Item> {
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
    prototypes
}

fn generate_glyphs(window: &mut PistonWindow) -> Glyphs {
    let assets = Search::ParentsThenKids(3, 3)
        .for_folder("fonts")
        .expect("Error finding folder");
    let ref font = assets.join("m5x7.ttf");
    let factory = window.factory.clone();
    let glyphs = Glyphs::new(font, factory, TextureSettings::new()).expect("Error with glyphs");
    glyphs
}

fn generate_textures(window: &mut PistonWindow) -> HashMap<String, G2dTexture> {
    // Collect the graphics ("textures").
    let assets = Search::ParentsThenKids(3, 3)
        .for_folder("images")
        .expect("Error finding folder");
    let image_names = [
        IMG_SKY,
        IMG_CLOUD_1,
        IMG_CLOUD_2,
        IMG_WOOD_FLOOR,
        IMG_WHEEL,
        IMG_TREE,
        IMG_GRUNE,
        IMG_PORTAL,
        IMG_HEART,
        IMG_ITEM_BISKET,
        IMG_ITEM_SWORD,
        IMG_ITEM_GRUNE,
        IMG_ITEM_LOGS,
        IMG_STONE_WALL,
        IMG_STONE_WALL_TEXTURE,
        IMG_STONE_WALL_EDGE_1_SIDE,
        IMG_STONE_WALL_EDGE_2_SIDE,
        IMG_STONE_WALL_EDGE_3_SIDE,
        IMG_STONE_WALL_EDGE_CORNER,
        IMG_STONE_WALL_FRONT,
        IMG_STONE_WALL_FRONT_R_EDGE,
        IMG_STONE_WALL_FRONT_L_EDGE,
        IMG_STONE_WALL_FRONT_DEEP,
        IMG_WATER,
        IMG_WATER_TEXTURE,
        IMG_GRASS_FLOOR,
        IMG_GRASS_FLOOR_TEXTURE,
        IMG_DIRT_FLOOR,
        IMG_DIRT_FLOOR_TEXTURE,
        IMG_GRASS_DIRT_FLOOR_1_SIDE,
        IMG_GRASS_DIRT_FLOOR_2_SIDE,
        IMG_GRASS_DIRT_FLOOR_3_SIDE,
        IMG_GRASS_DIRT_FLOOR_4_SIDE,
        IMG_GRASS_DIRT_FLOOR_CORNER,
        IMG_TITLE_NO_TEXT,
        IMG_TITLE_TEXT,
        IMG_PLAYER_IDLE_S_0,
        IMG_PLAYER_IDLE_S_1,
        IMG_PLAYER_IDLE_S_2,
        IMG_PLAYER_IDLE_N_0,
        IMG_PLAYER_IDLE_N_1,
        IMG_PLAYER_IDLE_N_2,
        IMG_PLAYER_IDLE_E_0,
        IMG_PLAYER_IDLE_E_1,
        IMG_PLAYER_IDLE_E_2,
        IMG_PLAYER_IDLE_W_0,
        IMG_PLAYER_IDLE_W_1,
        IMG_PLAYER_IDLE_W_2,
        IMG_PLAYER_MOVING_S_0,
        IMG_PLAYER_MOVING_S_1,
        IMG_PLAYER_MOVING_S_2,
        IMG_PLAYER_MOVING_N_0,
        IMG_PLAYER_MOVING_N_1,
        IMG_PLAYER_MOVING_N_2,
        IMG_PLAYER_MOVING_E_0,
        IMG_PLAYER_MOVING_E_1,
        IMG_PLAYER_MOVING_E_2,
        IMG_PLAYER_MOVING_W_0,
        IMG_PLAYER_MOVING_W_1,
        IMG_PLAYER_MOVING_W_2,
    ];

    let mut textures: HashMap<String, G2dTexture> = HashMap::new();

    let ts = TextureSettings::new().filter(Filter::Nearest);

    for image_name in image_names.into_iter() {
        let filename = image_name.to_owned().to_owned() + ".png";
        let img = Texture::from_path(
            &mut window.factory,
            assets.join(filename.clone()),
            Flip::None,
            &ts,
        ).expect(&format!("Not found: {:?}", filename));

        textures.insert(image_name.to_string(), img);
    }

    // // Import all player sprites
    // let dirs = ["N", "W", "S", "E"];
    // for j in 0..dirs.len() {
    //     for i in 1..4 {
    //         let filename = format!("{}{}{}{}{}", "player_idle", dirs[j], "_", i.to_string(), ".png");
    //         let mut map_name = format!("{}{}{}{}", "player_idle", dirs[j], "_", i.to_string());
    //         let sprite = Texture::from_path(
    //             &mut window.factory,
    //             assets.join(&filename),
    //             Flip::None,
    //             &TextureSettings::new(),
    //         ).expect(&format!("Not found: {:?}", filename));
    //         textures.insert(map_name, sprite);
    //     }
    // }
    textures
}
