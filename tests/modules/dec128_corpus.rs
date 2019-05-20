extern crate serde_derive;
use self::serde_derive::Deserialize;

use bson::{decode_document, Bson};
use bson::decimal128::Decimal128;
use std::io::{Cursor, BufReader};
use std::env;
use std::fs::{self, File};
use std::path::Path;


/* General TODO:
 *     1. open json test file
 *     2. read file into json, and for each test:
 *          a. read the "canonical_extjson" field as JSON into extjson_doc
 *          b. find "$decimal128" field within extjson_doc (no matter how nested) and return value
 *              (which is always going to be a string) as extjson_str
 *          c. decode document from "canonical_bson" field as decoded (lines 352-353)
 *          c. find the to_string method of the Decimal128 type, call it on decoded and store
 *             result as decoded_str
 *          d. compare decoded_str and extjson_str: if the same, then test passes
 */
#[derive(Deserialize, Debug)]
struct Test {
    description: String,
    canonical_bson: String,
    canonical_extjson: String,
    lossy: Option<bool>
}
#[derive(Deserialize, Debug)]
struct TestFile {
    description: String,
    bson_type: String,
    test_key: String,
    valid: Vec<Test>
}

#[derive(Deserialize, Debug)]
struct D128 {
    #[serde(rename="$numberDecimal")]
    decimal128: String
}

#[derive(Deserialize, Debug)]
struct Decimal128ExtJSON {
    d: D128
}

fn run_test(description: &str, canonical_bson: &str, canonical_extjson: &str) {
    // Extract string value from extended JSON
    let d128: Decimal128ExtJSON = serde_json::from_str(canonical_extjson).unwrap();
    let extjson_str = d128.d.decimal128;

    // Decode and build document
    let bson_bytes = hex::decode(canonical_bson).unwrap();
    let decoded = decode_document(&mut Cursor::new(bson_bytes)).unwrap();

    let decoded_str = decoded.get("d").unwrap();
    match decoded_str {
        Bson::Decimal128(val) => {
            if extjson_str != val.to_string() {
                println!("\tFAIL: expected '{}' but got '{}' for test '{}'", extjson_str, val.to_string(), description);
            } else {
                println!("\tPASS: expected '{}' for test '{}'", val.to_string(), description);
            }
        }
        _ => unimplemented!(),
    }
}

#[test]
fn test_encode_decode_decimal128_corpus() { // TODO: get rid of all the unwraps()

    // These three values are eventually going to come from the test JSON
    let canonical_bson = "180000001364000000000000000000000000000000007C00";
    let canonical_extjson = "{\"d\" : {\"$numberDecimal\" : \"NaN\"}}";

    let path = env::current_dir().unwrap().join(Path::new("tests/modules/corpus")); // TODO: need to join everything?

    let test_files = fs::read_dir(path).unwrap();
    for fp in test_files {
        let file_path = fp.unwrap().path();
        println!("TEST_FILE: {}", file_path.display());
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let tests: TestFile = serde_json::from_reader(reader).unwrap();
        for test in tests.valid {
            run_test(&test.description, &test.canonical_bson, &test.canonical_extjson);
        }

    }

}


