use chrono::{DateTime, Utc};

pub struct Profile {
    name: String,
    start_time: DateTime<Utc>,
}

impl Drop for Profile {
    fn drop(&mut self) {
        let end_time = Utc::now();
        let duration = end_time - self.start_time;

        println!("{}: {}ms {}fps", self.name, duration.num_milliseconds(), 1000 / duration.num_milliseconds());
    }
}

pub fn scope(name: &str) -> Profile {
    Profile {
        name: String::from(name),
        start_time: Utc::now(),
    }
}
