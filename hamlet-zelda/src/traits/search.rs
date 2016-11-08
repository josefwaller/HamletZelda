use utilities::bbox::BBox;

use traits::position::Position;
use traits::direction::Direction;
/*
Contains a function checking whether the entity
can see a given viewbox
*/
pub trait Search: Position + Direction {
	
	
	/* 
	Checks if the enemy can see the bbox
	
	bbox: The Bounding Box to look for
	*/
	fn can_see_bbox(&mut self, bbox: &BBox) -> bool{
		
		// first checks if the player is within its view in any direction
		let pos = self.get_bbox();
		let view = self.get_view();
		if (bbox.x - pos.x).abs() < view {
			if (bbox.y - pos.y).abs() < view {
				
				// checks the enemy is looking in the right direction
				let direction = self.get_direction();
				if direction == self.UP() {
					
					if bbox.y < pos.y {
						return true
					}
					
				} else if direction == self.DOWN() {
					
					if bbox.y > pos.y {
						return true
					}
					
				} else if direction == self.RIGHT() {
					
					if bbox.x > pos.x {
						return true
					}
					
				} else if direction == self.LEFT() {
					
					if bbox.x < pos.x {
						return true
					}
				}
				
			}
		}
		
		false
	}
	
	// returns how far the entity can see
	fn get_view(&mut self) -> f64;
}