use crate::ir::{IntermediateRepresentation, Schema, FieldRestriction, Interface, Function};
use std::ops::Deref;

mod helpers;
mod interfaces;
mod protobuf;
mod encoder;
mod clients;

use helpers::Helpers;
use interfaces::TypescriptInterface;
use protobuf::Protobuf;
use encoder::TSProtobufEncoder;
use clients::TypescriptClient;

pub struct TypescriptGRPC {}


impl TypescriptGRPC {
    pub fn export_types(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        TypescriptInterface::export(ir)
    }

    pub fn export_client(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        TypescriptClient::export(ir)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Schema, FieldRestriction};
}