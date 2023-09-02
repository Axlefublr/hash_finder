use super::*;

#[test]
fn create_sha256_abc() {
    let result = create_sha256("abc".to_string());
    assert_eq!(
        result,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn multithread_hashing_more_than_hash_length() {
    let result = multithread_hashing(HASH_LENGTH + 1, 1);
    assert!(result.is_err());
}
