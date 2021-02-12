mod util;
use util::*;

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, NumberValidation, Schema, SchemaObject},
    JsonSchema,
};

#[derive(Debug, JsonSchema)]
pub struct Struct {
    foo: Replicas,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Replicas(u32);
impl JsonSchema for Replicas {
    fn schema_name() -> String {
        "custom".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        // serde_json::from_value(serde_json::json!({
        //     "type": "integer",
        //     "minimum": 0,
        //     "maximum": 10,
        // }))
        // .unwrap()
        SchemaObject::builder()
            .instance_type(InstanceType::Integer)
            .number_validation(
                NumberValidation::builder()
                    .minimum(0)
                    .maximum(10)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
            .into()
    }
}

#[test]
fn builder_custom() -> TestResult {
    test_default_generated_schema::<Struct>("builder-custom")
}
