use traits::has_bbox::HasBBox;
use utilities::bbox::BBox;

use traits::has_sprite::HasSprite;

/*
An Enemy that chases the player if it sees him
*/
pub struct Chaser {
	
	// its position and dimensions
	x: f64,
	y: f64,
	w: f64,
	h: f64,
	
	// its speed
	speed: f64,
	
	// which direction it's looking
	direction: u8

}

impl HasBBox for Chaser {
	
	/*
	see traits/get_bbox.rs
	*/
	fn get_bbox(&mut self) -> BBox {
		
		BBox::new(
			self.x,
			self.y,
			self.w,
			self.h
		)
	}
}

impl HasSprite for Chaser {
	
	/*
	see traits/get_sprite.rs
	*/
	fn get_sprite (&mut self) -> String {
		
		String::from("Hello world!")
	}
}