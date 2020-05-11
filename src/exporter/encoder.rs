use crate::ir::{IntermediateRepresentation, Schema, FieldRestriction};
use crate::exporter::helpers::Helpers;
use std::ops::Deref;

pub struct TSProtobufEncoder {}

impl TSProtobufEncoder {
    pub fn export(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        let mut imports: String = "\
        export function encodeArray<T, U>( encodeFct(T)=> V): (args: T[]) =>  U[] {return (args) => args.map(encodeFct)}\n\n\
        export function identity<T>( arg: T): T {return arg}\n\n\
        ".to_string();
        ir.schemas
            .iter()
            .fold(Ok(imports), | acc, schema| {
            acc.and_then(|mut output| {
                TSProtobufEncoder::format_root_schema(schema)
                    .map(| converted |{
                        output.push_str(converted.as_str());
                        output
                    })
            })
        })
    }

    fn format_root_schema(schema: &Schema) -> Result<String, &'static str> {
        match schema {
            Schema::RefSchema { .. } => { Err("Schema::RefSchema not implemented") }
            Schema::ObjectSchema { name, fields, unknown } => {
                match name {
                    Some(n) => {TSProtobufEncoder::format_object(n, fields, unknown) }
                    None => { Err("can't name a function to encode an unnamed object") }
                }
            }
            Schema::StringSchema { .. } => { Err("Schema::StringSchema not implemented") }
            Schema::IntSchema { .. } => { Err("Schema::IntSchema not implemented") }
            Schema::FloatSchema { .. } => { Err("Schema::FloatSchema not implemented") }
            Schema::BooleanSchema { .. } => { Err("Schema::BooleanSchema not implemented") }
            Schema::ArraySchema { item } => { TSProtobufEncoder::format_root_array(item) }
            Schema::OneOf { .. } => { Err("Schema::OneOf not implemented") }
        }
    }

    fn format_sub_schema(schema: &Schema) -> Result<String, &'static str> {
        match schema {
            Schema::RefSchema { name } => { TSProtobufEncoder::get_encoder(name)  }
            Schema::ObjectSchema { name, fields, unknown } => {
                match name {
                    Some(n) => { TSProtobufEncoder::get_encoder(n) }
                    None => { TSProtobufEncoder::format_object_fields(fields, unknown) }
                }
            }
            Schema::ArraySchema { item } => { TSProtobufEncoder::format_sub_array(item) }
            Schema::OneOf { .. } => { Err("Schema::OneOf not implemented") }
            _ => { Ok("".to_string())}
        }
    }

    fn format_root_array(item: &Box<Schema>) -> Result<String, &'static str> {
        //how to name this encoder ??
        Err("root Schema::ArraySchema not implemented")
    }

    fn format_sub_array(item: &Box<Schema>) -> Result<String, &'static str> {
        match item.deref() {
            Schema::RefSchema { name } => { TSProtobufEncoder::get_encoder(name) },
            Schema::ObjectSchema { name, fields, unknown } => {
                match name{
                    Some(n) => { TSProtobufEncoder::get_encoder(n)},
                    None => { TSProtobufEncoder::format_object_fields(fields, unknown) }
                }
            },
            Schema::ArraySchema { item } => {
                TSProtobufEncoder::format_sub_array(item)
                    .map(|converted|{
                        format!("encodeArray({})",converted)
                    })
            },
            Schema::OneOf { .. } => {Err("Schema::OneOf not implemented")},
            sch => { Ok("encodeArray(identity)".to_string())}
        }

    }

    fn get_encoder(name: &String) -> Result<String, &'static str> {
        Helpers::format_interface_name(name)
            .map(|interface_name| format!("encode{}", interface_name))
    }

    fn format_object(name: &String, fields: &Vec<(String, FieldRestriction)>, unknown: &bool) -> Result<String, &'static str> {
        Helpers::format_interface_name(name)
            .and_then(|interface_name|
                if *unknown {
                    TSProtobufEncoder::format_object_fields(fields,unknown)
                        .map(|converted|{
                            format!("export function encode{}(arg: {}): {{value: string}} {}\n\n",
                                    interface_name, interface_name,converted)
                        })
                } else {
                    TSProtobufEncoder::format_object_fields(fields,unknown)
                        .map(|converted|{
                            format!("export function encode{}(arg: {}): {{[x: string]: any}} {}\n\n",
                                    interface_name, interface_name,converted)
                        })
                }
            )
    }

    fn format_object_fields(fields: &Vec<(String, FieldRestriction)>, unknown: &bool) -> Result<String, &'static str> {
        if *unknown {
            Ok("{\n\treturn {value: JSON.stringify(arg)}\n}".to_string())
        } else {
            fields
                .iter()
                .fold(Ok(Vec::new()),| acc, (key, field )|{
                    acc.and_then(|mut output| {
                        TSProtobufEncoder::format_sub_schema(&field.base)
                            .map(|converted|{
                                if field.required {
                                    if converted.len() == 0 {
                                        format!("{k}: arg.{k}", k= key)
                                    }else {
                                        format!("{k}: {c}(arg.{k})", k= key, c= converted)
                                    }
                                } else {
                                    format!("{k}: arg.{k}?{c}(arg.{k}): null", k= key, c= converted)
                                }
                            })
                            .map(|converted|{
                                output.push(converted);
                                output
                            })
                    })
                })
                .map(| output|{
                    format!("{{\n\treturn {{\n\t{}\n}}\n}}", output.join(",\n\t"))
                })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Schema, FieldRestriction};

    fn merge_all_times_generated(expected: &str) -> String {
        let mut res = "\
        export function encodeArray<T, U>( encodeFct(T)=> V): (args: T[]) =>  U[] {return (args) => args.map(encodeFct)}\n\n\
        export function identity<T>( arg: T): T {return arg}\n\n\
        ".to_string();
        res.push_str(expected);
        res
    }

    #[test]
    fn export_empty_schema() {
        let expected = merge_all_times_generated("");
        let ir = IntermediateRepresentation::init();
        assert_eq!(TSProtobufEncoder::export(&ir), Ok(expected));
    }

    #[test]
    fn export_object_schema_with_primitive() {
        let expected = merge_all_times_generated("\
        export function encodeIInterface(arg: IInterface): {[x: string]: any} {\n\treturn {\n\t\
        integer: arg.integer,\n\t\
        integer2: arg.integer2,\n\t\
        comment: arg.comment\n}\n}\n\n");
        let mut ir = IntermediateRepresentation::init();
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("integer".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("integer2".to_string(), FieldRestriction { required: true, base: Schema::IntSchema { max: None, min: None, valid_values: None } }));
        fields.push(("comment".to_string(), FieldRestriction { required: true, base: Schema::StringSchema { max: None, min: None, regex: None, valid_values: None } }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(TSProtobufEncoder::export(&ir), Ok(expected));
    }


    #[test]
    fn export_array_primitive() {
        let expected = merge_all_times_generated("\
        export function encodeIInterface(arg: IInterface): {[x: string]: any} {\n\treturn {\n\t\
        arr: encodeArray(identity)(arg.arr)\n\
        }\n}\n\n");
        let mut ir = IntermediateRepresentation::init();
        let schema = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        fields.push(("arr".to_string(), FieldRestriction { required: true, base: Schema::ArraySchema { item: Box::new(schema) } }));
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: false });
        assert_eq!(TSProtobufEncoder::export(&ir), Ok(expected));
    }

//    #[test]
//    fn export_one_of() {
//        let expected = " string | number".to_owned();
//        let mut ir = IntermediateRepresentation::init();
//        let schema_string = Schema::StringSchema { max: None, min: None, regex: None, valid_values: None };
//        let schema_int = Schema::IntSchema { max: None, min: None, valid_values: None };
//        ir.add_schema(Schema::OneOf { values: vec!(schema_string, schema_int) });
//        assert_eq!(TSProtobufEncoder::export(&ir), Ok(expected));
//    }


    #[test]
    fn export_object_with_unknown() {
        let expected = merge_all_times_generated(&"export function encodeIInterface(arg: IInterface): {value: string} {\n\treturn {value: JSON.stringify(arg)}\n}\n\n");
        let mut ir = IntermediateRepresentation::init();
        let mut fields: Vec<(String, FieldRestriction)> = Vec::new();
        ir.add_schema(Schema::ObjectSchema { name: Some("Interface".to_string()), fields, unknown: true });
        assert_eq!(TSProtobufEncoder::export(&ir), Ok(expected));
    }
}