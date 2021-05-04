use openapiv3::{OpenAPI, ReferenceOr, PathItem, Parameter, ParameterSchemaOrContent, SchemaKind, ArrayType, Type, Schema, Response, StatusCode, MediaType, Components, Operation};
use openapiv3::ReferenceOr::{Item, Reference};

enum  HTTP_VERB {
    GET,
    DELETE,
    POST,
    PUT,
}

pub struct TSExport {
    pub imports: String,
    pub parameters: String,
    pub content: String,
}

impl TSExport {
    pub fn new(imports: String, parameters: String, content: String) -> TSExport {
        TSExport { imports, parameters, content }
    }
}

pub struct TSHTTPServer {}

impl TSHTTPServer {
    pub fn export(ir: &OpenAPI) -> Result<String, String> {
        TSHTTPServer::generate_service_server(ir)
            .map(|export| {
                let mut output = "import * as express from 'express'\
                \nimport { Router } from 'express'\
                \n\nimport { Request, RequestMetadata, UseCase } from './genericsTypes'\
                \n\nimport {".to_owned();
                if export.imports.len() > 0 {
                    output.push_str(&export.imports)
                }
                output.push_str("\n} from './types'\n\n");
                output.push_str("export function buildRouter(");
                output.push_str(&export.parameters);
                output.push_str("\n\t\t\trouter: Router = express.Router()\n\t\t) {");
                output.push_str(&export.content);
                output.push_str("\n\treturn router\n}");
                output.to_owned()
            })
    }

    fn generate_service_server(ir: &OpenAPI) -> Result<TSExport, String> {
        let mut counter: i64 = 0;
        let mut export = TSExport::new("".to_owned(), "".to_owned(), "".to_owned());
        export = ir.paths.iter().fold(
            export,
            |mut acc: TSExport, (key, value): (&String, &ReferenceOr<PathItem>)| {
                match value {
                    Item(path_data) => {
                        TSHTTPServer::write_handle_for_http_verb(counter, &mut acc, &key, &path_data.delete, &HTTP_VERB::DELETE);
                        TSHTTPServer::write_handle_for_http_verb(counter, &mut acc, &key, &path_data.get, &HTTP_VERB::GET);
                        TSHTTPServer::write_handle_for_http_verb(counter, &mut acc, &key, &path_data.post, &HTTP_VERB::POST);
                        TSHTTPServer::write_handle_for_http_verb(counter, &mut acc, &key, &path_data.put, &HTTP_VERB::PUT);
                    }
                    Reference { reference } => {
                        println!("pas de value !!! {}", reference);
                    }
                };
                acc
            });
        Ok(export)
    }

    fn write_handle_for_http_verb(mut counter: i64, mut acc: &mut TSExport, key: &&String, get: &Option<Operation>, verb: &HTTP_VERB) {
        get
            .as_ref()
            .map(|operation| {
                let id = TSHTTPServer::get_name_from_operation(counter, &verb, &operation);
                TSHTTPServer::write_build_router_part(&mut acc, &verb, &key, &id);
            });
    }

