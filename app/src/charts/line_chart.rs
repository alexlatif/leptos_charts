use crate::charts::{Chart, DataHandler};
use crate::components::data_control::{FieldType, FormField};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use polars::prelude::*;

#[derive(Clone)]
pub struct LineChart {
    form_fields: Vec<FormField>,
}

impl LineChart {
    pub fn new() -> Self {
        Self {
            form_fields: vec![
                FormField::new(
                    "Power".to_string(),
                    FieldType::Number,
                    "2".to_string(),
                ),
                FormField::new(
                    "Chart Type".to_string(),
                    FieldType::Select(vec![
                        "Line".to_string(),
                        "Bar".to_string(),
                    ]),
                    "Line".to_string(),
                ),
            ],
        }
    }
}

impl Chart for LineChart {
    fn title(&self) -> &'static str {
        "Line Chart"
    }

    fn form_fields(&self) -> Vec<FormField> {
        self.form_fields.clone()
    }

    fn render(
        &self,
        root: &DrawingArea<CanvasBackend, Shift>,
        data: &DataFrame,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render chart
        Ok(())
    }
}

impl DataHandler for LineChart {
    fn update_data(
        &self,
        params: Vec<(String, String)>,
    ) -> Result<DataFrame, Box<dyn std::error::Error>> {
        // Fetch data based on form field parameters
        let df = DataFrame::default();
        Ok(df)
    }
}
