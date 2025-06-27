use super::error::Result;
use crate::registration::convention::Convention;
use crate::registration::event::Event;
use crate::registration::gender::Gender;
use crate::registration::gender::Gender::{Female, Male};
use crate::statistics::error::DrawingError;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use plotters::coord::Shift;
use plotters::prelude::{FontTransform::*, *};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

/// Draw a histogram based on the convention's data.
/// The histogram represents for each event of the convention
/// the repartition of participants between males and females.
///
/// Once generated, the graph is saved to a new file in given folder.
#[allow(dead_code)]
pub fn draw_and_export_graph(
    convention: &Convention,
    year: u16,
    folder: &std::path::Path,
) -> Result<()> {
    let file = folder.join(PathBuf::from(format!("{year}.png")));
    let drawing_area = draw_graph_by_gender_by_event(convention, year, &file)?;
    drawing_area
        .present()
        .map_err(|e| DrawingError::Presentation(e.to_string()))?;

    Ok(())
}

fn draw_graph_by_gender_by_event<'a>(
    convention: &Convention,
    year: u16,
    file: &'a PathBuf,
) -> Result<DrawingArea<BitMapBackend<'a>, Shift>> {
    let data = group_by_gender_by_event(convention);

    let root_drawing_area = create_drawing_area(file);
    let root_drawing_area = init_drawing_area(root_drawing_area)?;

    let events_count = convention.events().len();

    let margin_bottom = compute_margin_bottom(convention);
    let upper_y_bound = compute_upper_y_bound(&data);

    let caption = create_caption(year);

    let mut chart = create_chart_context(
        &root_drawing_area,
        margin_bottom,
        &caption,
        events_count as f32 * 2.0,
        upper_y_bound,
    )?;
    draw_chart(&mut chart, &data, events_count)?;

    Ok(root_drawing_area)
}

fn create_drawing_area(file: &PathBuf) -> DrawingArea<BitMapBackend, Shift> {
    BitMapBackend::new(file, (2048, 2048)).into_drawing_area()
}

fn init_drawing_area<DB, CT>(drawing_area: DrawingArea<DB, CT>) -> Result<DrawingArea<DB, CT>>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    drawing_area
        .fill(&WHITE)
        .map_err(|e| DrawingError::DrawingArea(e.to_string()))?;
    Ok(drawing_area)
}

fn compute_margin_bottom(convention: &Convention) -> u32 {
    let longest_event_name_length = compute_longest_event_name_length(convention);
    (longest_event_name_length as i32).ilog2() * 50 // FIXME: This does not make enough space for very long event names...
}

fn compute_longest_event_name_length(convention: &Convention) -> usize {
    convention
        .events()
        .iter()
        .map(|event| event.name().len())
        .max()
        .unwrap_or(10)
}

fn compute_upper_y_bound(data: &BTreeMap<&Event, HashMap<Gender, u64>>) -> i32 {
    let max_participants_count = compute_max_participants_count(data);
    (((max_participants_count + 10) / 10) * 10) as i32
}

fn compute_max_participants_count(data: &BTreeMap<&Event, HashMap<Gender, u64>>) -> u64 {
    data
        .values()
        .map(|participants| participants
            .values()
            .map(|count| *count)
            .max()
            .unwrap_or(10))
        .max()
        .unwrap_or(10)
}

fn create_caption(year: u16) -> String {
    format!("Répartition femmes/hommes par épreuve ({year})")
}

fn create_chart_context<'c, DB>(
    drawing_area: &DrawingArea<DB, Shift>,
    margin_bottom: u32,
    caption: &str,
    upper_x_bound: f32,
    upper_y_bound: i32,
) -> Result<ChartContext<'c, DB, Cartesian2d<RangedCoordf32, RangedCoordi32>>>
where
    DB: DrawingBackend,
{
    let mut chart = ChartBuilder::on(drawing_area)
        .margin_bottom(margin_bottom)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(0.0..upper_x_bound, 0..upper_y_bound)
        .map_err(|e| DrawingError::ChartContext(e.to_string()))?;

    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_x_mesh()
        .draw()
        .map_err(|e| DrawingError::ChartContext(e.to_string()))?;

    Ok(chart)
}

