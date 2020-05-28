use csv::Reader;
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
pub struct Reservation {
    guest: String,
    checked_in: bool,
}
/// count the number of reservations
pub fn count_reservations(bytes: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_reader(bytes);
    let mut count = 0;
    for result in reader.deserialize() {
        let _record: Reservation = result?;
        count += 1;
    }
    Ok(count)
}
#[cfg(test)]
mod tests {
    use super::*;
    use csv::ErrorKind;
    use csv::Reader;
    #[test]
    fn reservation_count() {
        let reservations = "guest,checked_in
        fred,false";
        let count = count_reservations(reservations.as_bytes()).unwrap();
        assert_eq!(count, 1);
    }
}
