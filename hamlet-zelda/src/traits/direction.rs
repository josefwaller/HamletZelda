/*
Contains integer representations of 
different directions for entities.
Ex: Helps determine what is hit by
the player's sword based on what 
direction he is facing
*/
pub trait Direction {
	
	//Integer representations of different directions
	fn UP(&mut self) -> u8 {0}
	fn DOWN(&mut self) -> u8 {1}
	fn LEFT(&mut self) -> u8 {2}
	fn RIGHT(&mut self) -> u8 {3}
}