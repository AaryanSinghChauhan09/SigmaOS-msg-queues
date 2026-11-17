// Integration test for SigmaOS-msg-queues
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use msg_queues as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
