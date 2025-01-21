use tera::Tera;
use tera::Context;

fn main() {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => panic!("Erro parsing templates: {}.\n", e),
    };
    for (name, _item) in tera.templates.iter() {
        println!("Template: {}", &name);
        // panic!("And we are done");
    }

    let mut context = Context::new();
    context.insert("title", "From code");
    let text = match tera.render("views/child.html", &context) {
        Ok(t) => t,
        Err(e) => panic!("Error rendering view: {}.\n", e),
    };
    println!("───────── Rendered ─────────\n{}───────────────────────────\n", text);
}
