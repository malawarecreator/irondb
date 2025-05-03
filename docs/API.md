# IronDB API Guide<br>
IronDB is a rust-based key-value storage engine<br>
It has an easily interactable API<br>
In this guide, you will learn to use it<br>

## `irondb::db::dblock::Dblock`
A `Dblock` (datablock) is the smallest unit for storage in IronDB.
For reference, this is the Dblock struct:
```rs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dblock<A, B> {
    pub key: A,
    pub data: B
}

```
You can create a dblock like so: <br>
```rs
let test_block: Dblock<YOUR_KEY_TYPE, YOUR_VALUE_TYPE> = Dblock::new(KEY, VALUE);
```
`std::fmt::Display` is implemented for `Dblock`, so you can easily print it out:<br>
```rs
println!("{:?}", test_block);
```


Now we have data blocks, but how can we use them in a database?

## `irondb::db::database::Db`
A `Db` (database) is a container that can store many instances of `Dblock`.
For reference, this is the Db struct:
```rs
#[derive(Debug,PartialEq, Eq, Clone)]
pub struct Db<A, B> {
    pub datablocks: Box<Vec<Dblock<A, B>>>,
    pub id: &'static str,
}
```
### To create a Db:
```rs
let db: Db<i32,i32_> = Db::new(YOUR_ID (string));
```

For example: 
```rs
let db: Db<i32, i32> = Db::new(YOUR_ID (string));
```

### To add to a database:
```rs
let mut db: Db<&'static str, &'static str> = Db::new("randomid");
db.save(Dblock::new("key", "value"));
```

If you try to save a block that does not fit the TT constraints, you will get an error.

### To get and print a `Dblock` within a `Db`:
```rs
let mut db: Db<&'static str, &'static str> = Db::new("randomid");
db.save(Dblock::new("key", "value"));
db.getbp("key");
```

### To get the `Dblock` object itself:
```rs
let mut db: Db<&'static str, &'static str> = Db::new("randomid");
db.save(Dblock::new("key", "value"));
let block = db.getb("key");
```
### To remove a `Dblock` from a `Db`:
```rs
let mut db: Db<&'static str, &'static str> = Db::new("randomid");
db.save(Dblock::new("key", "value"));
let block = db.remove("key")
```
