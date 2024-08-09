#[derive(Debug, Clone, Copy)]
pub struct CrewQuarters {
	current:  u32,
	capacity: u32
}

impl super::SpaceshipBay for CrewQuarters {
	fn name(&self) -> &'static str { "crew_quarters" }

	fn new(capacity: u32) -> Self {
		Self {
			current: capacity,
			capacity
		}
	}
}

impl super::Container for CrewQuarters {
	fn units(&self) -> &'static str { "crew member(s)" }

	fn current(&self) -> u32 { self.current }

	fn capacity(&self) -> u32 { self.capacity }

	fn set_current(&mut self, n: u32) { self.current = n }

	fn set_capacity(&mut self, n: u32) { self.capacity = n }
}
