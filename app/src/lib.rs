use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod charts;
pub mod components;
pub mod error_template;

use charts::{bar_chart::BarChart, line_chart::LineChart};
use components::{
    auth_component::{AuthComponent, AuthState},
    chart_component::{ChartCanvas, ChartComponent},
};
use std::{cell::RefCell, rc::Rc};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        <Title text="Chart Dashboard"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=move || view! { <HomePage /> } />
                    <Route path="/charts" view=ChartsPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let auth_state = use_context::<Rc<RefCell<AuthState>>>()
        .expect("AuthState context should be provided");

    view! {
        <div>
            <h1>"Welcome to the Chart Dashboard"</h1>
            <AuthComponent auth_state=auth_state />
        </div>
    }
}

#[component]
fn ChartsPage() -> impl IntoView {
    view! {
        <ChartCanvas>
            <ChartComponent chart=LineChart::new() />
            <ChartComponent chart=BarChart::new() />
        </ChartCanvas>
    }
}
