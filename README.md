## Ndb: Safe and Efficient Content Removal for Rnostr Relays

**Ndb** (Nostr Database) is a command-line tool designed to empower Rnostr relay operators with the ability to safely remove unwanted content from their databases. This tool prioritizes responsible content management while respecting the decentralized nature of Nostr.

**Key Features:**

* **Effortless Removal:** Bypass request limits for efficient content deletion.
* **Granular Filtering:** Filter events by specific criteria like ID, author, event type (kind), date range, and limit the number of results.
* **Safeguarding with Dry-Run:** Preview deletions before committing changes with the dry-run mode.

**Installation & Usage**

You need Rust tools to compile
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

You need to clone the repo
> git clone https://github.com/henrixd7/ndb.git

Compile and install
> cd ndb\  
> cargo install --locked --path .  


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
Filter JSON string example:
{
    "ids": [ "b667fd4f...73fc78c", "c43245cb8...3cf811217" ],
    "authors": [ "336ccc3...158b6f8", "5be6446aa...461f6a9b1" ],
    "kinds": [ 0, 1, 5 ],
    "until": 1716514740,
    "since": 1715492848,
    "limit": 100
}
```

**Rnostr**\
A high-performance and scalable nostr relay written in Rust.  
https://github.com/rnostr/rnostr
