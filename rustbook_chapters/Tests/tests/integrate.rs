use Tests;

// tests directory is for using public API of your lib
// the entire directory is treated as a 'cargo test only' so only need #[test] header amd a;; files will be run
// NOTE: only crates (lib.rs) can have a tests directory

#[test]
fn integrattion() {
    assert!(true);
}
// unit tests must all pass first before this will test


// BIG NOTE: All files are run, so they are all seen as tests.  This means a file used as a 'helper'
// will appear in output w/ "0 tests run",  using directories with mod.rs can help avoid this, although not a huge issue
// just means more terminal output

