use thiserror::Error;

pub type Result<T, E = DrawingError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum DrawingError {
    #[error("Error while presenting the graph: {0}")]
    Presentation(String),
    #[error("Error with the drawing area: {0}")]
    DrawingArea(String),
    #[error("Error with the chart context: {0}")]
    ChartContext(String),
    #[error("Error while drawing the chart: {0}")]
    ChartDrawing(String),
}