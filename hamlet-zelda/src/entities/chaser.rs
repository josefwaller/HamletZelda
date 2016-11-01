extern crate piston_window;

use piston_window::{
	UpdateArgs
};

use traits::has_bbox::HasBBox;
use utilities::bbox::BBox;

use traits::has_sprite::HasSprite;
use traits::is_enemy::IsEnemy;

use entities::player::Player;

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
	
	fn update(&mut self, u: &UpdateArgs, p: &mut Player) {
		
		// gets the player's position
		let bbox = p.get_bbox();
		
		// getsthe difference between the player and enemy
		let diff_x = bbox.x - self.x;
		let diff_y = bbox.y - self.y;
		
		// gets the angle	
		let theta = f64::atan(diff_y / diff_x);
		
		// finds how much the enemy can move in each coordinate
		// without going over their speed
		let mut x = self.speed * f64::cos(theta);
		let mut y = self.speed * f64::sin(theta);
		
		// if the enemy is on the right of the player,
		// atan will still return a positive number so
		// the enemy needs to run the other way
		if diff_x < 0.0 {
			x *= -1.0;
			y *= -1.0;
		}
		
		// moves the enemy
		self.x += x * u.dt;
		self.y += y * u.dt;
		
	}
}