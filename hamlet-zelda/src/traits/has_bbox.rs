
use utilities::bbox::BBox;

/*
A trait used to determine if the entity has position and 
dimensions. Returns a BBox struct for comparing entities
*/
pub trait HasBBox {
	fn get_bbox(&mut self) -> BBox;
}