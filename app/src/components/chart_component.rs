use crate::charts::{Chart, DataHandler};
use leptos::html::Canvas;
use leptos::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use polars::prelude::*;
use std::rc::Rc;
use tracing::error;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[component]
pub fn ChartCanvas(children: Children) -> impl IntoView {
    view! {
        <div class="chart-canvas">
            {children()}
        </div>
    }
}

#[component]
pub fn ChartComponent<T: Chart + DataHandler + Clone + 'static>(
    chart: T,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();
    let (data, set_data) = create_signal(DataFrame::default());

    let chart = Rc::new(chart);
    let chart_clone = chart.clone();
    let update_chart = move || {
        if let Some(canvas_element) = canvas_ref.get() {
            if let Ok(html_canvas_element) =
                <HtmlCanvasElement as Clone>::clone(&canvas_element.clone())
                    .dyn_into::<HtmlCanvasElement>()
            {
                html_canvas_element.set_width(800);
                html_canvas_element.set_height(600);
                if let Some(backend) = CanvasBackend::with_canvas_object(
                    html_canvas_element.clone(),
                ) {
                    let root = backend.into_drawing_area();
                    root.fill(&WHITE).unwrap();

                    if let Err(err) = chart_clone.render(&root, &data.get()) {
                        error!("Failed to render chart: {}", err);
                    }
                    root.present().unwrap();
                } else {
                    error!("Failed to create CanvasBackend");
                }
            } else {
                error!("Failed to convert element to HtmlCanvasElement");
            }
        } else {
            error!("Canvas element not found");
        }
    };

    create_effect(move |_| {
        update_chart();
    });

    view! {
        <div>
            <h2>{chart.title()}</h2>
            <canvas _ref=canvas_ref id="chart-canvas" width="800" height="600"></canvas>
            {chart.form_fields().into_iter().map(|field| {
                let field_label = field.label.clone();
                let chart = chart.clone();
                field.render(move |value| {
                    // Update form field values and fetch data
                    let params = vec![(field_label.clone(), value.clone())];
                    if let Ok(data) = chart.update_data(params) {
                        set_data.set(data);
                    }
                })
            }).collect::<Vec<_>>()}
        </div>
    }
}
