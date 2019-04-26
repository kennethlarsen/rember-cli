Ideally I want to reuse to CLI pattern of having a fixtures/ folder of which I can copy to a new directory.

I can't quite figure out how bundling resources works in Rust. I'm currently getting it from a sort of hard coded path which I'm not sure will work on other machines.

Ideas for a solution:
* Create the folders and files based on Rust code
* Download fixtures on first run to a data dir