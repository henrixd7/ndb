Ndb is simple commandline tool for Rnostr database

Rnostr  
A high-performance and scalable nostr relay written in Rust.  
https://github.com/rnostr/rnostr

```
Usage: ndb [OPTIONS]

Options:
      --db <FILE>      Path to database [default: data/events]
      --count          Just show count of matching records
      --filter <json>  JSON string to filter query results [default: {}]
      --delete         Delete ALL matching records from database
      --dryrun         Skips actual deleting
      --pretty         Shows event in multiple lines and omits some info
  -h, --help           Print help
  -V, --version        Print version
```

```
Filter string:
{
    "ids": [ "b667fd4f...73fc78c", "c43245cb8...3cf811217" ],
    "authors": [ "336ccc3...158b6f8", "5be6446aa...461f6a9b1"],
    "kinds": [ 0, 1, 5 ],
    "until": 1716514740,
    "since": 1715492848,
    "limit": 100
}
```
