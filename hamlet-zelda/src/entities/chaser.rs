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
	direction: u8,
	
	// the points to walk between
	patrol: [[f64; 2]; 2],
	
	// the index of the current point it is walking to
	patrol_index: usize,
	
	// how far it can look
	view: f64

}

impl Chaser {
	
	/*
	Returns a new Chaser Enemy
	x: The X coordinate to start at
	y: The Y coordinate to start at
	patrol: An array of coordinates to walk to
		Ex: [[0, 0], [1, 1]] would tell the chaser to patrol between (x=0, y=0) and (x=1, y=1)
	*/
	pub fn new(x: f64, y: f64, patrol: [[f64; 2]; 2]) -> Chaser {
		Chaser {
			x: x,
			y: y,
			w: 50.0,
			h: 50.0,
			
			speed: 20.0,
			view: 200.0,
			
			patrol: patrol,
			patrol_index: 0,
			direction: 0
		}
	}
	
	//Integer representations of different directions
	fn UP(&mut self) -> u8 {0}
	fn DOWN(&mut self) -> u8 {1}
	fn LEFT(&mut self) -> u8 {2}
	fn RIGHT(&mut self) -> u8 {3}
	
	/* 
	Checks if the enemy can see the player
	
	bbox: The Bounding Box of the player
	*/
	fn can_see_player(&mut self, bbox: &BBox) -> bool{
		
		// first checks if the player is within its view in any direction
		if (bbox.x - self.x).abs() < self.view {
			if (bbox.y - self.y).abs() < self.view {
				
				// checks the enemy is looking in the right direction
				if self.direction == self.UP() {
					
					if bbox.y < self.y {
						return true
					}
					
				} else if self.direction == self.DOWN() {
					
					if bbox.y > self.y {
						return true
					}
					
				} else if self.direction == self.RIGHT() {
					
					if bbox.x > self.x {
						return true
					}
					
				} else if self.direction == self.LEFT() {
					
					if bbox.x < self.x {
						return true
					}
				}
				
			}
		}
		
		false
	}
	
	/*
	Moves the Enemy towards the player
	
	bbox: The Bounding Box of the player
	u: The Update Args
	*/
	fn chase_player(&mut self, bbox: &BBox, u: &UpdateArgs) {
		
		let speed = self.speed.clone();
		self.move_to_point(bbox.x + bbox.w / 2.0, bbox.y + bbox.h / 2.0, speed, &u);
		
	}
	
	/*
	Patrols between the enemy's two patrol points
	
	u: The UpdateArgs
	*/
	fn patrol(&mut self, u: &UpdateArgs) {
		
		// gets the point it needs to walk to
		let point = self.patrol[self.patrol_index];
		
		
	}
	
	/*
	Moves the enemy towards a point with the speed given
	
	x: The X coordinate
	y: The Y coordinate
	speed: The speed at which to move the enemy
	u: The Update Args
	*/
	fn move_to_point(&mut self, x: f64, y: f64, speed: f64, u: &UpdateArgs) {
			
		// gets the difference between the point and enemy
		let diff_x = x - (self.x + self.w / 2.0);
		let diff_y = y - (self.y + self.h / 2.0);
		
		// gets the angle	
		let theta = f64::atan(diff_y / diff_x);
		
		// finds how much the enemy can move in each coordinate
		// without going over their speed
		let mut x = self.speed * f64::cos(theta);
		let mut y = self.speed * f64::sin(theta);
		
		// if the enemy is on the right of the point,
		// atan will still return a positive number so
		// the enemy needs to run the other way
		if diff_x < 0.0 {
			x *= -1.0;
			y *= -1.0;
		}
		
		// moves the enemy
		self.x += x * u.dt;
		self.y += y * u.dt;
		
		// checks if the enemy needs to change direction
			
		// checks if the point is closer to directly above/below the enemy
		if diff_x.abs() < diff_y.abs() {
			
			// checks if the point is above or below
			if diff_y > 0.0 {
				self.direction = self.DOWN();
			} else {
				self.direction = self.UP();
			}
			
		} else {
			// the player is closer to directly right/left than up/down
			
			// checks if the player is left or right
			if diff_x > 0.0 {
				self.direction = self.RIGHT();
			} else {
				self.direction = self.LEFT();
			}
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
		
		// checks if the chaser can see the player
		if self.can_see_player(&bbox) {
			
			// chases the player
			self.chase_player(&bbox, &u);
			
		} else {
			
			// walks between the two patrol points
			self.patrol(&u);
			
		}
		
	}
}