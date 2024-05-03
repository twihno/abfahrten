use chrono::{DateTime, TimeZone};

pub struct Departure<Tz: TimeZone> {
	destination: String,
	line_name: String,
	line_id: String,
	track: String,
	track_label: String,
	departure_planned: DateTime<Tz>,
	departure_live: DateTime<Tz>,
	on_time: bool,
	notification: bool,
	notification_ids: Vec<String>,
}

impl<Tz: TimeZone> Departure<Tz> {
	fn new(
		destination: String,
		line_name: String,
		line_id: String,
		track: String,
		track_label: String,
		departure_planned: DateTime<Tz>,
		departure_live: DateTime<Tz>,
		notification_ids: Vec<String>,
	) -> Departure<Tz> {
		Departure {
			destination: destination,
			line_name: line_name,
			line_id: line_id,
			track: track,
			track_label: track_label,
			departure_planned: departure_planned,
			departure_live: departure_live,
			on_time: departure_planned.,
			notification: notification,
			notification_ids: notification_ids,
		}
	}
}
