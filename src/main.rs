
use ndb::app;
use ndb::CmdArgs;
use nostr_db::Filter;
use std::path::PathBuf;
use serde_json;
use clap::{arg, command, value_parser};

fn main() {

    let matches = command!()
    .arg(
        arg!(
            --db <FILE> "Path to database"
        )
        .default_value("data/events")
        .required(false)
        .value_parser(value_parser!(PathBuf)),
    )
    .arg(
        arg!(
            --count "Just show count of matching records"
        )
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
        arg!(
            --filter <json> "JSON string to filter query results"
        )
        .default_value("{}")
    )
    .arg(
        arg!(
            --delete "Delete ALL matching records from database"
        )
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
        arg!(
            --dryrun "Skips actual deleting"
        )
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
        arg!(
            --pretty "Shows event in multiple lines and omits some info"
        )
        .action(clap::ArgAction::SetTrue)
    )

    .get_matches();

    let filter_json = matches.get_one::<String>("filter").unwrap();
    let filter: Filter = serde_json::from_str(&filter_json.as_str()).expect("json");    

    let args = CmdArgs{
        path: matches.get_one::<PathBuf>("db").unwrap().clone(),
        filter: filter.clone(),
        count: matches.get_one::<bool>("count").unwrap().clone(),
        delete: matches.get_one::<bool>("delete").unwrap().clone(),
        dryrun: matches.get_one::<bool>("dryrun").unwrap().clone(),
        pretty: matches.get_one::<bool>("pretty").unwrap().clone()
    };

    app(args);
}