fn draw_chart<DB>(
    chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf32, RangedCoordi32>>,
    data: &BTreeMap<&Event, HashMap<Gender, u64>>,
    events_count: usize,
) -> Result<()>
where
    DB: DrawingBackend,
{
    chart
        .draw_series(
            (0..events_count)
                .zip(data.iter())
                .flat_map(|(x, (event, counts))| {
                    let x = x as f32 * 2.0;

                    let (female_bar, count_female) =
                        draw_bar(x, *counts.get(&Female).unwrap_or(&0), true, false, MAGENTA);
                    let (male_bar, count_male) =
                        draw_bar(x + 1.0, *counts.get(&Male).unwrap_or(&0), false, true, BLUE);
                    let label = draw_label(x, event.name().as_str(), 2);

                    vec![
                        female_bar.into_dyn(),
                        male_bar.into_dyn(),
                        label.into_dyn(),
                        count_female.into_dyn(),
                        count_male.into_dyn(),
                    ]
                }),
        )
        .map_err(|e| DrawingError::ChartDrawing(e.to_string()))?;

    Ok(())
}

type Bar<'a> = (Rectangle<(f32, i32)>, Text<'a, (f32, i32), String>);
fn draw_bar<'a>(
    x: f32,
    count: u64,
    first_of_group: bool,
    last_of_group: bool,
    color: RGBColor,
) -> Bar<'a> {
    let x0 = x;
    let x1 = x + 1.0;
    let count = count as i32;
    let mut bar = Rectangle::new([(x0, 0), (x1, count)], color.filled());
    let left_margin = if first_of_group { 5 } else { 1 };
    let right_margin = if last_of_group { 5 } else { 1 };
    bar.set_margin(0, 0, left_margin, right_margin);

    let font_desc =
        FontDesc::new(FontFamily::SansSerif, 16_f64, FontStyle::Normal).transform(Rotate270);
    let count_label = Text::new(count.to_string(), (x + 0.25, count + 1), font_desc.clone());

    (bar, count_label)
}

fn draw_label<'a>(x: f32, event_name: &str, number_of_bars: usize) -> Text<'a, (f32, i32), String> {
    let font_desc =
        FontDesc::new(FontFamily::SansSerif, 16_f64, FontStyle::Normal).transform(Rotate90);
    Text::new(
        format!("  {event_name}"),
        (x + 0.25 + (number_of_bars as f32 / 2.0), -1),
        font_desc.clone(),
    )
}

