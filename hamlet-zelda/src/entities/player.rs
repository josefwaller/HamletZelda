extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs,

	// just passed down to the spritestore for rendering
	Graphics,
	Context,
	
	Key
};

// used to process key movements
use std::vec::Vec;

// see traits/has_sprite.rs
use traits::has_sprite::HasSprite;

// see traits/has_bbox.rs
use traits::has_bbox::HasBBox;

// used for HasBBox
use utilities::bbox::BBox;

// used for rendering
use utilities::sprite_store::SpriteStore;

/*
The player struct. Responsible for 
moving, rendering and anything else 
concerning the player.
*/
pub struct Player {
	
	// the position of the player
	x: f64,
	y: f64,
	w: f64,
	h: f64,
	
	// the speed
	speed: f64
}

impl Player {
	
	/* 
	Creates a new player
	
	Returns: A new Player object
	*/
	pub fn new() -> Player {
		Player{
			x: 0.0,
			y: 0.0,
			w: 50.0,
			h: 50.0,
			
			speed: 50.0
		}
	}
	
	/*
	Updates the player
	
	u: UpdateArgs from the window event
	*/
	pub fn update(&mut self, u: &UpdateArgs, keys: &Vec<i32>) {
		
		for key in keys.iter() {
			
			// checks if each key is currently pressed
			if key == &Key::Left.code() {
				
				// moves the player over
				self.x -= self.speed * u.dt;
			
			// repeats for each key	
			} else if key == &Key::Right.code() {
				self.x += self.speed * u.dt;
				
			} else if key == &Key::Up.code() {
				self.y -= self.speed * u.dt;
				
			} else if key == &Key::Down.code() {
				self.y += self.speed * u.dt;
				
			}
			
		}
		
	}
}

impl HasBBox for Player {
	
	/*
	See hes_bbox.rs
	*/
	fn get_bbox(&mut self) -> BBox {
		
		// returns a Bounding box of the player
		BBox::new(
			self.x,
			self.y,
			self.w,
			self.h
		)
	}
}

impl HasSprite for Player {
	
	/*
	See has_sprite.rs
	*/
	fn get_sprite(&mut self) -> String {
		
		// temporarily returns a nonsense string
		// will return a sprite key when sprites are
		// implemented
		String::from("Hello World!")
	}
}