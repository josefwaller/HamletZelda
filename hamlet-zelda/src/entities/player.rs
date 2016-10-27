extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs,

	// just passed down to the spritestore for rendering
	Graphics,
	Context
};

// see traits/has_bbox.rs
use traits::has_bbox::HasBBox;

// used for HasBBox
use utilities::bbox::BBox;
use utilities::sprite_store::SpriteStore;
/*
The player struct. Responsible for 
moving, rendering and anything else 
concerning the player.
*/
pub struct Player {
	x: f64,
	y: f64,
	w: f64,
	h: f64
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
			w: 20.0,
			h: 20.0
		}
	}
	
	/*
	Updates the player
	
	u: UpdateArgs from the window event
	*/
	pub fn update(&mut self, u: &UpdateArgs) {
		
	}
	
	/*
	Renders the player
	
	r: RenderArgs from the window event
	*/
	pub fn render<T: Graphics>(&mut self, r: &RenderArgs, ss: &mut SpriteStore, c: Context, g: &mut T) {
		
		ss.render_sprite(
			self.get_bbox(),
			"This will be a key eventually, but right now it can be anything",
			c,
			g
		);

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