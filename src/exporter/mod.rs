mod clients;

use clients::{{TSHTTPClient}};
use openapiv3::{OpenAPI};

pub struct TypescriptHTTP {}

impl TypescriptHTTP {
    pub fn export_client(ir: &OpenAPI) -> Result<String, String> {
        TSHTTPClient::export(ir)
    }
}
