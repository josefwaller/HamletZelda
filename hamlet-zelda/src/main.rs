extern crate piston_window;

use piston_window::{
	
	// used for creating the window
	PistonWindow,
	WindowSettings,
	
	// used in the game loop
	RenderEvent,
	UpdateEvent,
	PressEvent,
	ReleaseEvent,
	
	// the opengl
	OpenGL
};

// the entities
mod entities;

// the main app struct
// see app.rs
mod app;
use app::App;

fn main() {
    
	// initializes the OpenGL pointers
	let opengl = OpenGL::V3_2;
	
	// creates the window
	let mut window: PistonWindow = WindowSettings::new("The Legend Of Zelda: Hamlet", [1000, 800])
	// sets to exit on esc
	.exit_on_esc(true)
	// sets opengl to use
	.opengl(opengl)
	// builds and unwraps the window
	.build()
	.unwrap();
	
	// creates the app
	let mut app = App::new();
	
	// cycles through window events
	while let Some(e) = window.next() {
		
		// when a key is pressed
		if let Some(k) = e.press_args() {
			
		}
		
		// when a key is released
		if let Some(k) = e.release_args() {
			
		}
		
		// when the game should update
		if let Some(u) = e.update_args() {
			
			// updates the game
			app.update(&u);
		}
		
		// when the game should render
		if let Some(r) = e.render_args() {
			
			// renders the game
			app.render(&r);
		}
		
	}
}
