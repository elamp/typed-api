use crate::ir::{IntermediateRepresentation, Function, Schema, Interface};
use std::ops::Deref;

pub struct TypescriptGRPCClient{}

impl TypescriptGRPCClient {
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

    //fn generate_service_client(ir: &IntermediateRepresentation) -> Result<String, &'static str> {
    //    ir.interfaces.iter().fold(Ok("".to_string()), |acc, interface| {
    //        acc
    //            .and_then(|mut output| {
    //                output.push_str("class ");
    //                TypescriptGRPC::format_class_name(&interface.name)
    //                    .map(|converted| {
    //                        output.push_str(converted.as_str());
    //                        output.push_str(" {\n\tprotected untypedClient: any\n\n");
    //                        output.push_str(" {\tconstructor(url: string, credential:any = grpc.credentials.createInsecure()) {\n");
    //                        output.push_str(" {\t\tthis.untypedClient= new untypedUserClientService(this.serviceEndpoint, credential)\n");
    //                        output.push_str(" {\t}\n");
    //                        output
    //                    })
    //            })
    //            .and_then(|mut output| {
    //                TypescriptGRPC::generate_functions(&interface.functions)
    //                    .map(|converted| {
    //                        output.push_str(converted.as_str());
    //                        output
    //                    })
    //            })
    //            .map(|mut output| {
    //                output.push_str("}\n\n");
    //                output
    //            })
    //    })
    //}
    //
    //fn generate_functions(functions: &Vec<Function>) -> Result<String, &'static str> {
    //    functions.iter().fold(Ok("".to_string()), |acc, function| {
    //        acc.and_then(|mut output| {
    //            TypescriptGRPC::generate_function(function)
    //                .map(|converted| {
    //                    output.push_str(converted.as_str());
    //                    output
    //                })
    //        })
    //    })
    //}
    //
    //fn generate_function(function: &Function) -> Result<String, &'static str> {
    //    let mut output = "\n\n\t".to_string();
    //    output.push_str(function.name.as_str());
    //    output.push_str("(");
    //    output.push_str("metada?: grpc.Metadata){\n\t\tconst encodedData = encode");
    //    output.push_str(function.name.as_str());
    //    output.push_str("(");
    //    output.push_str(")\n\t\treturn this.untypedClient.");
    //    output.push_str(function.name.as_str());
    //    output.push_str("(encodedData, metadata).then(res => decode");
    //    output.push_str(function.name.as_str());
    //    output.push_str("(res))\n}");
    //    Ok(output)
    //}
    //
    //fn generate_function_args(withType: bool) -> Result<String, &'static str>{
    //    //function.args.map(TypescriptGRPC::generate_function_arg);
    //    Err("Not impl")
    //}
    //
    //fn generate_function_arg(arg: Schema, withType: bool)-> Result<String, &'static str>{
    //    match arg {
    //        Schema::RefSchema { name } => {
    //            TypescriptGRPC::format_interface_name(name)
    //        }
    //        Schema::ObjectSchema { name, fields, unknown: _ } => {
    //            match name {
    //                Some(value) => { TypescriptGRPC::format_interface(value, fields) }
    //                None => { TypescriptGRPC::format_object(fields) }
    //            }
    //        }
    //        Schema::StringSchema { .. } => {
    //            let mut output = "".to_owned();
    //            output.push_str(" string");
    //            Ok(output)
    //        }
    //        Schema::IntSchema { .. } => {
    //            let mut output = "".to_owned();
    //            output.push_str(" number");
    //            Ok(output)
    //        }
    //        Schema::FloatSchema { .. } => {
    //            let mut output = "".to_owned();
    //            output.push_str(" number");
    //            Ok(output)
    //        }
    //        Schema::BooleanSchema { .. } => {
    //            let mut output = "".to_owned();
    //            output.push_str(" boolean");
    //            Ok(output)
    //        }
    //        Schema::ArraySchema { item } => {
    //            TypescriptGRPC::format_schema(item.deref()).map(|mut converted| {
    //                converted.push_str("[]");
    //                converted
    //            })
    //        }
    //        Schema::OneOf { values } => { TypescriptGRPC::format_one_of(values) }
    //    };
    //    if withType {
    //            Err("Not impl")
    //    } else {
    //        Ok(format!("{}"))
    //    };
    //    Err("Not impl")
    //}
    //
    //fn add_client_type(interface: Interface) -> Result<String, &'static str> {
    //    TypescriptGRPC::format_var_name(&interface.name)
    //        .and_then(|instanceName| {
    //            TypescriptGRPC::format_class_name(&interface.name)
    //                .map(|typeName| {
    //                    let mut output = "\t".to_string();
    //                    output.push_str(instanceName.as_str());
    //                    output.push_str(": ");
    //                    output.push_str(typeName.as_str());
    //                    output
    //                })
    //        })
    //}
    //
    //fn format_var_name(name: &String) -> Result<String, &'static str> { Err("format_var_name not implemented")}
    //
    //fn format_class_name(name: &String) -> Result<String, &'static str> {Err("format_class_name not implemented")}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Schema, FieldRestriction};
}