#[cfg(not(debug_assertions))]
use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, instrument, warn};

pub const GUI_CONFIG_FILENAME: &str = "gui_config.toml";
pub const GUI_STATE_FILENAME: &str = "gui_state.toml";
pub const GUI_TELEMETRY_FILENAME: &str = "gui";
const FALLBACK_DIR: &str = ".tivra";

/// Maximum allowed size for configuration files (1MB).
const MAX_CONFIG_SIZE: u64 = 1024 * 1024;

/// Errors that can occur during configuration loading or saving.
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    Serialize(toml::ser::Error),
    FileTooLarge,
}

// ==================================================================================
// SYNCHRONOUS OPERATIONS
// ==================================================================================

/// Robustly loads a configuration or creates a default one if missing/corrupt.
///
/// This function never fails or panics. It logs errors via tracing and returns a default instance.
#[instrument(skip(dir, filename), fields(dir = ?dir.as_ref(), filename = %filename))]
pub fn setup_config<T, P>(dir: P, filename: &str) -> T
where
    T: Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let path = dir.as_ref().join(filename);

    if path.exists() {
        match load_config(&path) {
            Ok(config) => return config,
            Err(e) => {
                warn!(path = ?path, error = ?e, "Failed to load existing config. Falling back to default.");
            }
        }
    } else {
        info!(path = ?path, "Config file not found. Initializing default configuration.");
    }

    // Reachable if file doesn't exist OR if load failed.
    let config = T::default();

    if let Err(e) = save_config(&path, &config) {
        error!(path = ?path, error = ?e, "Failed to persist default configuration to disk.");
    }

    config
}

/// Loads and deserializes a configuration file from the specified path.
///
/// Returns an error if the file exceeds `MAX_CONFIG_SIZE` or if parsing fails.
#[instrument(skip(path), fields(path = ?path.as_ref()))]
pub fn load_config<T, P>(path: P) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let metadata = fs::metadata(path).map_err(|e| {
        error!(error = ?e, "Failed to read file metadata");
        ConfigError::Io(e)
    })?;

    if metadata.len() > MAX_CONFIG_SIZE {
        error!(
            size = metadata.len(),
            limit = MAX_CONFIG_SIZE,
            "Config file exceeds maximum allowed size"
        );
        return Err(ConfigError::FileTooLarge);
    }

    let content = fs::read_to_string(path).map_err(|e| {
        error!(error = ?e, "Failed to read config file");
        ConfigError::Io(e)
    })?;

    let config = toml::from_str(&content).map_err(|e| {
        error!(error = ?e, "Failed to parse TOML content");
        ConfigError::Parse(e)
    })?;

    debug!("Configuration loaded successfully");
    Ok(config)
}

/// Serializes and saves a configuration to the specified path.
///
/// Automatically creates parent directories if they do not exist.
#[instrument(skip(config, path), fields(path = ?path.as_ref()))]
pub fn save_config<T, P>(path: P, config: &T) -> Result<(), ConfigError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let content = toml::to_string_pretty(config).map_err(|e| {
        error!(error = ?e, "Failed to serialize config");
        ConfigError::Serialize(e)
    })?;

    // Ensure the parent directory exists before writing
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            debug!(parent = ?parent, "Creating parent directory");
            fs::create_dir_all(parent).map_err(ConfigError::Io)?;
        }
    }

    fs::write(path, content).map_err(|e| {
        error!(error = ?e, "Failed to write config file");
        ConfigError::Io(e)
    })?;

    debug!("Configuration saved successfully");
    Ok(())
}

// ==================================================================================
// ASYNCHRONOUS OPERATIONS
// ==================================================================================

/// Robustly loads a configuration or creates a default one if missing/corrupt (Async version).
///
/// This function never fails or panics. It logs errors via tracing and returns a default instance.
#[instrument(skip(dir, filename), fields(dir = ?dir.as_ref(), filename = %filename))]
pub async fn setup_config_async<T, P>(dir: P, filename: &str) -> T
where
    T: Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let path = dir.as_ref().join(filename);

    if path.exists() {
        match load_config_async(&path).await {
            Ok(config) => return config,
            Err(e) => {
                warn!(path = ?path, error = ?e, "Failed to load existing config. Falling back to default.");
            }
        }
    } else {
        info!(path = ?path, "Config file not found. Initializing default configuration.");
    }

    let config = T::default();
    if let Err(e) = save_config_async(&path, &config).await {
        error!(path = ?path, error = ?e, "Failed to persist default configuration to disk.");
    }
    config
}

