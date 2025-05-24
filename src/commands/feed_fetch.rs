use nu_json::value::ToJson;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    IntoPipelineData, LabeledError, PipelineData, Signature, Span, Spanned, SyntaxShape, Type,
    Value, category_from_string,
};

use crate::util::get_resource_content;

pub struct FeedFetch;

impl PluginCommand for FeedFetch {
    type Plugin = crate::WebfeedPlugin;

    fn name(&self) -> &str {
        "feed fetch"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::record())
            .required(
                "url",
                SyntaxShape::String,
                "link to the feed, or a local file",
            )
            .category(category_from_string("webfeed"))
    }

    fn description(&self) -> &str {
        "Convert an RSS, ATOM, or JSONFeed feed to structured data."
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let resource = call.req::<Spanned<String>>(0)?;

        let raw_feed =
            get_resource_content(&resource.item).map_err(|e| LabeledError::new(format!("{e}")))?;

        let feed = match feed_rs::parser::parse(raw_feed.as_bytes()) {
            Ok(f) => f,
            Err(err) => return Err(LabeledError::new(format!("Error parsing feed: {err}"))),
        };

        Ok(nujson_to_value(feed.to_json(), call.head).into_pipeline_data())
    }
}

fn nujson_to_value(feed: nu_json::Value, span: Span) -> Value {
    match feed {
        nu_json::Value::Null => Value::nothing(span),
        nu_json::Value::Bool(b) => Value::bool(b, span),
        nu_json::Value::I64(i) => Value::int(i, span),
        nu_json::Value::U64(u) => {
            if u > i64::MAX as u64 {
                Value::error(
                    nu_protocol::ShellError::CantConvert {
                        to_type: "i64-sized integer".into(),
                        from_type: "value larger than i64".into(),
                        span,
                        help: None,
                    },
                    span,
                )
            } else {
                Value::int(u as i64, span)
            }
        }
        nu_json::Value::F64(f) => Value::float(f, span),
        nu_json::Value::String(s) => Value::string(s, span),
        nu_json::Value::Array(values) => Value::list(
            values
                .into_iter()
                .map(|x| nujson_to_value(x, span))
                .collect(),
            span,
        ),
        nu_json::Value::Object(map) => Value::record(
            map.into_iter()
                .map(|(k, v)| (k, nujson_to_value(v, span)))
                .collect(),
            span,
        ),
    }
}
