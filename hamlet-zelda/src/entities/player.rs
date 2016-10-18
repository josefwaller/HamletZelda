extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs
};

/*
The player struct. Responsible for 
moving, rendering and anything else 
concerning the player.
*/
pub struct Player {
	
}

impl Player {
	
	/* 
	Creates a new player
	
	Returns: A new Player object
	*/
	pub fn new() -> Player {
		Player{}
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
	pub fn render(&mut self, r: &RenderArgs) {
		
	}
}