use openapiv3::{OpenAPI, ReferenceOr, PathItem, Parameter, ParameterSchemaOrContent, SchemaKind, ArrayType, Type, Schema, Response, StatusCode, MediaType};
use openapiv3::ReferenceOr::{Item, Reference};
use crate::exporter::TypescriptHTTP;

pub struct TSExport {
    pub imports: String,
    pub types: String,
    pub parameters: String,
    pub content: String
}

impl TSExport {
    pub fn new(imports: String, types: String, parameters: String, content: String) -> TSExport{
        TSExport { imports, types, parameters, content}
    }
}

pub struct TSHTTPClient{}

impl TSHTTPClient {
    pub fn export(ir: &OpenAPI) -> Result<String, String> {
        TSHTTPClient::generate_service_client(ir)
            .map(|export| {
                let mut output = "import * as express from 'express'\
                \nimport { Router } from 'express'\
                \n\nimport { Request, RequestMetadata, UseCase } from './types'".to_owned();
                if export.imports.len() > 0 {
                    output.push_str(&export.imports)
                }
                output.push_str("\n\n");
                if export.types.len() > 0 {
                    output.push_str(&export.types);
                    output.push_str("\n\n");
                }
                output.push_str("export function buildRouter(");
                output.push_str(&export.parameters);
                output.push_str(") {");
                output.push_str(&export.content);
                output.push_str("\n\treturn router\n}");
                output.to_owned()
            })
    }

    fn generate_service_client(ir: &OpenAPI) -> Result<TSExport, String> {
        let mut counter: i64 = 0;
        let mut export = TSExport::new("".to_owned(), "".to_owned(), "".to_owned(), "".to_owned());
        match &ir.components {
            Some(components) => {
                for key_value in components.schemas.iter() {
                    let (key, value): (&String, &ReferenceOr<Schema>) = key_value;
                    export.types.push_str("export interface ");
                    export.types.push_str(&key);
                    export.types.push_str(" ");
                    export.types.push_str(&TSHTTPClient::write_ref_or_schema_as_js_type(value));
                }
                export.types.push_str(&"\n\n")
            },
            None => {
                //do nothing
            }
        }
        export = ir.paths.iter().fold(
            export,
            |mut acc: TSExport, (key, value): (&String, &ReferenceOr<PathItem>)| {
                println!("ça map, {}", key);
                match value {
                    Item(path_data) => {
                        (path_data.get)
                            .as_ref()
                            .map(|operation| {
                                let id = match &operation.operation_id {
                                    Some(value) => value.to_owned(),
                                    None => {
                                        counter = counter + 1;
                                        format!("create{}", counter)
                                    }
                                };
                                acc.types.push_str("export interface ");
                                acc.types.push_str(&id);
                                acc.types.push_str("Query {\n\t");
                                for parameter_ref_or_item in operation.parameters.iter() {
                                    match parameter_ref_or_item {
                                        ReferenceOr::Item(parameter) => {
                                            println!("ReferenceOr::Item {:?}", parameter);
                                            match parameter {
                                                Parameter::Query{ parameter_data, allow_reserved: _, style: _, allow_empty_value: _ } => {
                                                    acc.types.push_str(&parameter_data.name);
                                                    acc.types.push_str(": ");
                                                    match &parameter_data.format {
                                                        ParameterSchemaOrContent::Schema (ref_or_item) => {
                                                            acc.types.push_str(&TSHTTPClient::write_ref_or_schema_as_js_type(ref_or_item))
                                                        }
                                                        ParameterSchemaOrContent::Content(_content) => {

                                                        }
                                                    }
                                                },
                                                _ => {

                                                }
                                            }
                                        }
                                        ReferenceOr::Reference { reference } => {
                                            println!("ReferenceOr::Reference {:?}", reference);
                                        }
                                    }
                                }
                                acc.types.push_str("\n}\n\n");


                                acc.types.push_str("export type ");
                                acc.types.push_str(&id);
                                acc.types.push_str("Return = ");
                                let map= &operation.responses.responses;
                                for tuple in map.iter() {
                                    let (status, response): (&StatusCode, &ReferenceOr<Response>) = tuple;
                                    println!("status {}, reponse ", status);
                                    match response {
                                        ReferenceOr::Item(response) => {
                                           for app_content in response.content.iter() {
                                               let (app_type, content): (&String, &MediaType) = app_content;
                                               match app_type.as_str() {
                                                   "application/json" => {
                                                       match &content.schema {
                                                           Some(ReferenceOr::Item(schema)) => {
                                                               acc.types.push_str(&TSHTTPClient::write_schema_as_js_type(&schema));
                                                           }
                                                           Some(ReferenceOr::Reference { reference }) => {
                                                               acc.types.push_str(&TSHTTPClient::get_type_from_reference(&reference))
                                                           }
                                                           None => {

                                                           }
                                                       }
                                                   }
                                                   _ => {
                                                       println!("{} isn't supported for now", app_type);
                                                   }
                                               }
                                           }
                                        },
                                        ReferenceOr::Reference {reference: _} => {
                                            //do nothing
                                        }
                                    }
                                }

                                acc.parameters.push_str(&id);
                                acc.parameters.push_str(": UseCase<");
                                acc.parameters.push_str(&id);
                                acc.parameters.push_str("Query, ");
                                acc.parameters.push_str(&id);
                                acc.parameters.push_str("Return>, router: Router = express.Router()");
                                acc.content.push_str("\n\trouter.get('");
                                acc.content.push_str(&key);
                                acc.content.push_str("', (req: Request<listRootsQuery, {}>, res, next) => {\n\t\t");
                                acc.content.push_str(&id);
                                acc.content.push_str("\n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.query)");
                                acc.content.push_str("\n\t\t\t.then(result => res.json(result), error => res.send())");
                                acc.content.push_str("\n\t})");
                            });
                    },
                    Reference { reference } => {
                        println!("pas de value !!! {}", reference);
                    }
                };
                acc
            });
        Ok(export)
    }

