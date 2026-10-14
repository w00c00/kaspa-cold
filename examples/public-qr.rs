//! Public BIP39 test vector only. NEVER fund its addresses.
fn main() {
    let phrase = kaspa_cold::core::restore("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
    let public = phrase.public_wallet().unwrap();
    let address = public
        .address(kaspa_cold::core::Network::Mainnet, 0)
        .unwrap();
    let qr = kaspa_cold::qr::address_qr(&address).unwrap();
    std::fs::write(
        "target/test-public-qr.png",
        kaspa_cold::qr::png_bytes(&qr).unwrap(),
    )
    .unwrap();
    std::fs::write("target/test-public-address.txt", address).unwrap();
}
