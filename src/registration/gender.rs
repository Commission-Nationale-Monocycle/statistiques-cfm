use crate::error;
use crate::error::ApplicationError;
use crate::error::ApplicationError::WrongFormat;
use crate::registration::gender::Gender::{Female, Male};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash, Ord, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl TryFrom<&String> for Gender {
    type Error = ApplicationError;

    fn try_from(value: &String) -> error::Result<Self, Self::Error> {
        match value as &str {
            "Male" => Ok(Male),
            "Female" => Ok(Female),
            _ => Err(WrongFormat(format!(
                "Gender should be `Male` or `Female`. Got `{value}` instead.",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    mod try_from_and_string {
        use crate::registration::gender::Gender;
        use parameterized::parameterized;

        #[test]
        #[ignore]
        fn ide_support() {
            // This ignored test is mandatory for IntelliJ to detect tests in this module.
        }

        #[parameterized(
            gender_string = { "Female", "Male" },
            expected_gender = { Gender::Female, Gender::Male }
        )]
        fn success(gender_string: &str, expected_gender: Gender) {
            assert_eq!(
                expected_gender,
                Gender::try_from(&String::from(gender_string)).unwrap()
            );
        }

        #[parameterized(
            gender_string = { "", "x" },
        )]
        #[should_panic(expected = "Gender should be `Male` or `Female`.")]
        fn fail(gender_string: &str) {
            Gender::try_from(&String::from(gender_string)).unwrap();
        }
    }
}
