extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs,
	
	// used for loading images (see App::load_images())
	PistonWindow
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
	Loads all the images used in the game
	
	w: The PistonWindow object, which is used for its Factory attribute
	*/
	pub fn load_images(&mut self, w: &mut PistonWindow) {
		
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
	prcesses when a key is pressed
	
	code: The key code (see piston::input::keyboard::Key)
	*/
	pub fn on_key_down(&mut self, code: i32) {
		
	}
	
	/*
	processes when a key is released
	
	code: The key code
	*/
	pub fn on_key_up(&mut self, code: i32) {
		
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