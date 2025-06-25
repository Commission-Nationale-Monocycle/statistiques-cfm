use crate::error::ApplicationError::MisformattedRow;
use crate::error::Result;
use crate::registration::gender::Gender;
use calamine::Data;
use derive_getters::Getters;

#[derive(Debug, Getters, PartialOrd, PartialEq, Clone)]
pub struct Registrant {
    id: u16,
    first_name: String,
    last_name: String,
    birthday: String,
    age: u8,
    gender: Gender,
    club: String,
}

impl Registrant {
    pub fn new(
        id: u16,
        first_name: String,
        last_name: String,
        birthday: String,
        age: u8,
        gender: Gender,
        club: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            birthday,
            age,
            gender,
            club,
        }
    }
}

const EVENT_REGISTRATION_STRING: &str = "VRAI";

/// Create a [Registrant] and its list of registered events from a spreadsheet row.
pub fn parse_row(row: &[Data]) -> Result<(Registrant, Vec<usize>)> {
    match row {
        [
            Data::Float(id),
            Data::String(first_name),
            Data::String(last_name),
            Data::String(birthday),
            Data::Float(age),
            Data::String(gender),
            Data::String(club),
            registered_events @ ..,
        ] => {
            let registrant = Registrant::new(
                *id as u16,
                first_name.to_string(),
                last_name.to_string(),
                birthday.to_string(),
                *age as u8,
                gender.try_into()?,
                club.to_string(),
            );

            let registered_events = registered_events
                .iter()
                .enumerate()
                .filter(|(_, registered)| match registered {
                    Data::String(value) => value.as_str() == EVENT_REGISTRATION_STRING,
                    Data::Bool(value) => *value,
                    _ => false,
                })
                .map(|(index, _)| index)
                .collect();

            Ok((registrant, registered_events))
        }
        &_ => Err(MisformattedRow)?,
    }
}

#[cfg(test)]
mod tests {
    mod parse_row {
        use crate::registration::gender::Gender;
        use crate::registration::registrant::{parse_row, Registrant};
        use calamine::Data;

        #[test]
        fn success_no_event() {
            let id = 1_u16;
            let first_name = "John";
            let last_name = "Doe";
            let birthday = "2010-01-01";
            let age = 15;
            let gender = "Male";
            let club = "This is a club";

            let expected_registrant = Registrant::new(
                id,
                first_name.to_string(),
                last_name.to_string(),
                birthday.to_string(),
                age as u8,
                Gender::Male,
                club.to_string(),
            );

            let row = vec![
                Data::Float(id as f64),
                Data::String(first_name.to_string()),
                Data::String(last_name.to_string()),
                Data::String(birthday.to_string()),
                Data::Float(age as f64),
                Data::String(gender.to_string()),
                Data::String(club.to_string()),
            ];

            let (registrant, registered_events) = parse_row(&row).unwrap();

            assert_eq!(expected_registrant, registrant);
            assert_eq!(Vec::<usize>::new(), registered_events);
        }

        #[test]
        fn success_no_event_registered() {
            let id = 1_u16;
            let first_name = "John";
            let last_name = "Doe";
            let birthday = "2010-01-01";
            let age = 15;
            let gender = "Male";
            let club = "This is a club";

            let expected_registrant = Registrant::new(
                id,
                first_name.to_string(),
                last_name.to_string(),
                birthday.to_string(),
                age as u8,
                Gender::Male,
                club.to_string(),
            );

            let row = vec![
                Data::Float(id as f64),
                Data::String(first_name.to_string()),
                Data::String(last_name.to_string()),
                Data::String(birthday.to_string()),
                Data::Float(age as f64),
                Data::String(gender.to_string()),
                Data::String(club.to_string()),
                Data::String("".to_string()),
                Data::String("".to_string()),
                Data::String("".to_string()),
                Data::String("".to_string()),
            ];

            let (registrant, registered_events) = parse_row(&row).unwrap();

            assert_eq!(expected_registrant, registrant);
            assert_eq!(Vec::<usize>::new(), registered_events);
        }

        #[test]
        fn success_events_registered() {
            let id = 1_u16;
            let first_name = "John";
            let last_name = "Doe";
            let birthday = "2010-01-01";
            let age = 15;
            let gender = "Male";
            let club = "This is a club";

            let expected_registration = Registrant::new(
                id,
                first_name.to_string(),
                last_name.to_string(),
                birthday.to_string(),
                age as u8,
                Gender::Male,
                club.to_string(),
            );

            let row = vec![
                Data::Float(id as f64),
                Data::String(first_name.to_string()),
                Data::String(last_name.to_string()),
                Data::String(birthday.to_string()),
                Data::Float(age as f64),
                Data::String(gender.to_string()),
                Data::String(club.to_string()),
                Data::String("VRAI".to_string()),
                Data::String("".to_string()),
                Data::String("".to_string()),
                Data::String("VRAI".to_string()),
            ];

            let (registrant, registered_events) = parse_row(&row).unwrap();

            assert_eq!(expected_registration, registrant);
            assert_eq!(vec![0, 3], registered_events);
        }

        #[test]
        #[should_panic(expected = "MisformattedRow")]
        fn fail_empty_row() {
            let row = vec![];
            parse_row(&row).unwrap();
        }

        #[test]
        #[should_panic(expected = "MisformattedRow")]
        fn fail_wrong_format() {
            let row = vec![Data::String("id".to_string())];
            parse_row(&row).unwrap();
        }
    }
}
