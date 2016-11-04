extern crate piston_window;

// contains update information
use piston_window::UpdateArgs;

// see traits/mod.rs for corresponding file
use traits::direction::Direction;
use traits::has_sprite::HasSprite;
use traits::has_bbox::HasBBox;
use traits::is_enemy::IsEnemy;

// see utilities/bbox.rs
use utilities::bbox::BBox;

// see entities/player.rs
use entities::player::Player;

/*
The Archer Enemy. Moves to the same 
x or y coordinate as the player and 
shoots and arrow at him.
*/
pub struct Archer {
	x: f64,
	y: f64,
	w: f64,
	h: f64,
	
	// the direction the Archer is facing
	direction: u8
}

impl Archer {
	fn new(x: f64, y: f64) -> Archer {
		
		Archer {
			x: x,
			y: y,
			w: 50.0,
			h: 50.0,
			direction: 0
		}
	}
}

// see traits/is_enemy.rs
impl IsEnemy for Archer {
	fn update(&mut self, u: &UpdateArgs, p: &mut Player) {
		
	}
}

// see traits/has_bbox.rs
impl HasBBox for Archer {
	fn get_bbox(&mut self) -> BBox {
		BBox::new(
			self.x,
			self.y,
			self.w,
			self.h
		)
	}
}

// see traits/has_sprite.rs
impl HasSprite for Archer {
	fn get_sprite(&mut self) -> String {
		String::from("Hello World")
	}
	fn get_debug_color(&mut self) -> [f32; 4] {
		[0.0, 1.0, 1.0, 1.0]
	}
}

// see traits/direction.rs
impl Direction for Archer {
	
	fn get_direction(&mut self) -> u8 {self.direction}
	fn set_direction(&mut self, d: u8) {
		self.direction = d;
	}
	
}