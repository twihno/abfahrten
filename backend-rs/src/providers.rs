pub mod common;
pub mod de;

pub trait Provider {
	fn get_station(id: String);
	fn get_departures(id: String);
}
