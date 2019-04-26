Ideally I want to reuse to CLI pattern of having a fixtures/ folder of which I can copy to a new directory.

I can't quite figure out how bundling resources works in Rust. It works (in 0.1.4) locally but once downloaded from Cargo it can't locate the fixture files.

Ideas for a solution:
* Create the folders and files based on Rust code
* Download fixtures on first run to a data dir