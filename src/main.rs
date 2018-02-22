/**
    Aether of Enclaves will be a 32-bit exploration game, in which the user 
    controls a main character and an airship and travels through the sky - 
    picking up crew members, discovering new islands, interacting with NPCs, 
    and exploring.

    V 0.0.0

    2018 Samuel Eubanks, McKenzie Weller
*/

extern crate piston;
extern crate piston_window;
use piston_window::*;
mod input_handler;

fn main() {
    let mut input_hnd = input_handler::InputHandler::new();

    let mut window: PistonWindow = WindowSettings::new("AOE", (200, 200))
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args(){
            input_hnd.handle_input(button);
        }
    }
}
