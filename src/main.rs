// use serde_json::{Number, Value};
// use std::fs;
// fn main() {
//     let mut sales_and_products = {
//         let file_content = fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
//         serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
//     };
//     println!("{:?}", serde_json::to_string_pretty(&sales_and_products).expect("LogRocket: error parsing to JSON"));

//     if let Value::Number(quantity) = &sales_and_products["sales"][1]["quantity"] {
//         sales_and_products["sales"][1]["quantity"] =
//             Value::Number(Number::from_f64(quantity.as_f64().unwrap() + 3.5).unwrap());
//     }
//     fs::write(
//         "./data/sales.json",
//         serde_json::to_string_pretty(&sales_and_products).expect("LogRocket: error parsing to JSON"),
//     )
//     .expect("LogRocket: error writing to file");
// }
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>
}
#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String
}
#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: u64,
    quantity: f32,
    unit: String
}
fn main() {}