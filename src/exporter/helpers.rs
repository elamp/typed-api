pub struct Helpers {}

impl Helpers {
    pub fn format_interface_name(name: &String) -> Result<String, &'static str> {
        return Ok(format!("I{}", name));
    }

    //TODO: search if isn't better to force camelcase
    pub fn format_variable_name(name: &String) -> Result<String, &'static str> {
        return Ok(name.clone());
    }
}