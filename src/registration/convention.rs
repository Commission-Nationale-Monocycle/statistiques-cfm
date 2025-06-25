use crate::error;
use crate::error::ApplicationError::NoHeaders;
use crate::registration::event::Event;
use crate::registration::registrant;
use crate::registration::registrant::Registrant;
use calamine::{open_workbook, HeaderRow, Reader, Xls};
use derive_getters::Getters;
use std::path::PathBuf;

/// A convention is a wrapper over its registrants, and events they have registered to.
///
/// Everything being cloned for now, it can be a bottleneck.
/// If performance issues arise, you know where to look for optimization :)
#[derive(Debug, Getters, PartialEq)]
pub struct Convention {
    /// Second member of this tuple directly refers to the events' index in [Convention::events].
    registrations: Vec<(Registrant, Vec<usize>)>,
    events: Vec<Event>,
    /// This directly use [Convention::events] indexes.
    participants_by_event: Vec<Vec<Registrant>>,
}

impl Convention {
    /// Building a convention is done by associating each participant to the event it has registered to.
    pub fn build(registrations: Vec<(Registrant, Vec<usize>)>, events: Vec<Event>) -> Self {
        let mut participants_by_event: Vec<_> = (0..events.len()).map(|_| vec![]).collect();

        for (registrant, registered_events) in registrations.clone() {
            for event in registered_events {
                participants_by_event
                    .get_mut(event)
                    .expect("No participant")
                    .push(registrant.clone());
            }
        }

        Convention {
            registrations,
            events,
            participants_by_event,
        }
    }

    #[cfg(test)]
    pub fn new(
        registrations: Vec<(Registrant, Vec<usize>)>,
        events: Vec<Event>,
        participants_by_event: Vec<Vec<Registrant>>,
    ) -> Self {
        Self {
            registrations,
            events,
            participants_by_event,
        }
    }
}

/// Load a convention into memory from an XLS file.
/// It is expected for the XLS file to have the following columns:
/// - Id: Integer
/// - First Name: String
/// - Last Name: String
/// - Birthday: String (mm.dd.YYYY)
/// - Age: Integer
/// - Gender: `Male` or `Female`
/// - Club: String
///
/// Followed by a list of events, whose cell's values could be nothing or `VRAI` or a `true` boolean.
#[allow(dead_code)]
pub fn load_convention(path: &PathBuf) -> error::Result<Convention> {
    let mut workbook: Xls<_> = open_workbook(path)?;
    let range = workbook
        .with_header_row(HeaderRow::FirstNonEmptyRow)
        .worksheet_range("Sheet1")?;

    let headers = range.headers();
    let events = retrieve_event_list(headers)?;
    let mut registrations = vec![];

    for row in range.rows().skip(1) {
        // Skipping the header line
        let (registrant, registered_events) = registrant::parse_row(row)?;
        registrations.push((registrant, registered_events));
    }

    let convention = Convention::build(registrations, events);
    Ok(convention)
}

/// If there's a header, then retrieves the event list which should be denoted by having ` - ` in their names.
pub fn retrieve_event_list(headers: Option<Vec<String>>) -> error::Result<Vec<Event>> {
    if let Some(headers) = headers {
        let events = headers
            .into_iter()
            .filter(|header| header.contains(" - ")) // FIXME: should not detect other lines such as "Basket - Nom du capitaine"
            .enumerate()
            .map(|(index, name)| Event::new(index, name))
            .collect();

        Ok(events)
    } else {
        Err(NoHeaders)?
    }
}

#[cfg(test)]
mod tests {
    mod load_convention {
        use super::super::load_convention;
        use super::super::Convention;
        use crate::registration::test_data;
        use test_data::*;

        #[test]
        fn success() {
            let expected_convention = Convention::new(
                get_test_registrations(),
                get_event_list(),
                get_participants_by_event(),
            );

            let path = get_test_asset("registrations.xls");
            let convention = load_convention(&path).unwrap();

            assert_eq!(expected_convention, convention);
        }

        #[test]
        #[should_panic(expected = "NotFound")]
        fn fail_on_file_not_found() {
            let path = get_test_asset("not_found.xls");
            load_convention(&path).unwrap();
        }

        #[test]
        #[should_panic(expected = "WorksheetNotFound")]
        fn fail_on_sheet_not_found() {
            let path = get_test_asset("wrong_sheet_name.xls");
            load_convention(&path).unwrap();
        }
    }

    mod retrieve_event_list {
        use super::super::retrieve_event_list;
        use crate::registration::event::Event;

        #[test]
        fn success() {
            let headers = Some(vec![
                "Id".to_string(),
                "First Name".to_string(),
                "Last Name".to_string(),
                "Birthday".to_string(),
                "Age".to_string(),
                "Gender".to_string(),
                "Club".to_string(),
                "Basket A - All".to_string(),
                "Cross long - All".to_string(),
            ]);

            let expected_event_list: Vec<Event> = vec!["Basket A - All", "Cross long - All"]
                .into_iter()
                .enumerate()
                .map(|(index, name)| Event::new(index, name.to_string()))
                .collect();

            let event_list = retrieve_event_list(headers).unwrap();

            assert_eq!(expected_event_list, event_list);
        }

        #[test]
        #[ignore] // FIXME: reactivate once event details ignoring has been fixed.
        fn success_ignore_event_details() {
            let headers = Some(vec![
                "Id".to_string(),
                "First Name".to_string(),
                "Last Name".to_string(),
                "Birthday".to_string(),
                "Age".to_string(),
                "Gender".to_string(),
                "Club".to_string(),
                "Basket A - All".to_string(),
                "Cross long - All".to_string(),
                "Basket A - Nom du capitaine".to_string(),
            ]);

            let expected_event_list: Vec<Event> = vec!["Basket A - All", "Cross long - All"]
                .into_iter()
                .enumerate()
                .map(|(index, name)| Event::new(index, name.to_string()))
                .collect();

            let event_list = retrieve_event_list(headers).unwrap();

            assert_eq!(expected_event_list, event_list);
        }

        #[test]
        #[should_panic(expected = "NoHeaders")]
        fn fail_no_header() {
            retrieve_event_list(None).unwrap();
        }
    }
}
