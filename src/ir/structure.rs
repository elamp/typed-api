pub enum Schema {
    RefSchema { name: String },
    ObjectSchema { name: Option<String>, fields: Vec<(String, FieldRestriction)>, unknown: bool },
    StringSchema { max: Option<i64>, min: Option<i64>, regex: Option<String>, valid_values: Option<Vec<String>> },
    IntSchema { max: Option<i64>, min: Option<i64>, valid_values: Option<Vec<i64>> },
    FloatSchema { max: Option<f64>, min: Option<f64> },
    BooleanSchema { },
    ArraySchema {item: Box<Schema> },
    OneOf { values: Vec<Schema> },
}

pub struct FieldRestriction {
    pub base: Schema,
    pub required: bool
}

pub struct Interface<'a> {
    functions: Vec<Function<'a>>
}

pub struct Function<'a> {
    name: String,
    args: Vec<&'a Schema>,
    result: &'a Schema
}

pub struct IntermediateRepresentation<'a> {
    pub interfaces: Vec<Interface<'a>>,
    pub schemas: Vec<Schema>,
}

impl<'instance> IntermediateRepresentation<'instance> {
    pub fn init() -> IntermediateRepresentation<'instance> {
        IntermediateRepresentation{ interfaces: Vec::new(), schemas: Vec::new()}
    }

    pub fn add_schema(&mut self, schema: Schema) -> () {
        self.schemas.push(schema);
    }
}