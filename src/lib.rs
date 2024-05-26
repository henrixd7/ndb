
use nostr_db::{ Db, Error, Event, Filter, Stats/*, SortList */ };
use std::path::Path;

type Result<T, E = Error> = core::result::Result<T, E>;

fn all(db: &Db, filter: &Filter) -> Result<(Vec<Event>, Stats)> {
    let reader = db.reader()?;
    let mut iter = db.iter(&reader, &filter)?;
    let mut events = Vec::new();
    while let Some(e) = iter.next() {
        let e = e.unwrap();
        events.push(e);
    }
    Ok((events, iter.stats()))
}

fn count(db: &Db, filter: &Filter) -> Result<(u64, Stats)> {
    let reader = db.reader()?;
    let iter = db.iter::<String, _>(&reader, filter)?;
    iter.size()
}

pub fn list(path: &Path, mut filter: Filter) {
    filter.build_words();

    let db = Db::open(path).expect("db open");

    let count = count(&db, &filter);
    println!("count: {count:?}");

    let el = all(&db, &filter).expect("all");
    let _count2 = el.0.len();

    for e in el.0.iter() {
        println!("{e:?}");
        println!("---------------------------------------------------");
    }    
}