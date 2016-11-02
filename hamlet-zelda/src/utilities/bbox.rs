/*
A Bounding Box struct. It has and x and y coordinate,
as well as width and height. Its main purpose is to 
be used in the HasBBox trait (see traits/has_bbox.rs)
*/
pub struct BBox {
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64
}

impl BBox {
	
	/*
	Creates a new BBox with the information given
	
	x: The x coordinate
	y: The y coordinate
	w: The width
	h: The height
	*/
	pub fn new(x: f64, y: f64, w: f64, h: f64) -> BBox {
		BBox {
			x: x,
			y: y,
			w: w,
			h: h
		}
	}	
	
	/* 
	Checks if this BBox collides with anoth BBox
	
	bbox: The other BBox
	*/
	pub fn collides(&mut self, bbox: BBox) -> bool {
		
		if self.x < bbox.x + bbox.w {
			if self.x + self.w > bbox.x {
				if self.y < bbox.y + bbox.h {
					if self.y + self.h > bbox.y {
						return true
					}
				}
			}
		}
		
		false
		
	}
}