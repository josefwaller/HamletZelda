/*
Contains functions needed to render something
*/
pub trait HasSprite {
	
	// returns the sprite key of the sprite to draw
	fn get_sprite(&mut self) -> String;
	
	// returns an rgba color to draw a rectangle in debug mode
	fn get_debug_color(&mut self) -> [f32; 4];
	
}