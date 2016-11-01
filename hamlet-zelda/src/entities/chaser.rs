extern crate piston_window;

use piston_window::{
	UpdateArgs
};

use traits::has_bbox::HasBBox;
use utilities::bbox::BBox;

use traits::has_sprite::HasSprite;
use traits::is_enemy::IsEnemy;

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

impl Chaser {
	
	pub fn new(x: f64, y: f64) -> Chaser {
		Chaser {
			x: x,
			y: y,
			w: 50.0,
			h: 50.0,
			
			speed: 20.0,
			
			direction: 0
		}
	}
	
	fn update (&mut self, u: &UpdateArgs) {
		
	}
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

/*
see traits/get_sprite.rs
*/
impl HasSprite for Chaser {
	
	fn get_sprite(&mut self) -> String {
		
		String::from("Hello world!")
	}
	
	fn get_debug_color(&mut self) -> [f32; 4] {
		
		[0.0, 1.0, 0.0, 1.0]
		
	}
}

/*
see traits/is_enemy.rs
*/
impl IsEnemy for Chaser {
	
}