/// Loads and deserializes a configuration file from the specified path (Async version).
#[instrument(skip(path), fields(path = ?path.as_ref()))]
pub async fn load_config_async<T, P>(path: P) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let metadata = tokio::fs::metadata(path).await.map_err(|e| {
        error!(error = ?e, "Failed to read file metadata");
        ConfigError::Io(e)
    })?;

    if metadata.len() > MAX_CONFIG_SIZE {
        error!(
            size = metadata.len(),
            limit = MAX_CONFIG_SIZE,
            "Config file exceeds maximum allowed size"
        );
        return Err(ConfigError::FileTooLarge);
    }

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        error!(error = ?e, "Failed to read config file");
        ConfigError::Io(e)
    })?;

    let config = toml::from_str(&content).map_err(|e| {
        error!(error = ?e, "Failed to parse TOML content");
        ConfigError::Parse(e)
    })?;

    debug!("Configuration loaded successfully");
    Ok(config)
}

/// Serializes and saves a configuration to the specified path (Async version).
///
/// Automatically creates parent directories if they do not exist.
#[instrument(skip(config, path), fields(path = ?path.as_ref()))]
pub async fn save_config_async<T, P>(path: P, config: &T) -> Result<(), ConfigError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let content = toml::to_string_pretty(config).map_err(|e| {
        error!(error = ?e, "Failed to serialize config");
        ConfigError::Serialize(e)
    })?;

    // Ensure the parent directory exists before writing
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            debug!(parent = ?parent, "Creating parent directory");
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(ConfigError::Io)?;
        }
    }

    tokio::fs::write(path, content).await.map_err(|e| {
        error!(error = ?e, "Failed to write config file");
        ConfigError::Io(e)
    })?;

    debug!("Configuration saved successfully");
    Ok(())
}

// ==================================================================================
// DIRECTORY HELPERS
// ==================================================================================

/// Holds the standard paths used by the application.
#[derive(Debug, Clone)]
pub struct AppDirs {
    pub config: PathBuf,
    pub data: PathBuf,
    pub state: PathBuf,
    pub runtime: PathBuf,
    pub logs: PathBuf,
}

impl AppDirs {
    pub fn gui_config_file(&self) -> PathBuf {
        self.config.join(GUI_CONFIG_FILENAME)
    }

    pub fn gui_state_file(&self) -> PathBuf {
        self.state.join(GUI_STATE_FILENAME)
    }
}

/// Resolves standard application paths and ensures the directories exist on the filesystem.
#[instrument]
pub fn get_and_setup_paths() -> Result<AppDirs, String> {
    info!("Resolving application paths");
    let paths = get_raw_paths();

    // Create essential directories immediately
    let dirs_to_create = [&paths.config, &paths.data, &paths.state];

    for dir in dirs_to_create {
        fs::create_dir_all(dir).map_err(|e| {
            let msg = format!("Could not create dir {:?}: {}", dir, e);
            error!(?dir, error = ?e, "Path setup failed");
            msg
        })?;
    }

    #[cfg(debug_assertions)]
    fs::create_dir_all(&paths.runtime).map_err(|e| {
        let msg = format!("Could not create runtime dir: {}", e);
        error!(?paths.runtime, error = ?e, "Path setup failed");
        msg
    })?;

    debug!(?paths, "Application paths setup complete");
    Ok(paths)
}

/// Determines the raw paths based on the build profile (Debug vs Release).
fn get_raw_paths() -> AppDirs {
    #[cfg(debug_assertions)]
    {
        // In debug mode, store everything in a local fallback dir
        let root = std::env::current_dir().unwrap().join(FALLBACK_DIR);
        AppDirs {
            config: root.join("config"),
            data: root.join("data"),
            state: root.join("state"),
            runtime: root.join("runtime"),
            logs: root.join("state").join("logs"),
        }
    }

    #[cfg(not(debug_assertions))]
    {
        // In release mode, use standard OS directories (XDG, AppData, etc.)
        use crate::APPNAME_LOWERCASE;
        let proj = ProjectDirs::from("", "", APPNAME_LOWERCASE)
            .expect("Could not determine home directory");
        AppDirs {
            config: proj.config_dir().to_path_buf(),
            data: proj.data_dir().to_path_buf(),
            state: proj.state_dir().unwrap_or(proj.data_dir()).to_path_buf(),
            runtime: proj.runtime_dir().unwrap_or(proj.cache_dir()).to_path_buf(),
            logs: proj
                .state_dir()
                .unwrap_or(proj.data_dir())
                .to_path_buf()
                .join("logs"),
        }
    }
}

