use crate::ir::{IntermediateRepresentation, Schema, FieldRestriction};
use std::ops::Deref;
use crate::exporter::helpers::Helpers;

pub struct TypescriptInterface {}

impl TypescriptInterface {
    pub fn export(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        ir.schemas
            .iter()
            .fold(Ok("".to_string()), |acc, schema| {
                acc.and_then(|mut output| {
                    TypescriptInterface::format_schema(schema).map(|converted| {
                        output.push_str(converted.as_str());
                        output
                    })
                })
            })
    }

    fn format_schema(schema: &Schema) -> Result<String, &'static str> {
        match schema {
            Schema::RefSchema { name } => {
                Helpers::format_interface_name(name)
            }
            Schema::ObjectSchema { name, fields, unknown: _ } => {
                match name {
                    Some(value) => { TypescriptInterface::format_interface(value, fields) }
                    None => { TypescriptInterface::format_object(fields) }
                }
            }
            Schema::StringSchema { .. } => {
                let mut output = "".to_string();
                output.push_str(" string");
                Ok(output)
            }
            Schema::IntSchema { .. } => {
                let mut output = "".to_string();
                output.push_str(" number");
                Ok(output)
            }
            Schema::FloatSchema { .. } => {
                let mut output = "".to_string();
                output.push_str(" number");
                Ok(output)
            }
            Schema::BooleanSchema { .. } => {
                let mut output = "".to_string();
                output.push_str(" boolean");
                Ok(output)
            }
            Schema::ArraySchema { item } => {
                TypescriptInterface::format_schema(item.deref()).map(|mut converted| {
                    converted.push_str("[]");
                    converted
                })
            }
            Schema::OneOf { values } => { TypescriptInterface::format_one_of(values) }
        }
    }

    fn format_interface(name: &String, fields: &Vec<(String, FieldRestriction)>) -> Result<String, &'static str> {
        Helpers::format_interface_name(name)
            .and_then(|name| {
                TypescriptInterface::format_object(fields).map(|converted| {
                    let mut output = "\n\nexport interface ".to_string();
                    output.push_str(name.as_str());
                    output.push_str(converted.as_str());
                    output
                })
            })
    }

    fn format_object(fields: &Vec<(String, FieldRestriction)>) -> Result<String, &'static str> {
        fields.iter()
            .fold(Ok(" {".to_string()), |acc, (k, v)| {
                acc.and_then(|mut output| {
                    TypescriptInterface::format_schema(&v.base).map(|converted| {
                        output.push_str("\n\t");
                        output.push_str(k.as_str());
                        output.push_str(":");
                        output.push_str(converted.as_str());
                        output
                    })
                })
            })
            .map(|mut output| {
                output.push_str("\n}");
                output
            })
    }

    fn format_one_of(schemas: &Vec<Schema>) -> Result<String, &'static str> {
        schemas.iter()
            .fold(Ok(Vec::new()), |acc, sch| {
                acc
                    .and_then(|mut output| {
                        TypescriptInterface::format_schema(sch).map(|converted| {
                            output.push(converted);
                            output
                        })
                    })
            })
            .map(|v| v.join(" |"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Schema, FieldRestriction};

    #[test]
    fn export_empty_schema() {
        let expected = "".to_owned();
        let ir = IntermediateRepresentation::init();
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_object_schema_with_primitive() {
        let expected = "\n\nexport interface IInterface {\n\tinteger: number\n\tinteger2: number\n\tcomment: string\n}".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("integer".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("integer2".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: Schema::StringSchema { max: None, min: None, regex: None, valid_values: None } }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: true });
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_string_regex_primitive() {
        let expected = " string".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let mut schema = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        ir.add_schema(schema);
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_int_primitive() {
        let expected = " number".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::IntSchema { max: None, min: None, valid_values: None };
        ir.add_schema(schema);
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_float_primitive() {
        let expected = " number".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::FloatSchema { max: None, min: None };
        ir.add_schema(schema);
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_boolean_primitive() {
        let expected = " boolean".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::BooleanSchema {};
        ir.add_schema(schema);
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_array_primitive() {
        let expected = " string[]".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        ir.add_schema(Schema::ArraySchema { item: Box::new(schema) });
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }

    #[test]
    fn export_one_of() {
        let expected = " string | number".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema_string = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        let schema_int = Schema::IntSchema { max: None, min: None, valid_values: None };
        ir.add_schema(Schema::OneOf { values: vec!(schema_string, schema_int) });
        assert_eq!(TypescriptInterface::export(&ir), Ok(expected));
    }
}