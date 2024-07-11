use anyhow::{Context, Result};
use serde::Serialize;
use tera::Tera;

pub struct Templater {}

impl Templater {
    pub fn new() -> Self {
        Self {}
    }
}

impl Templater {
    pub fn render_raw<S, T>(template: S, data: T) -> Result<String>
    where
        T: Serialize,
        S: Into<String>,
    {
        let mut tera = Tera::default();
        let context = tera::Context::from_serialize(data)?;

        Ok(tera.render_str(&template.into(), &context)?)
    }
}
