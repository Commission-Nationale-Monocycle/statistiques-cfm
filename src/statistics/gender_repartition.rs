use crate::registration::convention::Convention;
use crate::registration::event::Event;
use crate::registration::gender::Gender;
use crate::registration::gender::Gender::{Female, Male};
use plotters::prelude::{FontTransform::*, *};
use std::collections::{BTreeMap, HashMap};

fn draw_graph_by_gender_by_event(convention: &Convention, year: u16) {
    let file = format!("{year}.png");
    let root_drawing_area = BitMapBackend::new(&file, (2048, 2048)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let data = group_by_gender_by_event(convention);
    let events_count = convention.events().len();
    let max_participants_count = data
        .iter()
        .map(|(_, participants)| participants.iter().map(|(_, count)| *count).max().unwrap_or(10))
        .max()
        .unwrap_or(10);

    let upper_y_bound = (((max_participants_count + 10) / 10) * 10) as i32;

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .margin_bottom(300)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .caption(format!("Répartition femmes/hommes par épreuve ({year})"), ("sans-serif", 40))
        .build_cartesian_2d(0.0..events_count as f32 * 2.0, 0..upper_y_bound)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_x_mesh()
        .draw()
        .unwrap();
    chart
        .draw_series((0..events_count).zip(data.iter()).map(|(x, (event, counts))| {
            let x = x as f32;

            let x0 = x * 2.0;
            let x1 = x * 2.0 + 1.0;
            let females_count = *counts.get(&Female).unwrap_or(&0) as i32;
            let mut female_bar = Rectangle::new(
                [(x0, 0), (x1, females_count)],
                MAGENTA.filled(),
            );
            female_bar.set_margin(0, 0, 5, 1);

            let x0 = x * 2.0 + 1.0;
            let x1 = x * 2.0 + 2.0;
            let males_count = *counts.get(&Male).unwrap_or(&0) as i32;
            let mut male_bar = Rectangle::new(
                [(x0, 0), (x1, males_count)],
                BLUE.filled(),
            );
            male_bar.set_margin(0, 0, 1, 5);

            let font_desc = FontDesc::new(FontFamily::SansSerif, 16_f64, FontStyle::Normal).transform(Rotate90);
            let label = Text::new(format!("  {}", event.name().to_string()), (x * 2.0 + 1.25, -1), font_desc.clone());

            let font_desc = FontDesc::new(FontFamily::SansSerif, 16_f64, FontStyle::Normal).transform(Rotate270);
            let count_female = Text::new((females_count).to_string(), (x * 2.0 + 0.25, females_count + 1), font_desc.clone());
            let count_male = Text::new((males_count).to_string(), (x * 2.0 + 1.25, males_count + 1), font_desc.clone());

            vec![female_bar.into_dyn(), male_bar.into_dyn(), label.into_dyn(), count_female.into_dyn(), count_male.into_dyn()]
        })
            .flatten())
        .unwrap();

    root_drawing_area.present().unwrap();
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
    use std::path::PathBuf;
    use crate::registration::convention::load_convention;
    use super::*;
    use crate::registration::test_data::get_test_convention;

    #[test]
    fn test() {
        let convention = get_test_convention();
        draw_graph_by_gender_by_event(&convention, 2000);
    }
    
    #[test]
    fn test_cfm() {
        let years = [2016, 2017, 2018, 2019, 2023, 2024];
        for year in years {
            let convention = load_convention(&PathBuf::from(format!("test/assets/{year}.xls"))).unwrap();
            draw_graph_by_gender_by_event(&convention, year);
        }
    }
}
