use anyhow::Result;
use serde_json::json;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod utils;
use utils::get_prime_number_array;
use utils::is_param_numeric;

#[http_component]
fn handle_spin_rust(req: Request) -> Result<Response> {
    let params = req.uri().query().unwrap_or("");
    let key = "n";
    let result = is_param_numeric(params, key).unwrap();

    let list: Vec<usize> = match result {
        None => vec![],
        Some(number) => get_prime_number_array(number, 2, vec![]),
    };

    let json = json!({
        "input": result,
        "result": list,
        "message": "☕",
    });

    let response = format!("{}", json);

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(response.into()))?)
}
