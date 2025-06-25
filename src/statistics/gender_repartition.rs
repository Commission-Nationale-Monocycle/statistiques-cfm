use crate::registration::convention::Convention;
use crate::registration::event::Event;
use crate::registration::gender::Gender;
use crate::registration::gender::Gender::{Female, Male};
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use plotters::coord::Shift;
use plotters::prelude::{FontTransform::*, *};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[allow(dead_code)]
fn draw_graph_by_gender_by_event(convention: &Convention, year: u16) {
    let data = group_by_gender_by_event(convention);

    let file = PathBuf::from(format!("{year}.png"));
    let root_drawing_area = create_drawing_area(&file);

    let events_count = convention.events().len();

    let margin_bottom = compute_margin_bottom(convention);
    let upper_y_bound = compute_upper_y_bound(&data);

    let caption = create_caption(year);

    let mut chart = create_chart(
        &root_drawing_area,
        margin_bottom,
        &caption,
        events_count as f32 * 2.0,
        upper_y_bound,
    );
    draw_chart(&mut chart, &data, events_count);

    root_drawing_area.present().unwrap();
}

fn create_drawing_area(file: &PathBuf) -> DrawingArea<BitMapBackend, Shift> {
    let drawing_area = BitMapBackend::new(file, (2048, 2048)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    drawing_area
}

fn compute_margin_bottom(convention: &Convention) -> u32 {
    let longest_event_name_length = compute_longest_event_name_length(convention);
    (longest_event_name_length as i32).ilog2() * 50
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
    let max_participants_count = compute_max_participants_count(&data);
    (((max_participants_count + 10) / 10) * 10) as i32
}

fn compute_max_participants_count(data: &BTreeMap<&Event, HashMap<Gender, u64>>) -> u64 {
    data.iter()
        .map(|(_, participants)| {
            participants
                .iter()
                .map(|(_, count)| *count)
                .max()
                .unwrap_or(10)
        })
        .max()
        .unwrap_or(10)
}

fn create_caption(year: u16) -> String {
    format!("Répartition femmes/hommes par épreuve ({year})")
}

fn create_chart<'a, 'c>(
    drawing_area: &DrawingArea<BitMapBackend<'a>, Shift>,
    margin_bottom: u32,
    caption: &str,
    upper_x_bound: f32,
    upper_y_bound: i32,
) -> ChartContext<'c, BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordi32>> {
    let mut chart = ChartBuilder::on(drawing_area)
        .margin_bottom(margin_bottom) // FIXME: This does not make enough space for very long event names...
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(0.0..upper_x_bound, 0..upper_y_bound)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_x_mesh()
        .draw()
        .unwrap();

    chart
}


fn draw_chart(chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordi32>>, data: &BTreeMap<&Event, HashMap<Gender, u64>>, events_count: usize) {
    chart
        .draw_series(
            (0..events_count)
                .zip(data.iter())
                .map(|(x, (event, counts))| {
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
                })
                .flatten(),
        )
        .unwrap();
}

fn draw_bar<'a>(
    x: f32,
    count: u64,
    first_of_group: bool,
    last_of_group: bool,
    color: RGBColor,
) -> (Rectangle<(f32, i32)>, Text<'a, (f32, i32), String>) {
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
        format!("  {}", event_name),
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
    use crate::registration::convention::load_convention;
    use crate::registration::test_data::get_test_convention;
    use std::path::PathBuf;

    #[test]
    fn test() {
        let convention = get_test_convention();
        draw_graph_by_gender_by_event(&convention, 2000);
    }
}