fn group_by_gender_by_event(convention: &Convention) -> BTreeMap<&Event, HashMap<Gender, u64>> {
    convention
        .events()
        .iter()
        .map(|event| {
            (
                event,
                convention
                    .participants_by_event()
                    .get(*event.index())
                    .expect(
                        "The convention has been wrongly constructed. Events counts do not match.",
                    )
                    .iter()
                    .fold(HashMap::new(), |mut acc, registrant| {
                        acc.entry(registrant.gender().clone())
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                        acc
                    }),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::get_test_convention;

    mod draw_and_export_graph {
        use crate::test_data::get_test_convention;
        use crate::statistics::gender_repartition::draw_and_export_graph;
        use std::env::temp_dir;
        use std::path::PathBuf;

        #[test]
        fn success() {
            let temp_dir = temp_dir();
            let convention = get_test_convention();
            let year = 2025;
            draw_and_export_graph(&convention, year, &temp_dir).unwrap();

            assert!(temp_dir.join(PathBuf::from(format!("{year}.png"))).exists());
        }
    }

    mod draw_graph_by_gender_by_event {
        use crate::test_data::get_test_convention;
        use crate::statistics::gender_repartition::draw_graph_by_gender_by_event;
        use std::path::PathBuf;

        /// This test simply ensures the graph gets drawn.
        /// I have not yet found a way to ensure the graph represents what's expected...
        #[test]
        fn success() {
            let convention = get_test_convention();
            let year = 2025;
            let file = PathBuf::from(format!("{year}.png"));
            draw_graph_by_gender_by_event(&convention, year, &file).unwrap();
        }
    }

    mod group_by_gender_by_event {
        use super::*;

        #[test]
        fn success() {
            let convention = get_test_convention();
            let expected_data: BTreeMap<Event, HashMap<Gender, u64>> = [
                (
                    Event::new(15, "10 kilomètres - Illimité".to_string()),
                    [(Female, 2), (Male, 3)].into_iter().collect(),
                ),
                (
                    Event::new(14, "10 kilomètres - Standard 24\"".to_string()),
                    [(Male, 3), (Female, 7)].into_iter().collect(),
                ),
                (
                    Event::new(4, "100m - All".to_string()),
                    [(Female, 6), (Male, 4)].into_iter().collect(),
                ),
                (
                    Event::new(29, "250 mètres - All".to_string()),
                    [(Female, 6), (Male, 5)].into_iter().collect(),
                ),
                (
                    Event::new(7, "30m marcher sur la roue - All".to_string()),
                    [(Female, 4), (Male, 4)].into_iter().collect(),
                ),
                (
                    Event::new(5, "400m - All".to_string()),
                    [(Male, 1), (Female, 3)].into_iter().collect(),
                ),
                (
                    Event::new(27, "50 mètres - All".to_string()),
                    [(Male, 2), (Female, 3)].into_iter().collect(),
                ),
                (
                    Event::new(6, "50m un pied - All".to_string()),
                    [(Female, 7), (Male, 5)].into_iter().collect(),
                ),
                (
                    Event::new(22, "Basket - All".to_string()),
                    [(Male, 3), (Female, 8)].into_iter().collect(),
                ),
                (
                    Event::new(18, "Cross court - All".to_string()),
                    [(Male, 4), (Female, 7)].into_iter().collect(),
                ),
                (
                    Event::new(28, "Cross long - All".to_string()),
                    [(Male, 4), (Female, 8)].into_iter().collect(),
                ),
                (
                    Event::new(21, "Flat - All".to_string()),
                    [(Female, 7), (Male, 4)].into_iter().collect(),
                ),
                (
                    Event::new(26, "Groupe - All".to_string()),
                    [(Female, 5), (Male, 3)].into_iter().collect(),
                ),
                (
                    Event::new(23, "Hockey - All".to_string()),
                    [(Male, 5), (Female, 7)].into_iter().collect(),
                ),
                (
                    Event::new(24, "Individuel - All".to_string()),
                    [(Female, 2), (Male, 3)].into_iter().collect(),
                ),
                (
                    Event::new(10, "Lenteur arrière - All".to_string()),
                    [(Female, 5), (Male, 1)].into_iter().collect(),
                ),
                (
                    Event::new(0, "Lenteur avant (planche large) - All".to_string()),
                    [(Male, 4), (Female, 11)].into_iter().collect(),
                ),
                (
                    Event::new(9, "Lenteur avant - All".to_string()),
                    [(Female, 6), (Male, 5)].into_iter().collect(),
                ),
                (
                    Event::new(17, "Marathon (42,195 km) - Illimité".to_string()),
                    [(Male, 5), (Female, 6)].into_iter().collect(),
                ),
                (
                    Event::new(16, "Marathon (42,195 km) - Standard 29\"".to_string()),
                    [(Male, 3), (Female, 4)].into_iter().collect(),
                ),
                (
                    Event::new(25, "Paire - All".to_string()),
                    [(Male, 6), (Female, 8)].into_iter().collect(),
                ),
                (
                    Event::new(1, "Parcours IUF - All".to_string()),
                    [(Female, 6), (Male, 6)].into_iter().collect(),
                ),
                (
                    Event::new(8, "Parcours IUF - All".to_string()),
                    [(Male, 12), (Female, 16)].into_iter().collect(),
                ),
                (
                    Event::new(2, "Parcours d'initiation sport-co - All".to_string()),
                    [(Male, 5), (Female, 6)].into_iter().collect(),
                ),
                (
                    Event::new(3, "Parcours d'obstacles - All".to_string()),
                    [(Male, 2), (Female, 4)].into_iter().collect(),
                ),
                (
                    Event::new(13, "Relais 4 x 100m - All".to_string()),
                    [(Male, 3), (Female, 7)].into_iter().collect(),
                ),
                (
                    Event::new(11, "Saut en hauteur - All".to_string()),
                    [(Male, 2), (Female, 10)].into_iter().collect(),
                ),
                (
                    Event::new(12, "Saut en longueur - All".to_string()),
                    [(Female, 6), (Male, 1)].into_iter().collect(),
                ),
                (
                    Event::new(20, "Street - All".to_string()),
                    [(Male, 3), (Female, 3)].into_iter().collect(),
                ),
                (
                    Event::new(19, "Trial - All".to_string()),
                    [(Female, 3), (Male, 3)].into_iter().collect(),
                ),
            ]
            .into_iter()
            .collect();

            let data: BTreeMap<Event, HashMap<Gender, u64>> = group_by_gender_by_event(&convention)
                .into_iter()
                .map(|(event, data)| ((*event).clone(), data))
                .collect();
            assert_eq!(expected_data, data)
        }
    }
}
