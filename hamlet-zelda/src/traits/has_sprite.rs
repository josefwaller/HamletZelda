/*
Whether an object has a sprite to render
Used by SpriteStore to check if it can
render something or not
*/
pub trait HasSprite {
	
	fn get_sprite(&mut self) -> String;
}