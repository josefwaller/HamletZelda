extern crate piston_window;

use piston_window::{
	
	// arguments for updating
	UpdateArgs,
	
	// arguments for rendering
	RenderArgs
};

// The main app struct which contains all entities 
pub struct App {
	
}

impl App {
	
	// creates a new app
	pub fn new() -> App {
		App {}
	}
	
	// updates positions, status, etc
	pub fn update(&mut self, u: &UpdateArgs) {
		
	}
	
	// render all entities on the window
	pub fn render(&mut self, r: &RenderArgs) {
		
	}
}