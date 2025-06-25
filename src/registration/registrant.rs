use crate::error::ApplicationError::{MisformattedRow, WrongFormat};
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
    club: Option<String>,
}

impl Registrant {
    pub fn new(
        id: u16,
        first_name: String,
        last_name: String,
        birthday: String,
        age: u8,
        gender: Gender,
        club: Option<String>,
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
    let id = extract_id(row.first().ok_or(MisformattedRow)?)?;
    let first_name = extract_first_name(row.get(1).ok_or(MisformattedRow)?)?;
    let last_name = extract_last_name(row.get(2).ok_or(MisformattedRow)?)?;
    let birthday = extract_birthday(row.get(3).ok_or(MisformattedRow)?)?;
    let age = extract_age(row.get(4).ok_or(MisformattedRow)?)?;
    let gender = extract_gender(row.get(5).ok_or(MisformattedRow)?)?;
    let club = extract_club(row.get(6).ok_or(MisformattedRow)?)?;
    let registrant = Registrant::new(
        id,
        first_name.clone(),
        last_name.clone(),
        birthday,
        age,
        gender,
        club.cloned(),
    );

    let registered_events = row.iter().skip(7).enumerate()
        .filter(|(_, registered)| match registered {
            Data::String(value) => value.as_str() == EVENT_REGISTRATION_STRING,
            Data::Bool(value) => *value,
            _ => false,
        })
        .map(|(index, _)| index)
        .collect();

    Ok((registrant, registered_events))
}

fn extract_id(id_cell: &Data) -> Result<u16> {
    match id_cell {
        Data::Int(id) => (*id)
            .try_into()
            .map_err(|_| WrongFormat(format!("ID is too large (`{id}`)"))),
        Data::Float(id) => (*id as u64)
            .try_into()
            .map_err(|_| WrongFormat(format!("ID is too large (`{id}`)"))),
        _ => Err(WrongFormat("id has the wrong format".to_string())),
    }
}

fn extract_first_name(first_name_cell: &Data) -> Result<&String> {
    match first_name_cell {
        Data::String(first_name) => Ok(first_name),
        _ => Err(WrongFormat("first_name has the wrong format".to_string())),
    }
}

fn extract_last_name(last_name_cell: &Data) -> Result<&String> {
    match last_name_cell {
        Data::String(last_name) => Ok(last_name),
        _ => Err(WrongFormat("last_name has the wrong format".to_string())),
    }
}

fn extract_birthday(birthday_cell: &Data) -> Result<String> {
    match birthday_cell {
        Data::String(birthday) => Ok(birthday.to_string()),
        Data::DateTime(birthday) => Ok(birthday.to_string()),
        Data::DateTimeIso(birthday) => Ok(birthday.to_string()),
        _ => Err(WrongFormat("birthday has the wrong format".to_string())),
    }
}

fn extract_age(age_cell: &Data) -> Result<u8> {
    match age_cell {
        Data::Int(age) => (*age)
            .try_into()
            .map_err(|_| WrongFormat(format!("Age is too large (`{age}`)"))),
        Data::Float(age) => (*age as u64)
            .try_into()
            .map_err(|_| WrongFormat(format!("Age is too large (`{age}`)"))),
        _ => Err(WrongFormat("age has the wrong format".to_string())),
    }
}

fn extract_gender(gender_cell: &Data) -> Result<Gender> {
    match gender_cell {
        Data::String(gender) => Gender::try_from(gender),
        _ => Err(WrongFormat("gender has the wrong format".to_string())),
    }
}

fn extract_club(club_cell: &Data) -> Result<Option<&String>> {
    match club_cell {
        Data::String(club) => Ok(Some(club)),
        Data::Empty => Ok(None),
        _ => Err(WrongFormat("club has the wrong format".to_string())),
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
                Some(club.to_string()),
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
                Some(club.to_string()),
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
                Some(club.to_string()),
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
        #[should_panic(expected = "WrongFormat")]
        fn fail_wrong_format_id() {
            let id = f64::MAX;
            let first_name = "John";
            let last_name = "Doe";
            let birthday = "2010-01-01";
            let age = 15;
            let gender = "Male";
            let club = "This is a club";

            let row = vec![
                Data::Float(id),
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

            parse_row(&row).unwrap();
        }

        #[test]
        #[should_panic(expected = "WrongFormat")]
        fn fail_wrong_format_age() {
            let id = 1_f64;
            let first_name = "John";
            let last_name = "Doe";
            let birthday = "2010-01-01";
            let age = f64::MAX;
            let gender = "Male";
            let club = "This is a club";

            let row = vec![
                Data::Float(id),
                Data::String(first_name.to_string()),
                Data::String(last_name.to_string()),
                Data::String(birthday.to_string()),
                Data::Float(age),
                Data::String(gender.to_string()),
                Data::String(club.to_string()),
                Data::String("VRAI".to_string()),
                Data::String("".to_string()),
                Data::String("".to_string()),
                Data::String("VRAI".to_string()),
            ];

            parse_row(&row).unwrap();
        }
    }

    mod extract_id {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_id;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            id_cell = { &Data::Int(1), &Data::Float(1.0) },
            expected_id = {1_u16, 1_u16}
        )]
        fn success(id_cell: &Data, expected_id: u16) {
            let result = extract_id(id_cell).unwrap();
            assert_eq!(expected_id, result);
        }

        #[parameterized(
            id_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::String("1".to_string()) },
        )]
        fn fail(id_cell: &Data) {
            let result = extract_id(id_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_first_name {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_first_name;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            first_name_cell = { &Data::String("John".to_string()) },
            expected_first_name = {"John".to_string()}
        )]
        fn success(first_name_cell: &Data, expected_first_name: String) {
            let result = extract_first_name(first_name_cell).unwrap();
            assert_eq!(&expected_first_name, result);
        }

        #[parameterized(
            first_name_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::Empty },
        )]
        fn fail(first_name_cell: &Data) {
            let result = extract_first_name(first_name_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_last_name {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_last_name;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            last_name_cell = { &Data::String("John".to_string()) },
            expected_last_name = {"John".to_string()}
        )]
        fn success(last_name_cell: &Data, expected_last_name: String) {
            let result = extract_last_name(last_name_cell).unwrap();
            assert_eq!(&expected_last_name, result);
        }

        #[parameterized(
            last_name_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::Empty },
        )]
        fn fail(last_name_cell: &Data) {
            let result = extract_last_name(last_name_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_birthday {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_birthday;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            birthday_cell = { &Data::String("01.12.1980".to_string()) },
            expected_birthday = {"01.12.1980".to_string()}
        )]
        fn success(birthday_cell: &Data, expected_birthday: String) {
            let result = extract_birthday(birthday_cell).unwrap();
            assert_eq!(expected_birthday, result);
        }

        #[parameterized(
            birthday_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::Empty },
        )]
        fn fail(birthday_cell: &Data) {
            let result = extract_birthday(birthday_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_age {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_age;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            age_cell = { &Data::Int(1), &Data::Float(1.0) },
            expected_age = {1_u8, 1_u8}
        )]
        fn success(age_cell: &Data, expected_age: u8) {
            let result = extract_age(age_cell).unwrap();
            assert_eq!(expected_age, result);
        }

        #[parameterized(
            age_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::String("1".to_string()) },
        )]
        fn fail(age_cell: &Data) {
            let result = extract_age(age_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_gender {
        use crate::error::ApplicationError;
        use crate::registration::gender::Gender;
        use crate::registration::registrant::extract_gender;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            gender_cell = { &Data::String(("Male").to_string()), &Data::String(("Female").to_string()) },
            expected_gender = {Gender::Male, Gender::Female}
        )]
        fn success(gender_cell: &Data, expected_gender: Gender) {
            let result = extract_gender(gender_cell).unwrap();
            assert_eq!(expected_gender, result);
        }

        #[parameterized(
            gender_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX), &Data::String("1".to_string()) },
        )]
        fn fail(gender_cell: &Data) {
            let result = extract_gender(gender_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }

    mod extract_club {
        use crate::error::ApplicationError;
        use crate::registration::registrant::extract_club;
        use calamine::Data;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            club_cell = { &Data::String("This is a club".to_string()), &Data::Empty },
            expected_club = {Some("This is a club".to_string()), None}
        )]
        fn success(club_cell: &Data, expected_club: Option<String>) {
            let result = extract_club(club_cell).unwrap().cloned();
            assert_eq!(expected_club, result);
        }

        #[parameterized(
            club_cell = { &Data::Int(i64::MAX), &Data::Float(f64::MAX) },
        )]
        fn fail(club_cell: &Data) {
            let result = extract_club(club_cell).err().unwrap();
            assert!(matches!(result, ApplicationError::WrongFormat(_)));
        }
    }


}
