# Why Typed-api ?

Types are a useful part of documentation.
But types aren't enough. 
In many languages, we can't describe a type float type between 1. and 2.59 easly.
We think use schema description to validate user input is vital for an api
and that writing types is doing at least twice the work (schema+ server type and  maybe client's types).

So this program aims:
 - to generate type, server api and client api from validations schema
 - don't let the schemas be constrained by the chosen protocol

# Roadmap

 - [ ] generate GRPC / Typescript client and server
 - [ ] add cli
 - [ ] generate HTTP
 - [ ] add GUI 

# FAQ

##### Why I can use a `any` type ?
`any` is an absence of typing.
We prefer to use an enumeration of possible types.
Moreover, we find this more explicit and flexible.

