pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    //  todo!()

    // Try decoding the hex string to bytes
    let bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    // Make sure it's at least 4 bytes long
    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // Try to turn first 4 bytes into a [u8; 4] array
    let version_bytes: [u8; 4] = bytes[0..4]
        .try_into()
        .map_err(|_| "Could not extract version bytes".to_string())?;

    // Convert little endian bytes to u32
    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
