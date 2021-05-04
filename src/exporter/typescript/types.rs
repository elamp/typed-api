use openapiv3::{OpenAPI, ReferenceOr, PathItem, Parameter, ParameterSchemaOrContent, SchemaKind, ArrayType, Type, Schema, Response, StatusCode, MediaType, Components, Operation};
use openapiv3::ReferenceOr::{Item, Reference};
use indexmap::map::IndexMap;

enum  HTTP_VERB {
    GET,
    DELETE,
    LIST,
    POST,
    PUT,
}

pub struct TSTypes {}

impl TSTypes {
    pub fn export(ir: &OpenAPI) -> Result<String, String> {
        TSTypes::generate_service_client(ir)
    }

    fn generate_service_client(ir: &OpenAPI) -> Result<String, String> {
        let mut counter: i64 = 0;
        let mut export = "".to_string();
        match &ir.components {
            Some(components) => {
                TSTypes::generate_components_schemas(&mut export, components)
            }
            None => {
                //do nothing
            }
        }
        export = ir.paths.iter().fold(
            export,
            |mut acc: String, (key, value): (&String, &ReferenceOr<PathItem>)| {
                match value {
                    Item(path_data) => {
                        (path_data.get)
                            .as_ref()
                            .map(|operation| {
                                let verb = HTTP_VERB::GET;
                                let id = TSTypes::get_name_from_operation(counter, &verb, &operation);
                                TSTypes::write_query_interface(&mut acc, operation, &id);
                                TSTypes::write_return_type(&mut acc, &operation, &id);
                            });
                        (path_data.post)
                            .as_ref()
                            .map(|operation| {
                                let verb = HTTP_VERB::POST;
                                let id = TSTypes::get_name_from_operation(counter, &verb, &operation);
                                TSTypes::write_query_interface(&mut acc, operation, &id);
                                TSTypes::write_return_type(&mut acc, &operation, &id);
                            });
                    }
                    Reference { reference } => {
                        println!("pas de value !!! {}", reference);
                    }
                };
                acc
            });
        Ok(export)
    }

    fn write_query_interface(mut acc: &mut String, operation: &Operation, id: &String) {
        let mut possibles_types = Vec::new();
        let mut fields = Vec::new();
        for parameter_ref_or_item in operation.parameters.iter() {
            match parameter_ref_or_item {
                ReferenceOr::Item(parameter) => {
                    fields.push(parameter);
                }
                ReferenceOr::Reference { reference } => {
                    possibles_types.push(reference.to_string());
                }
            }
        }
        for parameter_ref_or_item in operation.request_body.iter() {
            match parameter_ref_or_item {
                ReferenceOr::Item(body) => {
                    possibles_types.push(TSTypes::parse_content_map(&body.content))
                }
                ReferenceOr::Reference { reference } => {
                    possibles_types.push(reference.to_string());
                }
            }
        }
        if(fields.len() > 0) {
            let mut interface = "{\n\t".to_string();
            for parameter in fields.iter() {
                match parameter {
                    Parameter::Query { parameter_data, allow_reserved: _, style: _, allow_empty_value: _ } => {
                        interface.push_str(&parameter_data.name);
                        interface.push_str(": ");
                        match &parameter_data.format {
                            ParameterSchemaOrContent::Schema(ref_or_item) => {
                                interface.push_str(&TSTypes::write_ref_or_schema_as_js_type(ref_or_item))
                            }
                            ParameterSchemaOrContent::Content(_content) => {}
                        }
                    }
                    _ => {}
                }
            }
            interface.push_str("\n}");
            possibles_types.push(interface)
        }
        acc.push_str("export type ");
        acc.push_str(&id);
        acc.push_str("Query = ");
        acc.push_str(&possibles_types.join(" | "));
        acc.push_str("\n\n");
    }

    fn write_return_type(mut acc: &mut String, operation: &&Operation, id: &String) {
        acc.push_str("export type ");
        acc.push_str(&id);
        acc.push_str("Return = ");
        let map = &operation.responses.responses;
        for tuple in map.iter() {
            let (status, response): (&StatusCode, &ReferenceOr<Response>) = tuple;
            match response {
                ReferenceOr::Item(response) => {
                    acc.push_str(&TSTypes::parse_content_map(&response.content));
                }
                ReferenceOr::Reference { reference: _ } => {
                    //do nothing
                }
            }
        }
        acc.push_str("\n\n");
    }

    fn parse_content_map(content: &IndexMap<String, MediaType>) ->  String {
        let mut acc = "".to_string();
        for app_content in content.iter() {
            let (app_type, content): (&String, &MediaType) = app_content;
            match app_type.as_str() {
                "application/json" => {
                    match &content.schema {
                        Some(ReferenceOr::Item(schema)) => {
                            acc.push_str(&TSTypes::write_schema_as_js_type(&schema));
                        }
                        Some(ReferenceOr::Reference { reference }) => {
                            acc.push_str(&TSTypes::get_type_from_reference(&reference))
                        }
                        None => {}
                    }
                }
                _ => {
                    println!("{} isn't supported for now", app_type);
                }
            }
        }
        return acc
    }

