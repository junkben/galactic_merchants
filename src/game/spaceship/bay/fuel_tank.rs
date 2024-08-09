#[derive(Debug, Clone, Copy)]
pub struct FuelTank {
	current:  u32,
	capacity: u32
}

impl super::SpaceshipBay for FuelTank {
	fn name(&self) -> &'static str { "fuel_tank" }

	fn new(capacity: u32) -> Self {
		Self {
			current: capacity,
			capacity
		}
	}
}

impl super::Container for FuelTank {
	fn units(&self) -> &'static str { "ton(s)" }

	fn current(&self) -> u32 { self.current }

	fn capacity(&self) -> u32 { self.capacity }

	fn set_current(&mut self, n: u32) { self.current = n }

	fn set_capacity(&mut self, n: u32) { self.capacity = n }
}
