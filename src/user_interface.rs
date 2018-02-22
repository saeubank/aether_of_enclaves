use piston_window::*;
use input_handler;


/**
	Implementation of the User Interface using Piston.

	@field window The interface window.
	@field input_hnd The Input Handler for the Interface.
*/
pub struct Interface {
	window: PistonWindow,
	input_hnd: input_handler::InputHandler,
}

impl Interface {
	// Constructor for the object. Creates a new window of
	// fixed size and a new Input Handler.
	pub fn new() -> Self {
		Interface {
			window: WindowSettings::new("AOE", (200, 200))
				.exit_on_esc(true)
		        .build()
		        .unwrap(),
		    input_hnd: input_handler::InputHandler::new(),
		}
	}

	// Updates the interface fields with input and other
	// game updates.
	// @return bool Whether updating continues.
	pub fn update(&mut self) -> bool {
		if let Some(e) = self.window.next() {
	        if let Some(button) = e.press_args(){
	            self.input_hnd.handle_input(button);

	            // Need to manually implement exit on escape.
	            if button == Button::Keyboard(Key::Escape) {
		        	return false;
		        }
	        }
	    }
	    true
	}


	pub fn display() {}
}