use std::cell::RefCell;
use clap::Parser;
use trustfall::{execute_query, FieldValue, TransparentValue};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use serde::Deserialize;


mod adapter;
mod parser;
mod util;


use crate::adapter::Soltrust;

#[derive(Debug, Clone, Deserialize)]
struct InputQuery<'a> {
    query: &'a str,

    args: BTreeMap<Arc<str>, FieldValue>,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    all: bool
}



fn run_query(path: &str, max_results: Option<usize>) {
    let content = util::read_file(path);
    let input_query: InputQuery = ron::from_str(&content).unwrap();

    let soltrust = Soltrust::new();
    let schema = soltrust.schema();

    let adapter = Rc::new(RefCell::new(soltrust));

    let query = input_query.query;
    let arguments = input_query.args;

    for data_item in execute_query(&schema, adapter, query, arguments)
        .expect("not a legal query")
        .take(max_results.unwrap_or(usize::MAX))
    {
        // The default `FieldValue` JSON representation is explicit about its type, so we can get
        // reliable round-trip serialization of types tricky in JSON like integers and floats.
        //
        // The `TransparentValue` type is like `FieldValue` minus the explicit type representation,
        // so it's more like what we'd expect to normally find in JSON.
        let transparent: BTreeMap<_, TransparentValue> =
            data_item.into_iter().map(|(k, v)| (k, v.into())).collect();
        println!("\n{}", serde_json::to_string_pretty(&transparent).unwrap());
    }
}

fn main() {
    let args = Args::parse();

    if args.all {
       run_query("./lints/divide_before_multiply.ron", None);
    }
}
