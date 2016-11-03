extern crate piston_window;
extern crate time;
extern crate gfx_device_gl;

use piston_window::{
	
	// used for creating the window
	PistonWindow,
	WindowSettings,
	
	// used in the game loop
	RenderEvent,
	UpdateEvent,
	PressEvent,
	ReleaseEvent,
	
	// used to compute input
	Button,
	
	// the opengl
	OpenGL,

	rectangle
};

// the entities
mod entities;

// the utilities (structs that do not represent an entity)
mod utilities;

// the traits
mod traits;

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
	
	// loads all the images before starting the game loop
	app.load_images(&mut window);
	
	// cycles through window events
	while let Some(e) = window.next() {
		
		// when a key is pressed
		if let Some(k) = e.press_args() {
		
			// checks that the key is on the keyboard
			// as opposed to a mouse button
			if let Button::Keyboard(p) = k {
					
				// processes the keypress
				app.on_key_down(p.code());	
			}
		}
		
		// when a key is released
		if let Some(k) = e.release_args() {
			
			// checks that the key way on the keyboard
			if let Button::Keyboard(p) = k {
				
				// processes the key release
				app.on_key_up(p.code());
			}
			
		}
		
		// when the game should update
		if let Some(u) = e.update_args() {
			
			// updates the game
			app.update(&u);
		}
		
		// when the game should render
		if let Some(r) = e.render_args() {
			
			window.draw_2d(&e, |c, g|{

				// renders the game
				app.render(&r, c, g);
				
			});
		}
		
	}
}
