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
