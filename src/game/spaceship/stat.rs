#[derive(Debug, Clone, Copy)]
pub struct SpaceshipStat {
	pub name:          &'static str,
	pub description:   &'static str,
	pub cargo_size:    u32,
	pub crew_size:     u32,
	pub engine_power:  u32,
	pub fuel_tank:     u32,
	pub passenger_max: u32
}
