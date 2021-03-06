extern crate piston_window;
extern crate time;

use piston_window::{
	UpdateArgs
};

use traits::position::Position;
use utilities::bbox::BBox;

use traits::has_sprite::HasSprite;
use traits::is_enemy::IsEnemy;

use entities::player::Player;

// see traits/direction.rs
use traits::direction::Direction;

// see traits/automove.rs
use traits::automove::AutoMove;

// see traits/patrol.rs
use traits::patrol::Patrol;

// see traits/search.rs
use traits::search::Search;

// used to time the enemy's actions
use time::Timespec;

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
	
	// its speed when chasing the player
	chase_speed: f64,
	
	// whether the enemy is chasing the player
	is_chasing_player: bool,
	
	// which direction it's looking
	direction: u8,
	
	// the points to walk between
	patrol: [[f64; 2]; 2],
	
	// the index of the current point it is walking to
	patrol_index: usize,
	
	// the time at which the enemy reached one of its
	// patrol points and started looking around
	look_time: time::Timespec,
	
	// the time the enemy should look around
	look_duration: i64,
	
	// the current step the looking action is on
	// goes left, forward, right, forward
	look_step: u8,
	
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
	pub fn new(patrol: [[f64; 2]; 2]) -> Chaser {
		Chaser {
			x: patrol[0][0],
			y: patrol[0][1],
			w: 50.0,
			h: 50.0,
			
			speed: 60.0,
			chase_speed: 20.0,
			is_chasing_player: false,
			
			view: 200.0,
			
			patrol: patrol,
			patrol_index: 0,
			
			look_time: Timespec::new(0, 0),
			look_duration: 4,
			look_step: 0,
			
			// this wil be immediantly changed so that the enemy is
			// facing one of its patrol points
			direction: 0
		}
	}
	
	/*
	Moves the Enemy towards the player
	
	bbox: The Bounding Box of the player
	u: The Update Args
	*/
	fn chase_player(&mut self, bbox: &BBox, u: &UpdateArgs) {
		
		let speed = self.chase_speed.clone();
		self.move_to_point(bbox.x + bbox.w / 2.0, bbox.y + bbox.h / 2.0, speed, &u);
		
	}
}

// see traits/search.rs
impl Search for Chaser {
	fn get_view(&mut self) -> f64 {
		self.view
	}
}

// see traits/automove.rs
impl AutoMove for Chaser {}

// see traits/direction.rs
impl Direction for Chaser {
	fn get_direction(&mut self) -> u8 { self.direction }
	fn set_direction(&mut self, d: u8) {
		self.direction = d;
	}
}

// see traits/position.rs
impl Position for Chaser {
	
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
	
	fn set_position(&mut self, x: f64, y: f64) {
		self.x = x;
		self.y = y;
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
		
		// returns a different direction depending on which direction the enemy is facing
		if self.direction == self.UP() {
			// green
			return [0.0, 1.0, 0.0, 1.0];
			
		} else if self.direction == self.LEFT() {
			// purple
			return [1.0, 0.0, 1.0, 1.0];
			
		} else if self.direction == self.RIGHT() {
			// white
			return [1.0, 1.0, 1.0, 1.0];
			
		} else {
			// yellow
			return [1.0, 1.0, 0.0, 1.0];
		}
		
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
		if self.can_see_bbox(&bbox) {
			
			// chases the player
			self.chase_player(&bbox, &u);
			self.is_chasing_player = true;
			
		} else {
			
			// checks if the enemy was chasing the player last
			// if so, the enemy should look around
			if self.is_chasing_player {
				self.is_chasing_player = false;
				self.look_time = time::now().to_timespec();
			}
			
			// walks between the two patrol points
			self.patrol(&u);
			
		}
		
	}
}

/*
see traits/patrol.rs
*/
impl Patrol for Chaser {
	
	fn get_look_duration(&mut self) -> i64 {
		self.look_duration
	}
	fn get_look_time(&mut self) -> time::Timespec {
		self.look_time
	}
	fn get_look_step(&mut self) -> u8 {
		self.look_step
	}
	fn get_patrol(&mut self) -> [[f64; 2]; 2] {
		self.patrol
	}
	fn get_patrol_index(&mut self) -> usize {
		self.patrol_index
	}
	fn get_patrol_speed(&mut self) -> f64 {
		self.speed
	}
	fn set_look_time(&mut self, l: time::Timespec) {
		self.look_time = l;
	}
	fn set_look_step(&mut self, s: u8) {
		self.look_step = s;
	}
	fn set_patrol_index(&mut self, s: usize) {
		self.patrol_index = s;
	}
	
}