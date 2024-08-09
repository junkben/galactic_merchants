#[derive(Debug, Clone, Copy)]
pub struct CargoBay {
	current:  u32,
	capacity: u32
}

impl super::SpaceshipBay for CargoBay {
	fn name(&self) -> &'static str { "cargo_bay" }

	fn new(capacity: u32) -> Self {
		Self {
			current: u32::MIN,
			capacity
		}
	}
}

impl super::Container for CargoBay {
	fn units(&self) -> &'static str { "ton(s)" }

	fn current(&self) -> u32 { self.current }

	fn capacity(&self) -> u32 { self.capacity }

	fn set_current(&mut self, n: u32) { self.current = n }

	fn set_capacity(&mut self, n: u32) { self.capacity = n }
}
