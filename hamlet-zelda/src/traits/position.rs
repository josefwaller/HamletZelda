use utilities::bbox::BBox;

/*
A trait used to determine if the entity has position and 
dimensions. Returns a BBox struct for comparing entities
*/
pub trait Position {
	fn set_position(&mut self, x: f64, y: f64);
	fn get_bbox(&mut self) -> BBox;
}