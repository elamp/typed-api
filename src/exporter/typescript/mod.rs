mod http_server;
mod types;

pub use self::http_server::{{TSHTTPServer}};
pub use self::types::{{TSTypes}};

use openapiv3::OpenAPI;

pub struct TypescriptHTTP {}

impl TypescriptHTTP {
    pub fn export_server(ir: &OpenAPI) -> Result<String, String> {
        TSHTTPServer::export(ir)
    }
    pub fn export_types(ir: &OpenAPI) -> Result<String, String> {
        TSTypes::export(ir)
    }
}