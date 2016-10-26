extern crate piston_window;

// used to load the textures
extern crate gfx_device_gl;

use piston_window::{
	
	// the texture object each sprite is stored as
	Texture,
	
	// the settings when creating/loading a texture
	TextureSettings,
	Flip,
	
	// used to load images (PistonWindow.Factory is required)
	PistonWindow
	
};

// used to load the texture from its path
use std::path::Path;

// used to hold the textures
use std::collections::HashMap;

/*
the SpriteStore object
loads all the sprites and returns references of them
used to make sure a sprite is not loaded twice
*/
pub struct SpriteStore {
	
	// a hashmap of images that have already been loaded
	// the key of each image wil be the path the image
	// was loaded from
	// Ex: loading the image at 'images/test.jpg' with 
	// save it with the key 'images/test.jpg'
	sprites: HashMap<String, Texture<gfx_device_gl::Resources>>
	
}

impl SpriteStore {
	
	/*
	creates a new SpriteStore object
	
	Returns: A new SpriteStore
	*/
	pub fn new() -> SpriteStore {
		
		SpriteStore {
			
			sprites: HashMap::new()
		}
	}
	
	/*
	Loads a new sprite, unless it has already been loaded
	The sprite key is the same as the path
	
	path: The path to the image to load
	*/
	pub fn add_sprite(&mut self, path: &str, w: &mut PistonWindow) {
		
		// adds a new sprite
		self.sprites.insert(
			
			// saves the key as the path to the sprite
			String::from(path),
			
			Texture::from_path(
				
				&mut w.factory,
				
				// creates a new path object ot load the image
				Path::new(path),
				
				// uses default settings
				Flip::None,
				&TextureSettings::new()
				
			).unwrap()
		);
	}
	
	/*
	Renders a sprite on the screen
	x: The x coordinate
	y: The y coordinate
	w: The width
	h: The height
	key: The key (or path) of the sprite
	*/
	pub fn render_sprite(&mut self, x: f64, y: f64, w: f64, h: f64, key: &str) {
		
	}
}