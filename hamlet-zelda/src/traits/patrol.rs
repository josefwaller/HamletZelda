extern crate time;
extern crate piston_window;
use piston_window::UpdateArgs;

use traits::direction::Direction;
use traits::position::Position;
use traits::automove::AutoMove;

/*
An entity that has a patrol 
represented by points which
it walks between
*/
pub trait Patrol: Direction + Position + AutoMove {
	
	/*
	Patrols between the enemy's two patrol points
	
	u: The UpdateArgs
	*/
	fn patrol(&mut self, u: &UpdateArgs) {
		
						
		// checks if the enemy is done looking around
		if time::now().to_timespec().sec - self.get_look_time().sec >= self.get_look_duration() {
			
			// gets the point it needs to walk to
			let point = self.get_patrol()[self.get_patrol_index()];
			
			// moves towards the point
			let speed = self.get_patrol_speed().clone();
			self.move_to_point(point[0], point[1], speed, &u);
			
			// checks if the enemy has reached the point
			let pos = self.get_bbox();
			if pos.x < point[0] {
				if pos.x + pos.w > point[0] {
					if pos.y < point[1] {
						if pos.y + pos.h > point[1] {
								
							// moves to the next point
							let index = (self.get_patrol_index() + 1) % self.get_patrol().len();
							self.set_patrol_index(index);
							self.set_look_time(time::now().to_timespec());
							
						}
					}
				}
			}
			
		} else {
				
			// looks around
			self.look_around(&u);
		}
	}
	
	/*
	Causes the enemy to look around for the player
	Looks (from the perspective of the enemy)
	Right, Forward, Left, Forward
	
	u: The UpdateArgs
	*/
	fn look_around(&mut self, u: &UpdateArgs) {
	
		// gets the time elapsed
		let elapsed = time::now().to_timespec().sec - self.get_look_time().sec;
		
		// looks differently depending on how long it has been
		// the entire process is:
		// - Look Left
		// - Face Forward
		// - Look Right
		// - Face Forward
		match self.get_look_step() {
			0 => {
				
				// looks left
				if elapsed < self.get_look_duration() / 4 {
					
					self.turn_left();
					self.set_look_step(1);
				}
			},
			1 => {
				
				// faces forward
				if elapsed >= self.get_look_duration() / 4 {
					
					self.turn_right();
					self.set_look_step(2);
				}
			},
			2 => {
				
				// looks right
				if elapsed >= self.get_look_duration() / 2 {
					
					self.turn_right();
					self.set_look_step(3);
				}
			},
			3 => {
				
				// faces forward
				if elapsed >= self.get_look_duration() * 3 / 4 {
					
					self.turn_left();
					self.set_look_step(0);
				}
			},
			_ => {
				panic!("Error! Chaser Enemy has an invalid direct!! Oh No!");
			}
		}
		
	}
	
	// get and set functions needed for the trait to move the enemy
	fn get_look_duration(&mut self) -> i64;
	fn get_look_time(&mut self) -> time::Timespec;
	fn get_look_step(&mut self) -> u8;
	
	fn get_patrol(&mut self) -> [[f64; 2]; 2];
	fn get_patrol_index(&mut self) -> usize;
	fn get_patrol_speed(&mut self) -> f64;
	
	fn set_look_time(&mut self, l: time::Timespec);
	fn set_look_step(&mut self, s: u8);
	fn set_patrol_index(&mut self, s: usize);
}