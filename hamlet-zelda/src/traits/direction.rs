/*
Contains integer representations of 
different directions for entities.
Ex: Helps determine what is hit by
the player's sword based on what 
direction he is facing
*/
pub trait Direction {
	
	//Integer representations of different directions
	#[allow(non_snake_case)]
	fn UP(&mut self) -> u8 {0}
	#[allow(non_snake_case)]
	fn DOWN(&mut self) -> u8 {1}
	#[allow(non_snake_case)]
	fn LEFT(&mut self) -> u8 {2}
	#[allow(non_snake_case)]
	fn RIGHT(&mut self) -> u8 {3}
	
	// returns the integer version of the current direction
	fn get_direction(&mut self) -> u8;
	
	// sets the direction
	fn set_direction(&mut self, d: u8);
	
	// turns the direction left or right
	fn turn_left(&mut self) {
		
		let direction = self.get_direction();
		
		if direction == self.UP() {	
			let dir = self.LEFT(); 
 			self.set_direction(dir);
			
		} else if direction == self.LEFT() {
			let dir = self.DOWN(); 
 			self.set_direction(dir);
			
		} else if direction == self.DOWN() {
			let dir = self.RIGHT(); 
 			self.set_direction(dir);
			
		} else {
			let dir = self.UP(); 
 			self.set_direction(dir);
			
		}
		
	}
	
	fn turn_right(&mut self) {
		
		let direction = self.get_direction();
		
		if direction == self.UP() {			
			let dir = self.RIGHT(); 
 			self.set_direction(dir);
			
		} else if direction == self.RIGHT() {
			let dir = self.DOWN(); 
 			self.set_direction(dir);
			
		} else if direction == self.DOWN() {
			let dir = self.LEFT(); 
 			self.set_direction(dir);
			
		} else {
			let dir = self.UP(); 
 			self.set_direction(dir);
			
		}
	}
}