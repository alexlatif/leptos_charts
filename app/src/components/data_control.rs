use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};

#[derive(Clone)]
pub enum FieldType {
    Text,
    Number,
    Select(Vec<String>),
}

#[derive(Clone)]
pub struct FormField {
    pub label: String,
    pub field_type: FieldType,
    pub default: String,
}

impl FormField {
    pub fn new(label: String, field_type: FieldType, default: String) -> Self {
        Self { label, field_type, default }
    }

    pub fn render(
        &self,
        update_signal: impl Fn(String) + 'static,
    ) -> impl IntoView {
        let default = self.default.clone();
        match &self.field_type {
            FieldType::Text => view! {
                <div class="form-field">
                    <label>{&self.label}</label>
                    <input type="text" value={default} on:input=move |e| {
                        if let Some(input) = e.target().expect("Text input error").dyn_into::<HtmlInputElement>().ok() {
                            update_signal(input.value());
                        }
                    }/>
                </div>
            },
            FieldType::Number => view! {
                <div class="form-field">
                    <label>{&self.label}</label>
                    <input type="number" value={default} on:input=move |e| {
                        if let Some(input) = e.target().expect("Number input error").dyn_into::<HtmlInputElement>().ok() {
                            update_signal(input.value());
                        }
                    }/>
                </div>
            },
            FieldType::Select(options) => view! {
                <div class="form-field">
                    <label>{&self.label}</label>
                    <select on:change=move |e| {
                        if let Some(select) = e.target().expect("Select input error").dyn_into::<HtmlSelectElement>().ok() {
                            update_signal(select.value());
                        }
                    }>
                        {options.iter().map(|option| view! { <option>{option}</option> }).collect::<Vec<_>>()}
                    </select>
                </div>
            },
        }
    }
}
