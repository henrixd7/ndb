
use nostr_db::{ Db, Error, Event, Filter, Stats/*, SortList */ };
//use std::path::Path;
use std::path::PathBuf;

type Result<T, E = Error> = core::result::Result<T, E>;

pub struct CmdArgs {
    pub path: PathBuf,
    pub filter: Filter,
    pub count: bool,
    pub delete: bool,
    pub dryrun: bool,
    pub pretty: bool
}

fn all(db: &Db, filter: Filter) -> Result<(Vec<Event>, Stats)> {
    let reader = db.reader()?;
    let mut iter = db.iter(&reader, &filter)?;
    let mut events = Vec::new();
    while let Some(e) = iter.next() {
        let e = e.unwrap();
        events.push(e);
    }
    Ok((events, iter.stats()))
}

pub fn app(mut args: CmdArgs) {

    args.filter.build_words();

    let db = Db::open(args.path).expect("db open");

    let (events, _stats) = all(&db, args.filter).expect("all");
    let count = events.len();

    if args.count {
        println!("count: {count:?}");
        return;    
    }

    if args.delete {
        if args.dryrun {
            for event in events.iter() {
                println!("Dry run: would delete id {}", hex::encode(event.id()));
            }
        } else {
            for event in events.iter() {
                println!("Deleting id {}", hex::encode(event.id()));
            }
            db.batch_del(events.iter().map(|e| e.id())).unwrap();
        }
        println!("Deleted {count} records")
    } else {
        for e in events.iter() {
            if args.pretty {
                print_event_pretty(&e);
            } else {
                print_event_oneliner(&e);
            }
        }    
    }
}

fn print_event_oneliner(event: &Event) { 
    let content = event.content().replace("\n", " ");
    println!( 
        "kind: {}, id: {}, npub: {}, created_at: {}, content: {}, sig: {}, tags: {:?}", 
            event.kind(), 
            hex::encode(event.id()), 
            hex::encode(event.pubkey()), 
            event.created_at(), 
            content, 
            hex::encode(event.sig()), 
            event.tags() 
    ); 
} 

fn print_event_pretty(event: &Event) { 
    println!( 
        "id         : {}\nnpub       : {}\nkind       : {}\ncreated_at : {}\ncontent    : {}\nsig        : {}", 
            hex::encode(event.id()), 
            hex::encode(event.pubkey()), 
            event.kind(), 
            event.created_at(), 
            event.content().replace("\n", "\n             "), // Indent new lines in content
            hex::encode(event.sig()), 
    ); 
    println!("tags       : {:?}", event.tags());
    println!(""); 
}
