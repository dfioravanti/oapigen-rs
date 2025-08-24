use anyhow::{Context, Result};
use handlebars::Handlebars;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates"]
struct HandlebarsTemplate;

pub const STRUCT_TEMPLATE_NAME: &str = "struct";

pub fn handlebars<'a>() -> Result<Handlebars<'a>> {
    let mut handlebars = Handlebars::new();

    for template_filename in HandlebarsTemplate::iter() {
        let file = HandlebarsTemplate::get(template_filename.as_ref())
            .with_context(|| format!("Failed to load file {:?}", template_filename))?;

        let template_content = std::str::from_utf8(file.data.as_ref())
            .with_context(|| format!("Failed to load content of file {:?}", template_filename))?;

        // the template_filename includes the extension, which we need to drop
        let template_name = template_filename.clone().replace(".hbs", "");
        handlebars
            .register_template_string(template_name.as_ref(), template_content)
            .with_context(|| {
                format!(
                    "Failed to register template for file {:?}",
                    template_filename
                )
            })?;
    }

    Ok(handlebars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let handlebars = handlebars();

        handlebars.unwrap();
    }
}
