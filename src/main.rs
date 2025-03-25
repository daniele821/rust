use serde_json::Value;

fn main() {
    let json_str = r#"{ "name": "Alice", "age": 30, "is_student": false }"#;

    // Parse JSON string into a serde_json::Value
    let value: Value = serde_json::from_str(json_str).unwrap();

    // Access values dynamically
    println!("{}", value["name"]); // Output: "Alice"
    println!("{}", value["age"]); // Output: 30
    println!("{}", value["is_student"]); // Output: false
}
