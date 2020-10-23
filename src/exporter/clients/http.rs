use crate::ir::{IntermediateRepresentation, Function, Schema, Interface};
use std::ops::Deref;

pub struct TypescriptHTTPClient{}

impl TypescriptHTTPClient {
    pub fn export(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
        Ok("//todo: Import".to_owned())
        //.and_then(|mut output| {
        //    TypescriptGRPC::generate_service_client(ir)
        //})
        // .and_then(|mut output| {
        //     output.push_str("export default class Client {");
        //     ir.foldInterfaces(Ok(output), |acc, interface| {
        //         acc.and_then(|mut output| {
        //             TypescriptGRPC::add_client_type(schema).map(|converted| {
        //                 output.push_str(converted.as_str());
        //                 output
        //             })
        //         })
        //     })
        // })
        // .and_then(|mut output| {
        //     ir.foldInterfaces(Ok(output), |acc, interface| {
        //         acc.and_then(|mut output| {
        //             TypescriptGRPC::format_schema(schema).map(|converted| {
        //                 output.push_str(converted.as_str());
        //                 output
        //             })
        //         })
        //     })
        // })
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Schema, FieldRestriction};
}