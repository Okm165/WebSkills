use crate::get_client;

#[test]
fn test_connect_to_redis() {
    let _cache = get_client().unwrap();
}
