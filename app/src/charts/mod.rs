use crate::components::data_control::FormField;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use polars::prelude::*;

pub mod bar_chart;
pub mod line_chart;

pub trait Chart {
    fn title(&self) -> &'static str;
    fn form_fields(&self) -> Vec<FormField>;
    fn render(
        &self,
        root: &DrawingArea<CanvasBackend, Shift>,
        data: &DataFrame,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait DataHandler {
    fn update_data(
        &self,
        params: Vec<(String, String)>,
    ) -> Result<DataFrame, Box<dyn std::error::Error>>;
}
