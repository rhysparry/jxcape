pub trait JsonValueCommand {
    fn get_json_value(&self) -> serde_json::Value;
}

pub struct JsonPrettyPrinter;

impl JsonFormatter for JsonPrettyPrinter {
    fn format(&self, value: serde_json::Value) -> String {
        serde_json::to_string_pretty(&value).unwrap()
    }
}

pub struct JsonDefaultPrinter;

impl JsonFormatter for JsonDefaultPrinter {
    fn format(&self, value: serde_json::Value) -> String {
        serde_json::to_string(&value).unwrap()
    }
}

pub trait JsonFormatter {
    fn format(&self, value: serde_json::Value) -> String;
}

impl dyn JsonFormatter {
    pub fn print(&self, value: serde_json::Value) {
        println!("{}", self.format(value));
    }
}
