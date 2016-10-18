extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs
};

// the player struct
pub struct Player {
	
}

impl Player {
	
	// creates a new player
	pub fn new() -> Player {
		Player{}
	}
	
	// updates the player
	pub fn update(&mut self, u: &UpdateArgs) {
		
	}
	
	// renders the player
	pub fn render(&mut self, r: &RenderArgs) {
		
	}
}