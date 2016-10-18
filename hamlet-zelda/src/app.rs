extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs
};


// the player
use entities::player::Player;

// the SpriteStore object
use utilities::sprite_store::SpriteStore;

// The main app struct which contains all entities 
pub struct App {
	
	// the player
	player: Player,
	
	// the SpriteStore
	sprite_store: SpriteStore
}

impl App {
	
	// creates a new app
	pub fn new() -> App {
		App {
			// creates a new player
			player: Player::new(),
			
			// creates a new spritestore
			sprite_store: SpriteStore::new()
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