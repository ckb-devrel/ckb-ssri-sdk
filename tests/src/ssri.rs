// Note: These tests currently depends on a running SSRI-server. See https://github.com/ckb-devrel/ssri-server

use std::ffi::CString;

use ckb_std::high_level::{decode_hex, encode_hex};
use reqwest::Client;
use serde_json::json;

use crate::utils::{get_ssri_response, method_path, method_path_hex};
use ckb_ssri_sdk::prelude::{decode_u64_vector, encode_u8_32_vector};

#[test]
fn test_method_path() {
    let name = "SSRI.version";
    let method_path = method_path(name);
    println!("Method path: {:?}", method_path);
    let method_path_hex = method_path_hex(name);
    println!("Method path hex: {:?}", method_path_hex);
}

#[tokio::test]
async fn test_get_methods() {
    let url = "http://localhost:9090";

    let get_methods_path_hex = method_path_hex("SSRI.get_methods");
    println!("Get methods path: {}", get_methods_path_hex);
    // Define the JSON payload
    let payload = json!({
        "id": 2,
        "jsonrpc": "2.0",
        "method": "run_script_level_code",
        "params": [
            "0x9e36b25f047c6262a138316394098e879a4128743aabd6ece4803364e5fdf8b3",
            0,
            [get_methods_path_hex, "0x0000000000000000", "0x0a00000000000000"]
        ]
    });

    // Create an HTTP client and send the request
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Request failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    let response_json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    println!("Response JSON: {:?}", response_json);

    let result_string = String::from(response_json["result"].as_str().unwrap())[2..].to_string();
    println!("Result string: {:?}", result_string);

    let result_in_c_string = CString::new(result_string).unwrap();
    let decoded_result = decode_hex(result_in_c_string.as_c_str()).unwrap();
    println!("Decoded result: {:?}", decoded_result);
    println!("Decoded result length: {:?}", decoded_result.len());
    // NOTE: The decoded result is a vector of u64 values as the implementation of SSRI.get_methods has already trimmed the first 4 u8;
    let method_u64_vector = decode_u64_vector(&decoded_result).unwrap();
    println!("Decoded result: {:?}", method_u64_vector);
}

#[tokio::test]
async fn test_version() {
    let url = "http://localhost:9090";

    let version_path_hex = method_path_hex("SSRI.version");
    println!("Version path: {:?}", version_path_hex);

    let payload = json!({
        "id": 2,
        "jsonrpc": "2.0",
        "method": "run_script_level_code",
        "params": [
            "0x9e36b25f047c6262a138316394098e879a4128743aabd6ece4803364e5fdf8b3",
            0,
            [version_path_hex]
        ]
    });

    // Create an HTTP client and send the request
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Request failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    let response_json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    println!("Response JSON: {:?}", response_json);

}

#[tokio::test]
pub async fn test_is_paused() {
    // Note: This is the lock hash of ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqtxe0gs9yvwrsc40znvdc6sg4fehd2mttsngg4t4
    let test_lock_hash_hex = "0xa9c1b6b195ce5b7a3f0bbc07d16e00100db0935798b9f7421cc86fb8218ba299";
    let test_lock_hash_u8_32: [u8;32] = decode_hex(CString::new(&test_lock_hash_hex[2..]).unwrap().as_c_str()).unwrap().try_into().unwrap();
    let test_lock_hash_array_vector = encode_u8_32_vector(vec![test_lock_hash_u8_32]);
    let test_lock_hash_array_encoded_hex = format!("0x{}", encode_hex(&test_lock_hash_array_vector).into_string().unwrap());
    println!("Test lock hash: {:?}", test_lock_hash_u8_32);
    println!("Test lock hash encoded: {:?}", test_lock_hash_array_vector);
    println!("Test lock hash encoded hex: {:?}", test_lock_hash_array_encoded_hex);

    let is_paused_path_hex = method_path_hex("UDT.is_paused");
    println!("is_paused path hex: {:?}", is_paused_path_hex);

    // Define the JSON payload
    let payload = json!({
        "id": 2,
        "jsonrpc": "2.0",
        "method": "run_script_level_code",
        "params": [
            "0xe06146dc630f96b3fb0a6cbaf81350b87aa8f195745bdf21d6d4f6de2f53d0cc",
            0,
            [is_paused_path_hex, test_lock_hash_array_encoded_hex]
        ]
    });

    let response_json: serde_json::Value = get_ssri_response(payload).await;
    println!("Response JSON: {:?}", response_json);
}

#[tokio::test]
pub async fn test_enumerate_paused() {
    let enumerate_paused_path_hex = method_path_hex("UDT.enumerate_paused");
    println!("enumerate_paused path hex: {:?}", enumerate_paused_path_hex);

    let payload = json!({
        "id": 2,
        "jsonrpc": "2.0",
        "method": "run_script_level_code",
        "params": [
            "0xe06146dc630f96b3fb0a6cbaf81350b87aa8f195745bdf21d6d4f6de2f53d0cc",
            0,
            [enumerate_paused_path_hex]
        ]
    });

    let response_json: serde_json::Value = get_ssri_response(payload).await;
    println!("Response JSON: {:?}", response_json);
}