    fn get_name_from_operation(mut counter: i64, http_verb: &HTTP_VERB,  operation: &&Operation) -> String {
        match &operation.operation_id {
            Some(value) => value.to_owned(),
            None => {
                let body = match http_verb {
                    HTTP_VERB::GET => {"get"}
                    HTTP_VERB::DELETE => {"delete"}
                    HTTP_VERB::LIST => {"list"}
                    HTTP_VERB::POST => {"post"}
                    HTTP_VERB::PUT => {"put"}
                };
                counter = counter + 1;
                format!("{}{}", body, counter)
            }
        }
    }

    fn generate_components_schemas(export: &mut String, components: &Components) {
        for key_value in components.schemas.iter() {
            let (key, value): (&String, &ReferenceOr<Schema>) = key_value;
            export.push_str("export interface ");
            export.push_str(&key);
            export.push_str(" ");
            export.push_str(&TSTypes::write_ref_or_schema_as_js_type(value));
        }
        export.push_str(&"\n\n")
    }

    fn write_ref_or_schema_as_js_type(ref_or_item: &ReferenceOr<Schema>) -> String {
        match ref_or_item {
            ReferenceOr::Item(schema) => {
                TSTypes::write_schema_as_js_type(&schema)
            }
            ReferenceOr::Reference { reference } => {
                TSTypes::get_type_from_reference(reference)
            }
        }
    }

    fn write_boxed_ref_or_schema_as_js_type(ref_or_item: &ReferenceOr<Box<Schema>>) -> String {
        match ref_or_item {
            ReferenceOr::Item(schema) => {
                TSTypes::write_schema_as_js_type(schema.as_ref())
            }
            ReferenceOr::Reference { reference } => {
                TSTypes::get_type_from_reference(reference)
            }
        }
    }

    fn write_schema_as_js_type(schema: &Schema) -> String {
        match &schema.schema_kind {
            SchemaKind::Type(Type::Array(ArrayType { items, min_items: _, max_items: _, unique_items: _ })) => {
                let mut arr_type = TSTypes::write_boxed_ref_or_schema_as_js_type(items);
                arr_type.push_str("[]");
                arr_type
            }
            SchemaKind::Type(Type::String(_string_type)) => {
                "string".to_string()
            }
            SchemaKind::Type(Type::Number(_number_type)) => {
                "number".to_string()
            }
            SchemaKind::Type(Type::Integer(_integer_type)) => {
                "number".to_string()
            }
            SchemaKind::Type(Type::Object(object)) => {
                let mut acc = "{\n\t".to_owned();
                let mut fields: Vec<String> = Vec::new();
                for key_value in object.properties.iter() {
                    let (key, value): (&String, &ReferenceOr<Box<Schema>>) = key_value;
                    let mut field = key.to_owned();
                    if object.required.contains(key) {
                        field.push_str(&": ");
                    } else {
                        field.push_str(&"?: ");
                    }
                    field.push_str(&TSTypes::write_boxed_ref_or_schema_as_js_type(value));
                    fields.push(field);
                }
                acc.push_str(&fields.join(",\n\t"));
                acc.push_str(&"\n}");
                acc
            }
            SchemaKind::Type(Type::Boolean {}) => {
                "boolean".to_string()
            }
            SchemaKind::OneOf { one_of: _ } => {
                // one_of: Vec<ReferenceOr<Schema>>
                "".to_string()
            }
            SchemaKind::AllOf { all_of: _ } => {
                // all_of: Vec<ReferenceOr<Schema>>,
                "".to_string()
            }
            SchemaKind::AnyOf { any_of: _ } => {
                // any_of: Vec<ReferenceOr<Schema>>
                "".to_string()
            }
            SchemaKind::Any(_any_schema) => {
                "any".to_string()
            }
        }
    }

    fn get_type_from_reference(reference: &String) -> String {
        let parts = reference.split("/");
        parts.last().unwrap_or("any").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use std::io::BufReader;
    use std::fs::File;
    use std::fs;


    #[test]
    fn export_empty_schema() {
        let expected = "";
        let ir = serde_json::from_str("{\"openapi\": \"3.0.0\",\"info\": {\"title\": \"test\",\"description\": \"Descr\",\"version\": \"0.0.1\"},\"paths\": {}}").expect("Could not deserialize input");
        assert_eq!(TSTypes::export(&ir), Ok(expected.to_owned()));
    }

    #[test]
    fn export_simple_schema() -> Result<(), String> {
        let expected = "export interface Root {\
                \n\tname: string,\
                \n\trarity?: number\
                \n}\
                \n\nexport type listRootsQuery = {\
                \n\tfields: string[]\
                \n}\
                \n\nexport type listRootsReturn = {\
                \n\tcount: number,\
                \n\tdata: Root[]\
                \n}\
                \n\nexport type createRootQuery = Root\
                \n\nexport type createRootReturn = Root\
                \n\n";
        let file = File::open("./data-tests/simple-open-api-3.0.json").expect("Invalid testFile");
        let reader = BufReader::new(file);
        let ir = serde_json::from_reader(reader).expect("Could not deserialize input");
        assert_eq!(TSTypes::export(&ir), Ok(expected.to_owned()));
        Ok(())
    }
}