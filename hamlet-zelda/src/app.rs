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

/*
The main app struct which contains all entities  
Runs their update and render functions as well as 
some other game logic
*/
pub struct App {
	
	// the player
	player: Player,
	
	// the SpriteStore
	sprite_store: SpriteStore
}

impl App {
	
	/*
	creates a new app
	
	Returns: A new App
	*/
	pub fn new() -> App {
		App {
			// creates a new player
			player: Player::new(),
			
			// creates a new spritestore
			sprite_store: SpriteStore::new()
		}
	}
	
	/*
	updates positions, status, etc
	
	u: UpdateArgs from the event
	*/
	pub fn update(&mut self, u: &UpdateArgs) {
		
		// updates the player
		self.player.update(u);
	}
	
	/*
	render all entities on the window
	
	r: RenderArgs from the event
	*/
	pub fn render(&mut self, r: &RenderArgs) {
		
		// renders the player
		self.player.render(r);
		
	}
}