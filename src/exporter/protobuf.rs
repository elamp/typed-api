use crate::ir::{IntermediateRepresentation, Schema, FieldRestriction};
use crate::exporter::helpers::Helpers;
use std::ops::Deref;

pub struct Protobuf {}

impl Protobuf {
    pub fn export(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        ir.schemas
            .iter()
            .fold(Ok("".to_string()), |acc, schema| {
                acc.and_then(|mut output| {
                    Protobuf::format_schema(schema).map(|converted| {
                        output.push_str(converted.as_str());
                        output
                    })
                })
            })
    }

    fn format_schema(schema: &Schema) -> Result<String, &'static str> {
        match schema {
            Schema::RefSchema { name } => { Helpers::format_interface_name(name) }
            Schema::ObjectSchema { name, fields, unknown } => {
                match name {
                    Some(name) => { Protobuf::format_message(name, fields, unknown) }
                    None => { Err("Protobuf can't handle ObjectSchema without name") }
                }
            }
            Schema::StringSchema { .. } => { Ok("string".to_string()) }
            Schema::IntSchema { .. } => { Ok("int64".to_string()) }
            Schema::FloatSchema { .. } => { Ok("double".to_string()) }
            Schema::BooleanSchema { .. } => { Ok("bool".to_string()) }
            Schema::ArraySchema { item } => { Err("Root Array must never occur") }
            Schema::OneOf { values } => { Err("Root oneOf must never occur") }
        }
    }

    fn format_message(name: &String, fields: &Vec<(String, FieldRestriction)>, unknown: &bool) -> Result<String, &'static str> {
        Helpers::format_message_name(name)
            .and_then(|message_name| {
                if *unknown {
                    Ok(format!("message {} {{\n\tvalue: string\n}}\n", message_name))
                } else {
                    let mut field_count = 0;
                    fields.iter()
                        .fold(Ok(format!("message {} {{", message_name)), |acc, (k, v)| {
                            acc.and_then(|mut output| {
                                match &v.base {
                                    Schema::ArraySchema { item } => {
                                        field_count = field_count + 1;
                                        match item.deref() {
                                            Schema::ArraySchema { .. } => { Err("format Schema::ArraySchema of Schema::ArraySchema is not implemented") }
                                            Schema::OneOf { values } => {
                                                Protobuf::format_one_of(values, &1)
                                                    .map(|(added, one_of)| {
                                                        let formatted = format!(
                                                            "\n\tmessage oneOf{key} {added}{one_of}\n\t}}\n\t\trepeated oneOf{key} = {index};",
                                                            key = k, added = added, one_of = one_of, index = field_count);
                                                        output.push_str(formatted.as_str());
                                                        output
                                                    })
                                            }
                                            item_schema => {
                                                Protobuf::format_schema(item_schema)
                                                    .map(|converted| {
                                                        let formatted = format!("\n\trepeated {} {} = {};", converted, k, field_count);
                                                        output.push_str(formatted.as_str());
                                                        output
                                                    })
                                            }
                                        }
                                    }
                                    Schema::OneOf { values } => {
                                        let start_count = field_count + 1;
                                        let field_count = field_count + values.len();
                                        Protobuf::format_one_of(values, &start_count)
                                            .map(|(messages_added, one_of)| {
                                                output.push_str(messages_added.as_str());
                                                output.push_str("oneof ");
                                                output.push_str(k);
                                                output.push_str(" ");
                                                output.push_str(one_of.as_str());
                                                output
                                            })
                                    }
                                    sch => {
                                        field_count = field_count + 1;
                                        Protobuf::format_message_field(&sch, k, &field_count)
                                            .map(|converted| {
                                                output.push_str(converted.as_str());
                                                output
                                            })
                                    }
                                }
                            })
                        })
                        .map(|mut output| {
                            output.push_str("\n}\n");
                            output
                        })
                }
            })
    }

    fn format_message_field(schema: &Schema, field_name: &String, field_count: &usize) -> Result<String, &'static str> {
        match schema {
            Schema::ObjectSchema { name, fields, unknown } => {
                if *unknown {
                    Err("format Schema::OneOf is not implemented")
                } else {
                    let checked_name = name.clone().unwrap_or_else(|| format!("Unnamed{}", field_count));
                    Protobuf::format_message(&checked_name, &fields, &false)
                        .map(|rewrite_message| {
                            let mut output = "\n\t".to_string();
                            output.push_str(rewrite_message.as_str());
                            Protobuf::write_message_field(&mut output, checked_name.as_str(), field_name.as_str(), field_count);
                            output
                        })
                }
            }
            sch => {
                Protobuf::format_schema(sch)
                    .map(|field_type| {
                        let mut output = "".to_string();
                        Protobuf::write_message_field(&mut output, field_type.as_str(), field_name.as_str(), field_count);
                        output
                    })
            }
        }
    }

    fn format_one_of(schemas: &Vec<Schema>, start_index: &usize) -> Result<(String, String), &'static str> {
        let mut needed_messages = "".to_string();
        let mut fields = "{".to_string();
        let mut index = start_index.clone();
        let mut needed_messages_index = 0;
        schemas.iter().fold(Ok((needed_messages, fields)), |acc, schema| {
            acc.and_then(|(mut needed_messages, mut fields)| {
                let res = match schema {
                    Schema::ArraySchema { item } => {
                        Protobuf::format_schema(item.deref())
                            .map(|converted| {
                                needed_messages_index = needed_messages_index + 1;
                                needed_messages.push_str(format!("\n\tmessage Array{} {{repeated {} values = 1;}}", needed_messages_index, converted).as_str());
                                fields.push_str(format!("\n\t\tArray{n} oneOf{m} = {m};", n = needed_messages_index, m = index).as_str());
                                (needed_messages, fields)
                            })
                    }
                    sch => {
                        Protobuf::format_schema(sch)
                            .map(|converted| {
                                fields.push_str(format!("\n\t\t{} oneOf{m} = {m};", n = converted, m = index).as_str());
                                (needed_messages, fields)
                            })
                    }
                };
                index = index + 1;
                res
            })
        })
            .map(|(mut needed_messages, mut fields)| {
                if needed_messages.len() > 0 {
                    needed_messages.push_str("\n\t")
                }
                fields.push_str("\n\t}");
                (needed_messages, fields)
            })
    }

    fn write_message_field(output: &mut String, field_type: &str, field_name: &str, field_count: &usize) -> () {
        output.push_str("\n\t");
        output.push_str(field_name);
        output.push_str(" = ");
        output.push_str(field_count.to_string().as_str());
        output.push_str(";");
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
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_object_schema_with_primitive() {
        let expected = "message Interface {\n\tinteger = 1;\n\tinteger2 = 2;\n\tcomment = 3;\n}\n".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("integer".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("integer2".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: Schema::StringSchema { max: None, min: None, regex: None, valid_values: None } }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_string_regex_primitive() {
        let expected = "string".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let mut schema = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        ir.add_schema(schema);
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_int_primitive() {
        let expected = "int64".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::IntSchema { max: None, min: None, valid_values: None };
        ir.add_schema(schema);
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_float_primitive() {
        let expected = "double".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::FloatSchema { max: None, min: None };
        ir.add_schema(schema);
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_boolean_primitive() {
        let expected = "bool".to_owned();
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::BooleanSchema {};
        ir.add_schema(schema);
        assert_eq!(Protobuf::export(&ir), Ok(expected));
    }

    #[test]
    fn export_array_primitive() {
        let expected = Ok("message Interface {\n\trepeated string comment = 1;\n}\n".to_owned());
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        let array_schema = Schema::ArraySchema { item: Box::new(schema) };
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: array_schema }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(Protobuf::export(&ir), expected);
    }

    #[test]
    fn export_one_of_array() {
        let expected = Ok("message Interface {\n\tmessage Array1 {repeated string values = 1;}\n\toneof comment {\n\t\tstring oneOf1 = 1;\n\t\tint64 oneOf2 = 2;\n\t\tArray1 oneOf3 = 3;\n\t}\n}\n".to_owned());
        let mut ir = IntermediateRepresentation::init();
        let schema_string = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        let schema_int = Schema::IntSchema { max: None, min: None, valid_values: None };
        let schema_array = Schema::ArraySchema { item: Box::new(Schema::StringSchema { max: None, min: None, regex: None, valid_values: None }) };
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: Schema::OneOf { values: vec!(schema_string, schema_int, schema_array) } }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(Protobuf::export(&ir), expected);
    }

    #[test]
    fn export_array_one_of() {
        let expected = Ok("message Interface {\n\tmessage oneOfcomment {\n\t\tstring oneOf1 = 1;\n\t\tint64 oneOf2 = 2;\n\t}\n\t}\n\t\trepeated oneOfcomment = 1;\n}\n".to_owned());
        let mut ir = IntermediateRepresentation::init();
        let schema_string = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        let schema_int = Schema::IntSchema { max: None, min: None, valid_values: None };
        let schema_one_of = Schema::OneOf { values: vec!(schema_string, schema_int) };
        let schema_array = Schema::ArraySchema { item: Box::new(schema_one_of) };
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: schema_array }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(Protobuf::export(&ir), expected);
    }

    #[test]
    fn export_object_with_unknown() {
        let expected = Ok("message Interface {\n\tvalue: string\n}\n".to_owned());
        let mut ir = IntermediateRepresentation::init();
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: true });
        assert_eq!(Protobuf::export(&ir), expected);
    }
}