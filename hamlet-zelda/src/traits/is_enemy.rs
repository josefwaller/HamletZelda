extern crate piston_window;
use piston_window::UpdateArgs;

// see has_bbox.rs and has_sprite.rs
use traits::position::Position;
use utilities::bbox::BBox;
use traits::has_sprite::HasSprite;

// see player.rs
use entities::player::Player;

/*
Enemy trait
Implemented by all enemies, and contains
functions they all need
Also forces them to implement HasBBox and 
HasSprite so that they can be rendered
*/
pub trait IsEnemy: Position + HasSprite {
	
	// updates the enemy
	fn update(&mut self, u: &UpdateArgs, p: &mut Player) {
		
	}
}