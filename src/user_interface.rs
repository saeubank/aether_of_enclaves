// use piston_window::*;
// use input_handler::{InputHandler};
// use game::{GameState, GAME_STATE, PLAYER};
//
// use self::GameState::*;
//
//
// /**
// 	Implementation of the User Interface using Piston.
//
// 	@field window The interface window.
// 	@field input_hnd The Input Handler for the Interface.
// */
// pub struct Interface {
// 	window: PistonWindow,
// 	input_hnd: InputHandler,
// }
//
// impl Interface {
// 	// Constructor for the object. Creates a new window of
// 	// fixed size and a new Input Handler.
// 	pub fn new() -> Self {
// 		Interface {
// 			window: WindowSettings::new("AOE", (400, 400))
// 				.exit_on_esc(true)
// 		        .build()
// 		        .unwrap(),
// 		    input_hnd: InputHandler::new(),
// 		}
// 	}
//
// 	// Updates the interface fields with input and other
// 	// game updates.
// 	// @return bool Whether updating continues.
// 	pub fn update(&mut self) -> bool {
// 		while let Some(e) = self.window.next() {
// 	        if let Some(button) = e.press_args(){
// 	            self.input_hnd.handle_input(button);
//
// 	            // Need to manually implement exit on escape.
// 	            if button == Button::Keyboard(Key::Escape) {
// 		        	return false;
// 		        }
// 	        }
// 	        self.display(e);
// 	        break;
// 	    }
// 	    true
// 	}
//
//
// 	// Displays the interface depending on Game State.
// 	// @param e PistonWindow Generic Event trait.
// 	pub fn display(&mut self, e: Event) {
// 		self.window.draw_2d(&e, |context, graphics| {
// 	        clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
// 	        unsafe {
// 				match GAME_STATE {
// 					Main => {
// 						// Draw Player.
// 						let red = [1.0, 0.0, 0.0, 1.0];
//            				let player_image = [PLAYER.x, PLAYER.y, 15.0, 15.0];
//            				rectangle(red, player_image, context.transform, graphics);
// 					},
// 					Title => {
//
// 					},
// 					_ => {}
// 				}
// 			}
//    	    });
// 	}
//
// }
