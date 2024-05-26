
use::ndb::list;
use nostr_db::Filter;
use std::path::PathBuf;
//use std::path::Path;
use serde_json;
use clap::{arg, command, value_parser};


fn main() {

    let matches = command!()
    .arg(
        arg!(
            -d --db <FILE> "Path to database"
        )
        .default_value("data/events")
        // We don't have syntax yet for optional options, so manually calling `required`
        .required(false)
        .value_parser(value_parser!(PathBuf)),
    )
    .arg(
        arg!(
            -f --filter <json> "JSON string to filter query results"
        )
        .default_value("{}")
        //.value_parser(value_parser!(PathBuf)),
    )
    .get_matches();

    let path = matches.get_one::<PathBuf>("db").unwrap();
    let filter_json = matches.get_one::<String>("filter").unwrap();

    let filter: Filter = serde_json::from_str(&filter_json.as_str()).expect("json");

    list(path, filter);
}
