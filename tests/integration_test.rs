// Integration test for SigmaOS-distribution-release-engineering
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use distribution_release_engineering as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
