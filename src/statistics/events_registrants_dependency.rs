//! Whether a couple of events shares a lot of registrants.

use crate::configuration::events_configuration::EventsConfiguration;
use crate::configuration::events_mapping::CategoriesMapping;
use crate::registration::convention::load_convention;
use crate::registration::registrant::Registrant;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn generate_csv_file(file: &PathBuf, events_mapping: &CategoriesMapping, events_configuration: &EventsConfiguration) -> String {
    let convention = load_convention(file).unwrap();
    let registrants = convention.participants_by_event();

    // Let's regroup events and categories that can be grouped
    // `events_to_regroup` maps each event to its category
    let mut events_to_regroup = HashMap::new();
    for (category_name, events_category) in events_configuration.categories() {
        if *events_category.can_regroup() {
            events_category.events()
                .iter()
                .for_each(|event| {
                    events_to_regroup.insert(event.0, category_name);
                });
        }
    }

    let events = convention.events();
    let mut grouped_events = HashSet::new();
    let mut grouped_registrants = HashMap::new();
    for (i, event) in events.iter().enumerate() {
        for (category_id, events_mapped_to_category) in events_mapping {
            for (event_key, possible_events_names) in events_mapped_to_category {
                if possible_events_names.contains(event.name()) {
                    if events_to_regroup.contains_key(event_key) {
                        grouped_events.insert(category_id);
                        grouped_registrants.entry(category_id)
                            .or_insert_with(HashSet::new)
                            .extend(registrants.get(i).unwrap());
                    } else {
                        grouped_events.insert(event.name());
                        grouped_registrants.insert(event.name(), registrants.get(i).unwrap().iter().collect::<HashSet<_>>());
                    }
                }
            }
        }
    }

    let mut grouped_events = grouped_events.into_iter().collect::<Vec<_>>();
    grouped_events.sort();
    let grouped_registrants = grouped_events.iter()
        .map(|grouped_event| grouped_registrants
            .get(grouped_event)
            .unwrap()
            .iter()
            .cloned()
            .collect())
        .collect::<Vec<_>>();

    // Now, we can compute dependencies
    let dependencies = compute_dependencies(&grouped_registrants);
    let dependencies: Vec<Vec<String>> = dependencies
        .iter()
        .map(|dependencies| {
            dependencies
                .iter()
                .map(|dependency| format!("{}/{} ({:.2}%)", dependency.0, dependency.1, (dependency.0 as f32 / dependency.1 as f32) * 100.0))
                .collect::<Vec<String>>()
        })
        .collect();

    let mut content = format!(";{}", grouped_events
        .iter()
        .map(|event_or_category| (*event_or_category).clone())
        .reduce(|acc, name| format!("{acc};{name}"))
        .unwrap());

    for (i, event) in grouped_events.iter().enumerate() {
        let line = dependencies
            .get(i)
            .unwrap()
            .iter()
            .cloned()
            .reduce(|acc, dependency| format!("{acc};{dependency}"))
            .unwrap();
        content = format!("{content}\n{};{line}", event);
    }

    content
}

fn compute_dependencies(registrants: &[HashSet<&Registrant>]) -> Vec<Vec<(usize, usize)>> {
    
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

fn compute_dependency(registrants_1: &HashSet<&Registrant>, registrants_2: &HashSet<&Registrant>) -> (usize, usize) {
    if registrants_1.is_empty() {
        (0, registrants_2.len())
    } else {
        (registrants_1
            .iter()
            .filter(|r| registrants_2.contains(**r))
            .count(), registrants_2.len())
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
        use std::collections::HashSet;
        use super::test_registrants;
        use crate::statistics::events_registrants_dependency::compute_dependencies;

        #[test]
        fn success() {
            let expected_result: Vec<Vec<(usize, usize)>> = vec![
                vec![(2, 2), (1, 3), (1, 3), (2, 4), (0, 0)],
                vec![(1, 2), (3, 3), (2, 3), (3, 4), (0, 0)],
                vec![(1, 2), (2, 3), (3, 3), (3, 4), (0, 0)],
                vec![(2, 2), (3, 3), (3, 3), (4, 4), (0, 0)],
                vec![(0, 2), (0, 3), (0, 3), (0, 4), (0, 0)],
            ];

            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = HashSet::from([&r1, &r2]);
            let registrants_2 = HashSet::from([&r2, &r3, &r4]);
            let registrants_3 = HashSet::from([&r1, &r3, &r4]);
            let registrants_4 = HashSet::from([&r1, &r2, &r3, &r4]);
            let registrants_5 = HashSet::from([]);

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

            let registrants_1 = vec![&r1, &r2, &r3, &r4].into_iter().collect();
            let registrants_2 = vec![&r1, &r2].into_iter().collect();

            assert_eq!((2,2), compute_dependency(&registrants_1, &registrants_2));
        }

        #[test]
        fn success_100_percents() {
            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![&r1, &r2].into_iter().collect();
            let registrants_2 = vec![&r1, &r2, &r3, &r4].into_iter().collect();

            assert_eq!((2,4), compute_dependency(&registrants_1, &registrants_2));
        }

        #[test]
        fn success_no_registrants() {
            let (r1, r2, r3, r4) = test_registrants();

            let registrants_1 = vec![].into_iter().collect();
            let registrants_2 = vec![&r1, &r2, &r3, &r4].into_iter().collect();

            assert_eq!((0, 4), compute_dependency(&registrants_1, &registrants_2));
        }
    }

}
