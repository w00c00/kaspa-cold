//! Offline-only key operations. No node, transport, storage, or logging API.
use kaspa_addresses::Prefix;
use kaspa_bip32::{ExtendedPrivateKey, Language, Mnemonic, SecretKey};
use kaspa_wallet_keys::derivation::{
    gen1::{PubkeyDerivationManager, WalletDerivationManager},
    traits::WalletDerivationManagerTrait,
};
use zeroize::Zeroizing;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Network {
    #[default]
    Mainnet,
    Tn10,
}
impl Network {
    pub fn name(self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Tn10 => "Testnet-10 (TN10)",
        }
    }
    fn prefix(self) -> Prefix {
        match self {
            Self::Mainnet => Prefix::Mainnet,
            Self::Tn10 => Prefix::Testnet,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    Random,
    WordCount,
    InvalidMnemonic,
    Derivation,
    Index,
    BackupMismatch,
}
pub type Result<T> = std::result::Result<T, Error>;

/// Intentionally has no Debug, Clone, Serialize, or raw private-key accessor.
pub struct SecretPhrase(Mnemonic);
impl SecretPhrase {
    pub fn phrase(&self) -> &str {
        self.0.phrase()
    }
    pub fn word(&self, i: usize) -> Option<&str> {
        self.phrase().split(' ').nth(i)
    }
    pub fn count(&self) -> usize {
        self.phrase().split(' ').count()
    }
    pub fn confirm(&self, positions: &[usize; 3], answers: &[Zeroizing<String>; 3]) -> Result<()> {
        if positions
            .iter()
            .zip(answers)
            .all(|(i, a)| self.word(*i) == Some(a.trim()))
        {
            Ok(())
        } else {
            Err(Error::BackupMismatch)
        }
    }
    pub fn public_wallet(&self) -> Result<PublicWallet> {
        let seed = self.0.to_seed("");
        let xprv = ExtendedPrivateKey::<SecretKey>::new(seed).map_err(|_| Error::Derivation)?;
        // The SDK returns a zeroizing representation; never print it or persist it.
        let text = xprv.to_string(kaspa_bip32::Prefix::KPRV);
        let manager = WalletDerivationManager::from_master_xprv(&text, false, 0, None)
            .map_err(|_| Error::Derivation)?;
        Ok(PublicWallet { manager })
    }
}

pub trait EntropySource {
    fn fill(&mut self, bytes: &mut [u8]) -> Result<()>;
}
pub struct OsEntropy;
impl EntropySource for OsEntropy {
    fn fill(&mut self, bytes: &mut [u8]) -> Result<()> {
        getrandom::fill(bytes).map_err(|_| Error::Random)
    }
}
pub fn generate(words: usize, random: &mut impl EntropySource) -> Result<SecretPhrase> {
    let len = match words {
        12 => 16,
        24 => 32,
        _ => return Err(Error::WordCount),
    };
    let mut entropy = Zeroizing::new(vec![0; len]);
    random.fill(&mut entropy)?;
    Mnemonic::from_entropy(entropy.to_vec(), Language::English)
        .map(SecretPhrase)
        .map_err(|_| Error::Derivation)
}
pub fn restore(input: &str) -> Result<SecretPhrase> {
    if input.len() > 1024 {
        return Err(Error::InvalidMnemonic);
    }
    let mut normalized = Zeroizing::new(String::with_capacity(input.len()));
    for word in input.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        for c in word.chars() {
            normalized.push(c.to_ascii_lowercase());
        }
    }
    if !matches!(normalized.split(' ').count(), 12 | 24) {
        return Err(Error::WordCount);
    }
    Mnemonic::new(normalized.as_str(), Language::English)
        .map(SecretPhrase)
        .map_err(|_| Error::InvalidMnemonic)
}
/// Select three distinct, unbiased positions. Entropy failure aborts confirmation setup.
pub fn challenge(words: usize, random: &mut impl EntropySource) -> Result<[usize; 3]> {
    if !matches!(words, 12 | 24) {
        return Err(Error::WordCount);
    }
    let limit = 256 - 256 % words;
    let mut positions = [0; 3];
    for n in 0..3 {
        // Bound a broken entropy provider instead of hanging the interface forever.
        let mut found = false;
        for _ in 0..1024 {
            let mut b = [0];
            random.fill(&mut b)?;
            let value = b[0] as usize;
            if value < limit && !positions[..n].contains(&(value % words)) {
                positions[n] = value % words;
                found = true;
                break;
            }
        }
        if !found {
            return Err(Error::Random);
        }
    }
    Ok(positions)
}

