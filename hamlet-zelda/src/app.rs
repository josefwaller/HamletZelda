extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs
};


// uses the player
use entities::player::Player;

// The main app struct which contains all entities 
pub struct App {
	
	// the player
	player: Player
}

impl App {
	
	// creates a new app
	pub fn new() -> App {
		App {
			player: Player::new()
		}
	}
	
	// updates positions, status, etc
	pub fn update(&mut self, u: &UpdateArgs) {
		
		// updates the player
		self.player.update(u);
	}
	
	// render all entities on the window
	pub fn render(&mut self, r: &RenderArgs) {
		
		// renders the player
		self.player.render(r);
		
	}
}