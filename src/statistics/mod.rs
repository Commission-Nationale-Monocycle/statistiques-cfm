use crate::registration::convention::Convention;
use crate::registration::event::Event;
use crate::registration::gender::Gender;
use crate::registration::gender::Gender::{Female, Male};
use plotters::prelude::*;
use std::collections::{BTreeMap, HashMap};
use plotters::style::FontTransform::Rotate90;

fn draw_graph_by_gender_by_event(convention: &Convention) {
    let root_drawing_area = BitMapBackend::new("0.1.png", (2048, 2048)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .margin_bottom(300)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .caption("Répartition femmes/hommes par épreuve", ("sans-serif", 40))
        .build_cartesian_2d((0..60).into_segmented(), 0..20)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_axis()
        .disable_x_mesh()
        .draw()
        .unwrap();
    let data = group_by_gender_by_event(convention);
    chart
        .draw_series((0..30).zip(data.iter()).map(|(x, (event, counts))| {
            let x0 = SegmentValue::Exact(x * 2);
            let x1 = SegmentValue::Exact(x * 2 + 1);
            let mut female_bar = Rectangle::new(
                [(x0, 0), (x1, *counts.get(&Female).unwrap_or(&0) as i32)],
                MAGENTA.filled(),
            );
            female_bar.set_margin(0, 0, 5, 1);

            let x0 = SegmentValue::Exact(x * 2 + 1);
            let x1 = SegmentValue::Exact(x * 2 + 2);
            let mut male_bar = Rectangle::new(
                [(x0, 0), (x1, *counts.get(&Male).unwrap_or(&0) as i32)],
            BLUE.filled(),
            );
            male_bar.set_margin(0, 0, 1, 5);

            let font_desc = FontDesc::new(FontFamily::SansSerif, 16_f64, FontStyle::Normal).transform(Rotate90);
            let label = Text::new(format!("  {}", event.name().to_string()), (SegmentValue::Exact(x * 2 + 1), -1), font_desc);

            vec![female_bar.into_dyn(), male_bar.into_dyn(), label.into_dyn()]
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
        draw_graph_by_gender_by_event(&convention);
    }

    #[test]
    fn test_2016() {
        let convention = load_convention(&PathBuf::from("2016.xls")).unwrap();
        draw_graph_by_gender_by_event(&convention);
    }
}
