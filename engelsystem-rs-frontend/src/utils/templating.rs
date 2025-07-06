use std::collections::HashMap;

use tera::{to_value, try_get_value};

#[macro_export]
macro_rules! render_template {
    ($templates:expr, $name:expr,
     [ $( $data_name:expr => $data_val:expr ),* ]
    ) => {{
        #[allow(unused_mut)]
        let mut context = ::tera::Context::new();
        context.insert("org", "Real Org");

        $(
            context.insert($data_name, $data_val);
        )*

        $templates.render($name, &context)
            .map_err(|e| {
                tracing::error!("Template error when rendering '{}': {e:?}", $name);
                $crate::error::generated::TemplateErr.into_error(e)
            })

    }};

    ($templates:expr, $name:expr, $session:expr,
     [ $( $data_name:expr => $data_val:expr ),* ]
    ) => {{
        let mut context = ::tera::Context::new();
        $session.base_data("Real Org").insert(&mut context);

        $(
            context.insert($data_name, $data_val);
        )*

        $templates.render($name, &context)
            .map_err(|e| {
                tracing::error!("Template error when rendering '{}': {e:?}", $name);
                snafu::IntoError::into_error($crate::error::generated::TemplateErr, e)
            })

    }};
}

pub fn duration_hh_mm(
    value: &serde_json::Value,
    _: &HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let value = try_get_value!("duration_hh_mm", "value", u32, value);

    let hours = value / 3_600;
    let minutes = (value % 3_600) / 60;

    Ok(to_value(format!("{:02}:{:02}", hours, minutes)).unwrap())
}
