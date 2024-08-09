mod cargo;
mod crew;
mod engines;
mod fuel_tank;
mod guests;

use std::fmt::Debug;

pub use cargo::CargoBay;
pub use crew::CrewQuarters;
pub use engines::Engines;
pub use fuel_tank::FuelTank;
pub use guests::GuestQuarters;

pub trait Container: Sized + Debug {
	/// String representation of the units of volume
	fn units(&self) -> &'static str;
	/// Current volume
	fn current(&self) -> u32;
	/// Max volume
	fn capacity(&self) -> u32;
	/// Set new container volume
	fn set_current(&mut self, volume: u32);
	/// Set new container capacity
	fn set_capacity(&mut self, volume: u32);

	// Utility functions
	fn is_full(&self) -> bool { self.current() == self.capacity() }
	fn is_empty(&self) -> bool { self.current() == u32::MIN }
}

pub trait SpaceshipBay: Container {
	/// Make a new Container
	fn new(capacity: u32) -> Self;
	/// String representation of the spaceship bay
	fn name(&self) -> &'static str;
}
