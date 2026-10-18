pub fn image_uri_is_valid(uri_bytes: &[u8]) -> bool {
    let Ok(text) = core::str::from_utf8(uri_bytes) else {
        return false;
    };
    if let Some(authority) = text.strip_prefix("https://") {
        return !authority.is_empty() && !authority.bytes().all(|b| b == b' ');
    }
    if let Some(cid) = text.strip_prefix("ipfs://") {
        return cid.len() >= 46;
    }
    false
}
