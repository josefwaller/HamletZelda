extern crate piston_window;

use piston_window::UpdateArgs;

use traits::direction::Direction;
use traits::position::Position;

/*
Automatically moves an entity with 
the 'move_to_point' method
*/
pub trait AutoMove: Direction + Position {
	
	/*
	Moves the entity towards a point with the speed given
	
	x: The X coordinate of the point
	y: The Y coordinate of the point
	speed: The speed at which to move the enemy
	u: The Update Args
	*/
	fn move_to_point(&mut self, x: f64, y: f64, speed: f64, u: &UpdateArgs) {
			
		// gets the BBox to use for position
		let pos = self.get_bbox();
			
		// gets the difference between the point and enemy
		let diff_x = x - (pos.x + pos.w / 2.0);
		let diff_y = y - (pos.y + pos.h / 2.0);
		
		// gets the angle	
		let theta = f64::atan(diff_y / diff_x);
		
		// finds how much the enemy can move in each coordinate
		// without going over their speed
		let mut x = speed * f64::cos(theta);
		let mut y = speed * f64::sin(theta);
		
		// if the enemy is on the right of the point,
		// atan will still return a positive number so
		// the enemy needs to run the other way
		if diff_x < 0.0 {
			x *= -1.0;
			y *= -1.0;
		}
		
		// moves the enemy
		self.set_position(pos.x + x * u.dt, pos.y + y * u.dt);
		
		// checks if the enemy needs to change direction
			
		// checks if the point is closer to directly above/below the enemy
		if diff_x.abs() < diff_y.abs() {
			
			// checks if the point is above or below
			if diff_y > 0.0 {
				let direction = self.DOWN();
				self.set_direction(direction);
				
			} else {
				let direction = self.UP();
				self.set_direction(direction);
			}
			
		} else {
			// the player is closer to directly right/left than up/down
			
			// checks if the player is left or right
			if diff_x > 0.0 {
				let direction = self.RIGHT();
				self.set_direction(direction);
				
			} else {
				let direction = self.LEFT();
				self.set_direction(direction);
			}
		}
		
	}
}