/// Only public derivation state survives after backup/recovery.
pub struct PublicWallet {
    manager: WalletDerivationManager,
}
impl PublicWallet {
    pub fn address(&self, network: Network, index: u32) -> Result<String> {
        if index >= 1 << 31 {
            return Err(Error::Index);
        }
        let key = self
            .manager
            .derive_receive_pubkey(index)
            .map_err(|_| Error::Derivation)?;
        PubkeyDerivationManager::create_address(&key, network.prefix(), false)
            .map(|a| a.to_string())
            .map_err(|_| Error::Derivation)
    }
    pub fn receipt(&self, network: Network, index: u32) -> Result<String> {
        let address = self.address(network, index)?;
        Ok(format!(
            "Kaspa Cold 0.1.0 — PUBLIC recovery information / 公开恢复说明\n\nNetwork / 网络: {}\nAddress / 地址: {}\nBIP39: English; 12 or 24 words; NO additional passphrase / 英文，无附加口令\nAccount / 账户: 0\nAddress index / 地址序号: {}\nReceive derivation / 收款路径: m/44'/111111'/0'/0/{}\nAddress type / 地址类型: Schnorr P2PK\n\nThis file contains NO mnemonic, seed or private key.\n此文件不含助记词、种子或私钥，不能单独恢复钱包。\nKeep your handwritten mnemonic offline and separate.\n请将手写助记词离线、分开保管。\nTestnet address prefixes do not distinguish TN10 from other Kaspa testnets.\n测试网地址前缀不能单独区分 TN10 与其他 Kaspa 测试网。\n",
            network.name(),
            address,
            index,
            index
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Public BIP39 zero-entropy vector, NEVER use this mnemonic for funds.
    const VECTOR: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    struct Fixed(u8);
    impl EntropySource for Fixed {
        fn fill(&mut self, b: &mut [u8]) -> Result<()> {
            b.fill(self.0);
            self.0 = self.0.wrapping_add(1);
            Ok(())
        }
    }
    struct Failing;
    impl EntropySource for Failing {
        fn fill(&mut self, _: &mut [u8]) -> Result<()> {
            Err(Error::Random)
        }
    }
    #[test]
    fn bip39_vector_and_restore() {
        let s = generate(12, &mut Fixed(0)).unwrap();
        assert_eq!(s.phrase(), VECTOR);
        let a = s.public_wallet().unwrap();
        let b = restore(VECTOR).unwrap().public_wallet().unwrap();
        for i in [0, 1, 19, 100] {
            assert_eq!(
                a.address(Network::Mainnet, i).unwrap(),
                b.address(Network::Mainnet, i).unwrap()
            );
        }
        assert_ne!(
            a.address(Network::Mainnet, 0).unwrap(),
            a.address(Network::Mainnet, 1).unwrap()
        );
    }
    #[test]
    fn words24() {
        let s = generate(24, &mut Fixed(0)).unwrap();
        assert_eq!(s.count(), 24);
        assert!(restore(s.phrase()).is_ok());
    }
    #[test]
    fn failures_closed() {
        assert!(matches!(generate(24, &mut Failing), Err(Error::Random)));
        assert_eq!(challenge(24, &mut Failing), Err(Error::Random));
        assert!(matches!(generate(15, &mut Fixed(0)), Err(Error::WordCount)));
        assert!(restore(&"abandon ".repeat(12)).is_err());
        assert!(restore("invalid").is_err());
        assert!(restore(&"zebra ".repeat(12)).is_err());
    }
    #[test]
    fn networks_same_key_different_checksum() {
        let w = restore(VECTOR).unwrap().public_wallet().unwrap();
        let m: kaspa_addresses::Address = w
            .address(Network::Mainnet, 0)
            .unwrap()
            .as_str()
            .try_into()
            .unwrap();
        let t: kaspa_addresses::Address = w
            .address(Network::Tn10, 0)
            .unwrap()
            .as_str()
            .try_into()
            .unwrap();
        assert_eq!(m.payload, t.payload);
        assert_ne!(m.prefix, t.prefix);
        assert_eq!(w.address(Network::Mainnet, 1 << 31), Err(Error::Index));
    }
    #[test]
    fn official_sdk_known_address() {
        // Published rusty-kaspa gen1 test vector; no private key fixture.
        let s = restore("hunt bitter praise lift buyer topic crane leopard uniform network inquiry over grain pass match crush marine strike doll relax fortune trumpet sunny silk").unwrap();
        assert_eq!(
            s.public_wallet()
                .unwrap()
                .address(Network::Tn10, 1)
                .unwrap(),
            "kaspatest:qrc2959g0pqda53glnfd238cdnmk24zxzkj8n5x83rkktx4h73dkc4ave6wyg"
        );
    }
    #[test]
    fn backup_and_public_export() {
        let s = restore(VECTOR).unwrap();
        let positions = challenge(12, &mut Fixed(0)).unwrap();
        assert_eq!(positions, [0, 1, 2]);
        let answers =
            std::array::from_fn(|i| Zeroizing::new(s.word(positions[i]).unwrap().to_owned()));
        assert_eq!(s.confirm(&positions, &answers), Ok(()));
        let bad = std::array::from_fn(|_| Zeroizing::new("wrong".to_owned()));
        assert_eq!(s.confirm(&positions, &bad), Err(Error::BackupMismatch));
        let receipt = s
            .public_wallet()
            .unwrap()
            .receipt(Network::Mainnet, 0)
            .unwrap();
        assert!(!receipt.contains(VECTOR));
        assert!(!receipt.contains("kprv"));
        assert!(!receipt.contains("abandon"));
    }
}
