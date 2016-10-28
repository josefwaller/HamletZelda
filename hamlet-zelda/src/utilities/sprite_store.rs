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
	PistonWindow,

	// used to draw images
	Graphics,
	Context,
	rectangle
	
};

// used to load the texture from its path
use std::path::Path;

// used to hold the textures
use std::collections::HashMap;

// used to position and scale the sprites
use traits::has_bbox::HasBBox;
use utilities::bbox::BBox;

// used to make sure the entity has a sprite
use traits::has_sprite::HasSprite;


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
	Renders an entity on the screen
	
	entity: The entity to render
		-> Must be able to get a BBox and sprite key from it
	c: The context
	g: The Graphics
	*/
	pub fn render_entity<T: HasBBox + HasSprite, G: Graphics>(&mut self, entity: &mut T, c: Context, g: &mut G) {

		// gets the bounding box and sprite to draw
		let bbox: BBox = entity.get_bbox();
		let sprite = entity.get_sprite();

		// draws a white rectangle in debug mode		
		rectangle(
			[1.0; 4],
			[bbox.x, bbox.y, bbox.w, bbox.h],
			c.transform,
			g
		);

	}
}