//! Whether a couple of events shares a lot of registrants.

use crate::registration::registrant::Registrant;

#[allow(dead_code)]
fn compute_dependencies(registrants: &[Vec<Registrant>]) -> Vec<Vec<f32>> {
    registrants
        .iter()
        .map(|registrants_1| {
            registrants
                .iter()
                .map(|registrants_2| compute_dependency(registrants_1, registrants_2))
                .collect()
        })
        .collect()
}

fn compute_dependency(registrants_1: &[Registrant], registrants_2: &[Registrant]) -> f32 {
    if registrants_1.is_empty() {
        1_f32
    } else {
        registrants_1
            .iter()
            .filter(|r| registrants_2.contains(r))
            .count() as f32
            / registrants_1.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::registration::gender::Gender;
    use crate::registration::registrant::Registrant;

    fn test_registrants() -> (Registrant, Registrant, Registrant, Registrant) {
        let r1 = Registrant::new(
            1,
            "John".to_string(),
            "Doe".to_string(),
            "01.01.1970".to_string(),
            55,
            Gender::Male,
            None,
        );
        let r2 = Registrant::new(
            2,
            "Dominique".to_string(),
            "Jacques".to_string(),
            "12.10.1962".to_string(),
            63,
            Gender::Female,
            None,
        );
        let r3 = Registrant::new(
            3,
            "Jeanne".to_string(),
            "Marie".to_string(),
            "22.08.1957".to_string(),
            68,
            Gender::Female,
            None,
        );
        let r4 = Registrant::new(
            4,
            "Timéo".to_string(),
            "Bernard".to_string(),
            "18.03.2012".to_string(),
            13,
            Gender::Male,
            None,
        );
        (r1, r2, r3, r4)
    }

    mod compute_dependencies {
        use super::test_registrants;
        use crate::statistics::events_registrannts_dependency::compute_dependencies;

        #[test]
        fn success() {
            let expected_result: Vec<Vec<f32>> = vec![
                vec![1.0, 0.5, 0.5, 1.0, 0.0],
                vec![1.0 / 3.0, 1.0, 2.0 / 3.0, 1.0, 0.0],
                vec![1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 0.0],
                vec![0.5, 0.75, 0.75, 1.0, 0.0],
                vec![1.0, 1.0, 1.0, 1.0, 1.0],
            ];

            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![r1.clone(), r2.clone()];
            let registrants_2 = vec![r2.clone(), r3.clone(), r4.clone()];
            let registrants_3 = vec![r1.clone(), r3.clone(), r4.clone()];
            let registrants_4 = vec![r1, r2, r3, r4];
            let registrants_5 = vec![];

            let result = compute_dependencies(&[registrants_1,
                registrants_2,
                registrants_3,
                registrants_4,
                registrants_5]);

            assert_eq!(expected_result, result);
        }
    }

    mod compute_dependency {
        use super::super::compute_dependency;
        use super::test_registrants;

        #[test]
        fn success_50_percents() {
            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![r1.clone(), r2.clone(), r3.clone(), r4.clone()];
            let registrants_2 = vec![r1, r2];

            assert_eq!(0.5_f32, compute_dependency(&registrants_1, &registrants_2));
        }

        #[test]
        fn success_100_percents() {
            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![r1.clone(), r2.clone()];
            let registrants_2 = vec![r1, r2, r3, r4];

            assert_eq!(1_f32, compute_dependency(&registrants_1, &registrants_2));
        }

        #[test]
        fn success_no_registrants() {
            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![];
            let registrants_2 = vec![r1, r2, r3, r4];

            assert_eq!(1_f32, compute_dependency(&registrants_1, &registrants_2));
        }
    }
}
