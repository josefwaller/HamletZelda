extern crate piston_window;
extern crate gfx_device_gl;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs,
	
	// used for loading images (see App::load_images())
	PistonWindow,

	// used for rendering
	Context,
	rectangle,
	Graphics,
	clear
};

// allows different types of enemies to be stored in the same array
use std::boxed::Box;

// common trait for all enemies
use traits::is_enemy::IsEnemy;

// position
use traits::position::Position;

// see entities/chaser.rs
use entities::chaser::Chaser;

// see entities/archer.rs
use entities::archer::Archer;

// the player
use entities::player::Player;

// the SpriteStore object
use utilities::sprite_store::SpriteStore;

// traits
use traits::has_sprite::HasSprite;

/*
The main app struct which contains all entities  
Runs their update and render functions as well as 
some other game logic
*/
pub struct App {
	
	// the player
	player: Player,
	
	// the SpriteStore
	sprite_store: SpriteStore,
	
	// the keys which are currently down
	keys: Vec<i32>,
	
	// the enemies
	enemies: Vec<Box<IsEnemy>>
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
			sprite_store: SpriteStore::new(),
			
			// creates a new vector
			keys: Vec::new(),
			
			enemies: vec![
				Box::new(Chaser::new([[200.0, 300.0], [400.0, 300.0]])),
				Box::new(Archer::new([[600.0, 200.0], [900.0, 200.0]]))]
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
		self.player.update(&u, &self.keys);
		
		// updates the enemies
		for e in &mut self.enemies {
			
			e.update(&u, &mut self.player);
		}
	}
	
	/*
	prcesses when a key is pressed
	
	code: The key code (see piston::input::keyboard::Key)
	*/
	pub fn on_key_down(&mut self, code: i32) {
		
		// checks this key is not already recorded as being pushed down
		match self.keys.iter().position(|&e| e == code) {
			
			// adds the key
			None => self.keys.push(code),
			_ => {}
			
		}
		
	}
	
	/*
	processes when a key is released
	
	code: The key code
	*/
	pub fn on_key_up(&mut self, code: i32) {
		
		// gets the index of the key
		match self.keys.iter().position(|&e| e == code) {
			Some(i) => {
				
				// removes the key
				self.keys.remove(i);
			}
			_ => {}
		}
		
	}
	/*
	render all entities on the window
	
	r: RenderArgs from the event
	c: The Context Object
	g: The Graphics Object
	*/
	pub fn render<T: Graphics>(&mut self, r: &RenderArgs, c: Context, g: &mut T) {

		// clears the screen
		clear([0.0; 4], g);

		// renders the player
		self.sprite_store.render_sprite(
			self.player.get_bbox(), 
			self.player.get_sprite(), 
			self.player.get_debug_color(), 
			c, g);
		
		// renders the enemies
		for e in &mut self.enemies {
			
			self.sprite_store.render_sprite(
				e.get_bbox(),
				e.get_sprite(),
				e.get_debug_color(),
				c, g);
		}
		
	}
}