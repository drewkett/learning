use serde::Deserialize;
#[derive(Deserialize)]
pub enum Reservation {
    /// yes I am attending
    Y,
    /// no I cannot make it
    N,
    /// noncommital responses
    M,
}
#[cfg(test)]
mod tests {
    use super::*;
    use csv::ErrorKind;
    use csv::Reader;
    #[test]
    fn reservation_invalid() {
        let reservations = "reservation
        z";

        let mut reader = Reader::from_reader(reservations.as_bytes());
        for result in reader.deserialize() {
            match result {
                Ok(raw) => {
                    let _record: Reservation = raw;
                }
                Err(err) => match err.kind() {
                    ErrorKind::Deserialize { err: other, pos: _ } => {
                        assert_eq!(other.to_string(), "invalid Reservation code provided");
                    }
                    _ => {}
                },
            };
        }
    }
}
