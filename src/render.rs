use tera::Tera;
use tera::Context;

pub struct Renderer {
    #[allow(dead_code)]
    source: String,
    engine: Tera,
}

impl Renderer {
    pub fn new(source: String) -> Self {
        let engine = match Tera::new(&source) {
            Ok(tera) => tera,
            Err(err) => panic!(
                "Error parsing templates from {}: {}.",
                source,
                err
            ),
        };

        Renderer {
            source,
            engine,
        }
    }

    pub fn render(&self, template: &str, context: &Context) -> String {
        match self.engine.render(template, context) {
            Ok(s) => s,
            Err(err) => panic!("Failed to render {}: {}", template, err),
        }
    }
}