pub const SILENT_CRATES_GUI: &[&str] = &[
    "i18n_embed",
    "async_io",
    "zbus",
    "wgpu_core",
    "calloop",
    "polling",
    "cosmic_text",
    "iced_wgpu",
    "iced_graphics",
    "sctk",
    "iced_winit",
    "naga",
    "wgpu_hal",
];

// ==================================================================================
// TESTS
// ==================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                name: "default".into(),
                value: 0,
            }
        }
    }

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    }

    // --- Synchronous Tests ---

    #[test]
    fn test_save_and_load_roundtrip() {
        init_tracing();
        let file = NamedTempFile::new().unwrap();
        let config = TestConfig {
            name: "test".into(),
            value: 99,
        };
        save_config(file.path(), &config).unwrap();
        let loaded: TestConfig = load_config(file.path()).unwrap();
        assert_eq!(config, loaded);
    }

    #[test]
    fn test_file_too_large() {
        init_tracing();
        let mut file = NamedTempFile::new().unwrap();
        let big_data = vec![b'a'; (MAX_CONFIG_SIZE + 10) as usize];
        file.write_all(&big_data).unwrap();
        assert!(matches!(
            load_config::<TestConfig, _>(file.path()),
            Err(ConfigError::FileTooLarge)
        ));
    }

    #[test]
    fn test_setup_config_flow() {
        init_tracing();
        let dir = tempfile::tempdir().unwrap();
        let filename = "app.toml";

        // 1. New file creation
        let cfg1: TestConfig = setup_config(dir.path(), filename);
        assert_eq!(cfg1, TestConfig::default());
        assert!(dir.path().join(filename).exists());

        // 2. Load existing
        let modified = TestConfig {
            name: "mod".into(),
            value: 1,
        };
        save_config(dir.path().join(filename), &modified).unwrap();
        let cfg2: TestConfig = setup_config(dir.path(), filename);
        assert_eq!(cfg2, modified);

        // 3. Corruption fallback
        fs::write(dir.path().join(filename), "INVALID TOML").unwrap();
        let cfg3: TestConfig = setup_config(dir.path(), filename);
        assert_eq!(cfg3, TestConfig::default());
    }

    // --- Asynchronous Tests ---

    #[tokio::test]
    async fn test_save_and_load_async_roundtrip() {
        init_tracing();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("async_config.toml");

        let config = TestConfig {
            name: "async_test".into(),
            value: 123,
        };

        save_config_async(&path, &config).await.unwrap();
        let loaded: TestConfig = load_config_async(&path).await.unwrap();
        assert_eq!(config, loaded);
    }

    #[tokio::test]
    async fn test_async_file_too_large() {
        init_tracing();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("large.toml");

        let big_data = vec![b'a'; (MAX_CONFIG_SIZE + 10) as usize];

        // Scope limits lifetime of file handle, ensuring it closes before load attempt
        {
            let mut file = tokio::fs::File::create(&path).await.unwrap();
            file.write_all(&big_data).await.unwrap();
            file.flush().await.unwrap();
        }

        assert!(matches!(
            load_config_async::<TestConfig, _>(&path).await,
            Err(ConfigError::FileTooLarge)
        ));
    }

    #[tokio::test]
    async fn test_setup_config_async_flow() {
        init_tracing();
        let dir = tempfile::tempdir().unwrap();
        let filename = "app_async.toml";
        let path = dir.path().join(filename);

        // 1. New file creation
        let cfg1: TestConfig = setup_config_async(dir.path(), filename).await;
        assert_eq!(cfg1, TestConfig::default());
        assert!(path.exists());

        // 2. Load existing
        let modified = TestConfig {
            name: "async_mod".into(),
            value: 55,
        };
        save_config_async(&path, &modified).await.unwrap();
        let cfg2: TestConfig = setup_config_async(dir.path(), filename).await;
        assert_eq!(cfg2, modified);

        // 3. Corruption fallback
        tokio::fs::write(&path, "INVALID TOML").await.unwrap();
        let cfg3: TestConfig = setup_config_async(dir.path(), filename).await;
        assert_eq!(cfg3, TestConfig::default());
    }

    #[test]
    fn test_get_and_setup_paths() {
        init_tracing();
        let paths = get_and_setup_paths().expect("Path setup failed");
        assert!(paths.config.exists());
        #[cfg(debug_assertions)]
        let _ = fs::remove_dir_all(paths.config.parent().unwrap());
    }
}
