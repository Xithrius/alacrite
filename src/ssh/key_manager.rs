use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{Result, eyre::eyre};
use ssh_key::{Algorithm, PrivateKey, PublicKey};
use tracing::info;

pub struct KeyManager {
    private_key: PrivateKey,
    public_key: PublicKey,
    /// Map of trusted peer IDs to their public keys
    trusted_keys: HashMap<String, PublicKey>,
    key_dir: PathBuf,
}

impl KeyManager {
    pub fn new(key_dir: &Path) -> Result<Self> {
        let private_key_path = key_dir.join("id_ed25519");
        let public_key_path = key_dir.join("id_ed25519.pub");

        // Try to load existing keys
        if private_key_path.exists() && public_key_path.exists() {
            Self::load_keys(key_dir)
        } else {
            // Generate new keys
            let key_manager = Self::generate_new_keys(key_dir)?;
            key_manager.save_keys()?;
            Ok(key_manager)
        }
    }

    fn generate_new_keys(key_dir: &Path) -> Result<Self> {
        info!("Generating new SSH key pair...");

        // Generate Ed25519 key pair using ssh-key crate
        let private_key = PrivateKey::random(&mut rand::thread_rng(), Algorithm::Ed25519)?;
        let public_key = private_key.public_key().clone();

        Ok(Self {
            private_key,
            public_key,
            trusted_keys: HashMap::new(),
            key_dir: key_dir.to_path_buf(),
        })
    }

    fn load_keys(key_dir: &Path) -> Result<Self> {
        info!("Loading existing SSH keys...");

        let private_key_path = key_dir.join("id_ed25519");
        let public_key_path = key_dir.join("id_ed25519.pub");

        // Check if both key files exist
        if !private_key_path.exists() {
            return Err(eyre!("Private key file not found: {:?}", private_key_path));
        }
        if !public_key_path.exists() {
            return Err(eyre!("Public key file not found: {:?}", public_key_path));
        }

        // Load private key
        let private_key_pem = fs::read_to_string(&private_key_path)
            .map_err(|e| eyre!("Failed to read private key: {}", e))?;
        let private_key = PrivateKey::from_openssh(&private_key_pem)
            .map_err(|e| eyre!("Failed to parse private key: {}", e))?;

        // Load public key
        let public_key_openssh = fs::read_to_string(&public_key_path)
            .map_err(|e| eyre!("Failed to read public key: {}", e))?;
        let public_key = PublicKey::from_openssh(&public_key_openssh)
            .map_err(|e| eyre!("Failed to parse public key: {}", e))?;

        let key_manager = Self {
            private_key,
            public_key,
            trusted_keys: HashMap::new(),
            key_dir: key_dir.to_path_buf(),
        };

        // Verify the loaded keys are valid
        key_manager.verify_keys()?;

        info!("SSH keys loaded successfully from {:?}", key_dir);
        Ok(key_manager)
    }

    pub fn save_keys(&self) -> Result<()> {
        // Create key directory if it doesn't exist
        fs::create_dir_all(&self.key_dir)
            .map_err(|e| eyre!("Failed to create key directory: {}", e))?;

        // Save private key in OpenSSH format
        let private_key_openssh = self
            .private_key
            .to_openssh(ssh_key::LineEnding::LF)
            .map_err(|e| eyre!("Failed to serialize private key: {}", e))?;
        let private_key_path = self.key_dir.join("id_ed25519");
        fs::write(&private_key_path, private_key_openssh).map_err(|e| {
            eyre!(
                "Failed to write private key to {:?}: {}",
                private_key_path,
                e
            )
        })?;

        // Save public key in OpenSSH format
        let public_key_openssh = self
            .public_key
            .to_openssh()
            .map_err(|e| eyre!("Failed to serialize public key: {}", e))?;
        let public_key_path = self.key_dir.join("id_ed25519.pub");
        fs::write(&public_key_path, public_key_openssh)
            .map_err(|e| eyre!("Failed to write public key to {:?}: {}", public_key_path, e))?;

        info!("SSH keys saved successfully to {:?}", self.key_dir);
        Ok(())
    }

    pub fn get_public_key_openssh(&self) -> Result<String> {
        Ok(self.public_key.to_openssh()?)
    }

    #[must_use]
    pub const fn get_private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    #[must_use]
    pub const fn get_public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Add a trusted peer's public key
    pub fn add_trusted_key(&mut self, peer_id: String, public_key: PublicKey) {
        self.trusted_keys.insert(peer_id, public_key);
        info!("Added trusted key for peer");
    }

    /// Get a trusted peer's public key
    #[must_use]
    pub fn get_trusted_key(&self, peer_id: &str) -> Option<&PublicKey> {
        self.trusted_keys.get(peer_id)
    }

    /// Check if a peer is trusted
    #[must_use]
    pub fn is_peer_trusted(&self, peer_id: &str) -> bool {
        self.trusted_keys.contains_key(peer_id)
    }

    /// Get the key directory path
    #[must_use]
    pub const fn key_dir(&self) -> &PathBuf {
        &self.key_dir
    }

    /// Verify that the loaded keys are valid
    pub fn verify_keys(&self) -> Result<()> {
        // Try to serialize to verify keys are valid
        let _ = self.private_key.to_openssh(ssh_key::LineEnding::LF)?;
        let _ = self.public_key.to_openssh()?;

        // Verify the public key matches the private key
        let derived_public = self.private_key.public_key();
        if *derived_public != self.public_key {
            return Err(eyre!("Public key does not match private key"));
        }

        info!("SSH keys verified successfully");
        Ok(())
    }

    /// Save trusted keys to disk
    pub fn save_trusted_keys(&self) -> Result<()> {
        let trusted_keys_path = self.key_dir.join("trusted_keys.json");

        // Convert PublicKey to serializable format
        let trusted_keys_serializable: HashMap<String, String> = self
            .trusted_keys
            .iter()
            .map(|(peer_id, public_key)| {
                (peer_id.clone(), public_key.to_openssh().unwrap_or_default())
            })
            .collect();

        let trusted_keys_json = serde_json::to_string_pretty(&trusted_keys_serializable)
            .map_err(|e| eyre!("Failed to serialize trusted keys: {}", e))?;

        fs::write(&trusted_keys_path, trusted_keys_json)
            .map_err(|e| eyre!("Failed to write trusted keys: {}", e))?;

        info!("Trusted keys saved to {:?}", trusted_keys_path);
        Ok(())
    }

    /// Load trusted keys from disk
    pub fn load_trusted_keys(&mut self) -> Result<()> {
        let trusted_keys_path = self.key_dir.join("trusted_keys.json");

        if trusted_keys_path.exists() {
            let trusted_keys_json = fs::read_to_string(&trusted_keys_path)
                .map_err(|e| eyre!("Failed to read trusted keys: {}", e))?;

            let trusted_keys_serializable: HashMap<String, String> =
                serde_json::from_str(&trusted_keys_json)
                    .map_err(|e| eyre!("Failed to parse trusted keys: {}", e))?;

            // Convert back to PublicKey objects
            self.trusted_keys = trusted_keys_serializable
                .into_iter()
                .filter_map(|(peer_id, key_str)| {
                    PublicKey::from_openssh(&key_str)
                        .map(|public_key| (peer_id, public_key))
                        .ok()
                })
                .collect();

            info!(
                "Loaded {} trusted keys from {:?}",
                self.trusted_keys.len(),
                trusted_keys_path
            );
        }

        Ok(())
    }
}
