extern crate piston_window;
use piston_window::UpdateArgs;

// see has_bbox.rs and has_sprite.rs
use traits::has_bbox::HasBBox;
use utilities::bbox::BBox;
use traits::has_sprite::HasSprite;

/*
Enemy trait
Implemented by all enemies, and contains
functions they all need
Also forces them to implement HasBBox and 
HasSprite so that they can be rendered
*/
pub trait IsEnemy: HasBBox + HasSprite {
	
	// updates the enemy
	fn update(&mut self, u: &UpdateArgs) {
		
	}
}