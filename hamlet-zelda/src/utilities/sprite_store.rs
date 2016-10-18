extern crate piston_window;

// used to find the assets folder
extern crate find_folder;

use piston_window::{
	
	// the texture object each sprite is stored as
	Texture,
	
	// the settings when creating/loading a texture
	TextureSettings
	
};

// used to load the texture from its path
use std::path::Path;

/*
the SpriteStore object
loads all the sprites and returns references of them
used to make sure a sprite is not loaded twice
*/
pub struct SpriteStore {
	
}

impl SpriteStore {
	
	/*
	creates a new SpriteStore object
	
	Returns: A new SpriteStore
	*/
	pub fn new() -> SpriteStore {
		SpriteStore{}
	}
	
	/*
	loads a new sprite, unless it has already been loaded
	returns a string name which can be used to draw the sprite
	
	path: The path to the image to load
	*/
	pub fn add_sprite(&mut self, path: &str) -> String {
		String::from("Hello world!")
	}
	
	/*
	Renders a sprite on the screen
	x: The x coordinate
	y: The y coordinate
	w: The width
	h: The height
	*/
	pub fn render_sprite(&mut self, x: f64, y: f64, w: f64, h: f64) {
		
	}
}