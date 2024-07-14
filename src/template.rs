use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use std::fmt::Display;
use std::path::Path;
use tera::Tera;

#[derive(Debug, Clone)]
pub struct Templater {
    engine: Tera,
}

impl Templater {
    pub fn new(glob: &str) -> Result<Self>
where {
        Ok(Self {
            engine: Tera::new(glob)?,
        })
    }
}

impl Display for Templater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for template in self.engine.get_template_names() {
            writeln!(f, "{}", template.bold())?;
        }

        Ok(())
    }
}

impl Templater {
    pub fn render<T>(&self, template_name: &str, data: &T) -> Result<String>
    where
        T: Serialize,
    {
        let context = tera::Context::from_serialize(data)?;

        Ok(self.engine.render(&template_name, &context)?)
    }

    pub fn add_template<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Is it safe to unwrap?
        let template_name = &path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        self.engine.add_template_file(&path, Some(template_name))?;

        Ok(())
    }

    pub fn add_raw_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.engine.add_raw_template(&name, &template)?;

        Ok(())
    }

    pub fn render_raw<T>(&mut self, template: &str, data: &T) -> Result<String>
    where
        T: Serialize,
    {
        let context = tera::Context::from_serialize(data)?;

        Ok(self.engine.render_str(&template, &context)?)
    }
}

impl Default for Templater {
    fn default() -> Self {
        Self {
            engine: Tera::new("templates/**").expect("Default templates should not fail to load"),
        }
    }
}
