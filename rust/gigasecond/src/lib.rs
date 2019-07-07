use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gig_sec = Duration::seconds(10f64.powi(9i32) as i64);
    match start.checked_add_signed(gig_sec) {
        Some(s) => return s,
        None => panic!("Could not add duration")
    };
}
