
use ndb::app;
use ndb::CmdArgs;
use nostr_db::Filter;
use std::path::PathBuf;
use nostr_db::SortList;
use clap::{
    command,
    value_parser,
    Arg,
    ArgAction,
    ArgMatches,
    parser::ValueSource
};

fn get_args() -> ArgMatches {
    command!()
    .arg(
        Arg::new("db")
            .long("db")
            .help("Path to database")
            .default_value("data/events")
            .required(false)
            .value_parser(value_parser!(PathBuf)),
    )
    .arg(
        Arg::new("count")
            .long("count")
            .help("Just show count of matching records")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("filter")
            .long("filter")
            .help("JSON string to filter query results")
            .default_value("{}")
            .conflicts_with_all(&["ids", "authors", "kinds", "since", "until", "limit", "search", "tags", "desc"]),
    )
    .arg(
        Arg::new("delete")
            .long("delete")
            .help("Delete ALL matching records from database")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("dryrun")
            .long("dryrun")
            .help("Skips actual deleting")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("pretty")
            .long("pretty")
            .help("Shows event in multiple lines and omits some info")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("ids")
            .short('i')
            .long("ids")
            .help("Comma-separated list of event ids")
            //.takes_value(true)
            .use_value_delimiter(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("authors")
            .short('a')
            .long("authors")
            .help("Comma-separated list of authors")
            //.takes_value(true)
            .use_value_delimiter(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("kinds")
            .short('k')
            .long("kinds")
            .help("Comma-separated list of kinds")
            //.takes_value(true)
            .use_value_delimiter(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("since")
            .short('s')
            .long("since")
            .help("Start time for events")
            //.takes_value(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("until")
            .short('u')
            .long("until")
            .help("End time for events")
            //.takes_value(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("limit")
            .short('l')
            .long("limit")
            .help("Limit number of events")
            //.takes_value(true)
            .conflicts_with("filter"),
    )
    .arg(
        Arg::new("search")
            .long("search")
            .help("Search keyword")
            //.takes_value(true)
            .conflicts_with("filter"),
    )
    /*
    .arg(
        Arg::new("tags")
            .long("tags")
            .help("Tags in the format key:value,key:value")
            //.takes_value(true)
            .conflicts_with("filter"),
    )
     */
    .arg(
        Arg::new("desc")
            .short('d')
            .long("desc")
            .help("Query by time descending order")
            .action(ArgAction::SetTrue)
            .conflicts_with("filter"),
    )
    .get_matches()    
}

fn main() {

    let matches = get_args();

    //let filter: Filter = if let Some(filter_str) = matches.get_one::<String>("filter") {
    //    serde_json::from_str(filter_str).unwrap()
    //} else {
    let filter: Filter = if let Some(ValueSource::CommandLine) = matches.value_source("filter") {
        let filter = matches.get_one::<String>("filter").unwrap();
        serde_json::from_str(filter).unwrap()
    } else {
        let mut filter = Filter::default();

        if let Some(ids) = matches.get_many::<String>("ids") {
            filter.ids = SortList::from(ids.map(|id| {
                let mut bytes = [0u8; 32];
                hex::decode_to_slice(id, &mut bytes).unwrap();
                bytes
            }).collect::<Vec<_>>());
        }

        if let Some(authors) = matches.get_many::<String>("authors") {
            filter.authors = SortList::from(authors.map(|author| {
                let mut bytes = [0u8; 32];
                hex::decode_to_slice(author, &mut bytes).unwrap();
                bytes
            }).collect::<Vec<_>>());
        }

        if let Some(kinds) = matches.get_many::<String>("kinds") {
            filter.kinds = SortList::from(kinds.map(|kind| kind.parse::<u16>().unwrap()).collect::<Vec<_>>());
        }

        if let Some(since) = matches.get_one::<String>("since") {
            filter.since = Some(since.parse::<u64>().unwrap());
        }

        if let Some(until) = matches.get_one::<String>("until") {
            filter.until = Some(until.parse::<u64>().unwrap());
        }

        if let Some(limit) = matches.get_one::<String>("limit") {
            filter.limit = Some(limit.parse::<u64>().unwrap());
        }

        if let Some(search) = matches.get_one::<String>("search") {
            filter.search = Some(search.to_string());
        }

        /*
        if let Some(tags) = matches.get_one::<String>("tags") {
            let tag_map = tags.split(',')
                .map(|tag| {
                    let mut kv = tag.splitn(2, ':');
                    let key = kv.next().unwrap().to_string();
                    let value = kv.next().unwrap().to_string();
                    (key, vec![value])
                })
                .collect();
            filter.tags = tag_map;
        }
         */

        filter.desc = matches.get_flag("desc");

        filter
    };

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