extern crate piston_window;
extern crate time;

// contains update information
use piston_window::UpdateArgs;

// see traits/mod.rs for corresponding file
use traits::direction::Direction;
use traits::has_sprite::HasSprite;
use traits::position::Position;
use traits::is_enemy::IsEnemy;
use traits::automove::AutoMove;
use traits::patrol::Patrol;
use traits::search::Search;

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
	speed: f64,
	
	// the direction the Archer is facing
	direction: u8,
	
	look_duration: i64,
	look_time: time::Timespec,
	look_step: u8,
	
	view: f64,
	
	patrol: [[f64; 2]; 2],
	patrol_index: usize
}

impl Archer {
	pub fn new(patrol: [[f64; 2]; 2]) -> Archer {
		
		Archer {
			x: patrol[0][0],
			y: patrol[0][1],
			w: 50.0,
			h: 50.0,
			speed: 40.0,
			direction: 0,
			
			look_duration: 4,
			look_time: time::now().to_timespec(),
			look_step: 0,
			
			view: 200.0,
			
			patrol: patrol,
			patrol_index: 0
		}
	}
}

// see traits/is_enemy.rs
impl IsEnemy for Archer {
	fn update(&mut self, u: &UpdateArgs, p: &mut Player) {
		
	}
}

// see traits/search.rs
impl Search for Archer {
	fn get_view(&mut self) -> f64 {
		self.view
	}
}

// see traits/automove.rs
impl AutoMove for Archer {}

// see traits/patrol.rs
impl Patrol for Archer {
	
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

// see traits/position.rs
impl Position for Archer {
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