    fn write_ref_or_schema_as_js_type(ref_or_item: &ReferenceOr<Schema>) -> String {
        match ref_or_item {
            ReferenceOr::Item(schema) => {
                TSHTTPClient::write_schema_as_js_type(&schema)
            }
            ReferenceOr::Reference { reference } => {
                TSHTTPClient::get_type_from_reference(reference)
            }
        }
    }

    fn write_boxed_ref_or_schema_as_js_type(ref_or_item: &ReferenceOr<Box<Schema>>) -> String {
        match ref_or_item {
            ReferenceOr::Item(schema) => {
                TSHTTPClient::write_schema_as_js_type(schema.as_ref())
            }
            ReferenceOr::Reference { reference } => {
                TSHTTPClient::get_type_from_reference(reference)
            }
        }
    }

    fn write_schema_as_js_type(schema: &Schema) -> String {
        match &schema.schema_kind {
            SchemaKind::Type(Type::Array(ArrayType { items, min_items: _, max_items: _, unique_items: _ })) => {
                let mut arr_type = TSHTTPClient::write_boxed_ref_or_schema_as_js_type(items);
                arr_type.push_str("[]");
                arr_type
            },
            SchemaKind::Type(Type::String(_string_type)) => {
                "string".to_string()
            },
            SchemaKind::Type(Type::Number(_number_type)) => {
                "number".to_string()
            },
            SchemaKind::Type(Type::Integer(_integer_type)) => {
                "number".to_string()
            },
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
                    field.push_str(&TSHTTPClient::write_boxed_ref_or_schema_as_js_type( value));
                    fields.push(field);
                }
                acc.push_str(&fields.join(",\n\t"));
                acc.push_str(&"\n}");
                acc
            },
            SchemaKind::Type(Type::Boolean {}) => {
                "boolean".to_string()
            },
            SchemaKind::OneOf { one_of: _ } => {
                // one_of: Vec<ReferenceOr<Schema>>
                "".to_string()
            },
            SchemaKind::AllOf { all_of: _ } => {
                // all_of: Vec<ReferenceOr<Schema>>,
                "".to_string()
            },
            SchemaKind::AnyOf { any_of: _ } => {
                // any_of: Vec<ReferenceOr<Schema>>
                "".to_string()
            },
            SchemaKind::Any(_any_schema) => {
                "any".to_string()
            }
        }
    }

    fn get_type_from_reference(reference: &String ) -> String {
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
        let expected = "import * as express from 'express'\
                \nimport { Router } from 'express'\
                \n\nimport { Request, RequestMetadata, UseCase } from './types'\
                \n\nexport function buildRouter() {\
                \n\treturn router\
                \n}";
        let ir = serde_json::from_str("{\"openapi\": \"3.0.0\",\"info\": {\"title\": \"test\",\"description\": \"Descr\",\"version\": \"0.0.1\"},\"paths\": {}}").expect("Could not deserialize input");
        assert_eq!(TSHTTPClient::export(&ir), Ok(expected.to_owned()));
    }

    #[test]
    fn export_y_schema() -> Result<(), String> {
        let expected = "import * as express from 'express'\
                \nimport { Router } from 'express'\
                \n\nimport { Request, RequestMetadata, UseCase } from './types'\
                \n\nexport interface Root {\
                \n\tname: string,\
                \n\trarity?: number\
                \n}\
                \n\nexport interface listRootsQuery {\
                \n\tfields: string[]\
                \n}\
                \n\nexport type listRootsReturn = {\
                \n\tcount: number,\
                \n\tdata: Root[]\
                \n}\
                \n\nexport function buildRouter(listRoots: UseCase<listRootsQuery, listRootsReturn>, router: Router = express.Router()) {\
                \n\trouter.get('/roots', (req: Request<listRootsQuery, {}>, res, next) => {\
                \n\t\tlistRoots\
                \n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.query)\
                \n\t\t\t.then(result => res.json(result), error => res.send())\
                \n\t})\
                \n\treturn router\
                \n}";
        let file = File::open("./src/exporter/clients/data-tests/simple/simple-open-api-3.0.json").expect("Invalid testFile");
        let reader = BufReader::new(file);
        let ir = serde_json::from_reader(reader).expect("Could not deserialize input");
        assert_eq!(TSHTTPClient::export(&ir), Ok(expected.to_owned()));
        Ok(())
    }
}