    fn write_build_router_part(mut acc: &mut TSExport, http_verb: &HTTP_VERB, key: &&String, id: &String) {
        acc.imports.push_str("\n\t");
        acc.imports.push_str(&id);
        acc.imports.push_str("Query,\n\t");
        acc.imports.push_str(&id);
        acc.imports.push_str("Return,");

        acc.parameters.push_str("\n\t\t\t");
        acc.parameters.push_str(&id);
        acc.parameters.push_str(": UseCase<");
        acc.parameters.push_str(&id);
        acc.parameters.push_str("Query, ");
        acc.parameters.push_str(&id);
        acc.parameters.push_str("Return>,");

        let param = match http_verb {
            HTTP_VERB::GET => {
                acc.content.push_str("\n\trouter.get('");
                acc.content.push_str(&key);
                acc.content.push_str("', (req: Request<");
                acc.content.push_str(&id);
                acc.content.push_str("Query>, res, next) => {\n\t\t");
                acc.content.push_str(&id);
                acc.content.push_str("\n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.");
                acc.content.push_str("query");
                }
            HTTP_VERB::DELETE => {
                acc.content.push_str("\n\trouter.delete('");
                acc.content.push_str(&key);
                acc.content.push_str("', (req: Request<");
                acc.content.push_str(&id);
                acc.content.push_str("Query>, res, next) => {\n\t\t");acc.content.push_str(&id);
                acc.content.push_str("\n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.");
                acc.content.push_str("query");
            }
            HTTP_VERB::POST => {
                acc.content.push_str("\n\trouter.post('");
                acc.content.push_str(&key);
                acc.content.push_str("', (req: Request<");
                acc.content.push_str(&id);
                acc.content.push_str("Query>, res, next) => {\n\t\t");acc.content.push_str(&id);
                acc.content.push_str("\n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.");
                acc.content.push_str("body");
            }
            HTTP_VERB::PUT => {
                acc.content.push_str("\n\trouter.put('");
                acc.content.push_str(&key);
                acc.content.push_str("', (req: Request<");
                acc.content.push_str(&id);
                acc.content.push_str("Query>, res, next) => {\n\t\t");acc.content.push_str(&id);
                acc.content.push_str(&id);
                acc.content.push_str("\n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.");
                acc.content.push_str("body");
            }
        };
        acc.content.push_str(")\n\t\t\t.then(result => res.json(result), error => res.send())\n\t})");
    }

    fn get_name_from_operation(mut counter: i64, http_verb: &HTTP_VERB,  operation: &&Operation) -> String {
        match &operation.operation_id {
            Some(value) => value.to_owned(),
            None => {
                let body = match http_verb {
                    HTTP_VERB::GET => {"get"}
                    HTTP_VERB::DELETE => {"delete"}
                    HTTP_VERB::POST => {"post"}
                    HTTP_VERB::PUT => {"put"}
                };
                counter = counter + 1;
                format!("{}{}", body, counter)
            }
        }
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
                \n\nimport { Request, RequestMetadata, UseCase } from './genericsTypes'\
                \n\nimport {\
                \n} from './types'\
                \n\nexport function buildRouter(\
                \n\t\t\trouter: Router = express.Router()\
                \n\t\t) {\
                \n\treturn router\
                \n}";
        let ir = serde_json::from_str("{\"openapi\": \"3.0.0\",\"info\": {\"title\": \"test\",\"description\": \"Descr\",\"version\": \"0.0.1\"},\"paths\": {}}").expect("Could not deserialize input");
        assert_eq!(TSHTTPServer::export(&ir), Ok(expected.to_owned()));
    }

    #[test]
    fn export_y_schema() -> Result<(), String> {
        let expected = "import * as express from 'express'\
                \nimport { Router } from 'express'\
                \n\nimport { Request, RequestMetadata, UseCase } from './genericsTypes'\
                \n\nimport {\
                \n\tlistRootsQuery,\
                \n\tlistRootsReturn,\
                \n\tcreateRootQuery,\
                \n\tcreateRootReturn,\
                \n} from './types'\
                \n\nexport function buildRouter(\
                \n\t\t\tlistRoots: UseCase<listRootsQuery, listRootsReturn>,\
                \n\t\t\tcreateRoot: UseCase<createRootQuery, createRootReturn>,\
                \n\t\t\trouter: Router = express.Router()\
                \n\t\t) {\
                \n\trouter.get('/roots', (req: Request<listRootsQuery>, res, next) => {\
                \n\t\tlistRoots\
                \n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.query)\
                \n\t\t\t.then(result => res.json(result), error => res.send())\
                \n\t})\
                \n\trouter.post('/roots', (req: Request<createRootQuery>, res, next) => {\
                \n\t\tcreateRoot\
                \n\t\t\t.execute(req.session, req.headers as RequestMetadata, req.body)\
                \n\t\t\t.then(result => res.json(result), error => res.send())\
                \n\t})\
                \n\treturn router\
                \n}";
        let file = File::open("./data-tests/simple-open-api-3.0.json").expect("Invalid testFile");
        let reader = BufReader::new(file);
        let ir = serde_json::from_reader(reader).expect("Could not deserialize input");
        assert_eq!(TSHTTPServer::export(&ir), Ok(expected.to_owned()));
        Ok(())
    }
}