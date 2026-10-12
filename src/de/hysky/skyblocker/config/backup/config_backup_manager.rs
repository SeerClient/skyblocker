//! Ported from de.hysky.skyblocker.config.backup.ConfigBackupManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager;
use crate::entry::skyblocker_mod;
use mcrust::java::time::format::date_time_formatter::DateTimeFormatter;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: private static final Path BACKUP_DIR = SkyblockerMod.CONFIG_DIR.resolve("config_backups");
pub static BACKUP_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("config_backups"));
/// java: public static final DateTimeFormatter FORMATTER = DateTimeFormatter.ofPattern("yyyyMMdd_HHmmss");
pub static FORMATTER: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| DateTimeFormatter::of_pattern("yyyyMMdd_HHmmss"));
/// java: private static final int MAX_BACKUPS = 20;
pub const MAX_BACKUPS: i32 = 20;

/// java: public class ConfigBackupManager
#[derive(Clone)]
pub struct ConfigBackupManager {}

impl ConfigBackupManager {
    /// java: private ConfigBackupManager()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                std::fs::create_dir_all(BACKUP_DIR.get_mut()).expect("IOException");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to create backup directory!"));
            }
        })(), compat::executor);
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> backupConfig());
    }

    /// java: public static void backupConfig()
    pub fn backup_config() {
        ConfigBackupManager::backup_config_1(ConfigType::Main);
    }

    /// java: public static void backupConfig(ConfigType type)
    pub fn backup_config_1(mut r#type: ConfigType) {
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if !(((r#type).fields().path).exists()) {
                return;
            }
            std::fs::create_dir_all(BACKUP_DIR.get_mut()).expect("IOException");
            let mut backups: Vec<std::path::PathBuf> = ConfigBackupManager::list_backups_1(r#type);
            // TODO(fabric2seer): the condition: Files.mismatch
            //   java: !backups.isEmpty() && Files.mismatch(type.path, backups.getFirst()) == -1
            if todo!("fabric2seer: the condition (TODO above)") {
                return;
            }
            let mut backup: std::path::PathBuf = BACKUP_DIR.get_mut().join(&(r#type).get_backup_file_name());
            // TODO(fabric2seer): static field StandardCopyOption.REPLACE_EXISTING
            //   java: Files.copy(type.path, backup, StandardCopyOption.REPLACE_EXISTING);
            ConfigBackupManager::clean_old_backups(r#type);
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker] Failed to backup config!", e);
        //   java: 		}
    }

    /// java: public static List<Path> listBackups() throws IOException
    pub fn list_backups() -> Vec<std::path::PathBuf> {
        return ConfigBackupManager::list_backups_1(ConfigType::Main);
    }

    /// java: public static List<Path> listBackups(ConfigType type) throws IOException
    pub fn list_backups_1(mut r#type: ConfigType) -> Vec<std::path::PathBuf> {
        if !(BACKUP_DIR.get_mut().exists()) {
            return vec![];
        }
        {
            let mut stream = std::fs::read_dir(BACKUP_DIR.get_mut()).expect("IOException").filter_map(|e| e.ok()).map(|e| e.path());
            return compat::sorted_by(stream.clone().filter(|__a0| (r#type).is_of_type(__a0)), |a, b| b.cmp(a)).collect::<Vec<_>>();
        }
    }

    /// java: public static void restoreBackup(Path backup) throws IOException
    pub fn restore_backup(backup: &std::path::PathBuf) {
        if !(ConfigType::Main.is_of_type(backup)) {
            logger().error(&format!("[Skyblocker Config Backup Manager] Attempted to restore a backup that is not of type MAIN: {}", backup.display()));
            return;
        }
        // TODO(fabric2seer): static field StandardCopyOption.REPLACE_EXISTING
        //   java: Files.copy(backup, SkyblockerConfigManager.getConfigPath(), StandardCopyOption.REPLACE_EXISTING);
        SkyblockerConfigManager::reload();
    }

    /// java: private static void cleanOldBackups(ConfigType type) throws IOException
    pub fn clean_old_backups(mut r#type: ConfigType) {
        let mut backups: Vec<std::path::PathBuf> = ConfigBackupManager::list_backups_1(r#type);
        for i in MAX_BACKUPS..(backups.len() as i32) {
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                std::fs::remove_file(&backups[(i) as usize]).is_ok();
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to delete old backup {}", backups[(i) as usize].display()));
            }
        }
    }

}

/// java: public enum ConfigType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfigType {
    Main,
    HudWidgets,
}

impl ConfigType {
    pub const VALUES: [ConfigType; 2] = [ConfigType::Main, ConfigType::HudWidgets];

    /// java: values()
    pub fn values() -> Vec<ConfigType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ConfigType::Main => "MAIN",
            ConfigType::HudWidgets => "HUD_WIDGETS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ConfigType {
        match name {
            "MAIN" => ConfigType::Main,
            "HUD_WIDGETS" => ConfigType::HudWidgets,
            _ => panic!("IllegalArgumentException: No enum constant ConfigType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static ConfigTypeFields {
        static FIELDS: LazyLock<Vec<ConfigTypeFields>> = LazyLock::new(|| vec![
            ConfigTypeFields::new(SkyblockerConfigManager::get_config_path()),
            ConfigTypeFields::new(widget_manager::FILE.get_mut().clone()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public boolean isOfType(Path path)
    pub fn is_of_type(&self, path: &std::path::PathBuf) -> bool {
        let mut name: String = std::path::PathBuf::from(path.file_name().unwrap_or_default()).to_string_lossy().to_string();
        return (name.starts_with(&format!("{}_", self.fields().base_name)) && name.ends_with(&format!(".{}", self.fields().extension)));
    }

    /// java: public String getBackupFileName()
    pub fn get_backup_file_name(&self) -> String {
        // TODO(fabric2seer): static call LocalDateTime.now(..)
        //   java: return baseName + "_" + FORMATTER.format(LocalDateTime.now()) + "." + extension;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each ConfigType constant
pub struct ConfigTypeFields {
    pub path: std::path::PathBuf,
    pub base_name: String,
    pub extension: String,
}

impl ConfigTypeFields {
    fn new(mut path: std::path::PathBuf) -> Self {
        let mut this = Self {
            path: Default::default(),
            base_name: Default::default(),
            extension: Default::default(),
        };
        this.path = path.clone();
        let mut file_name: String = std::path::PathBuf::from(path.file_name().unwrap_or_default()).to_string_lossy().to_string();
        // TODO(fabric2seer): static `FilenameUtils.getBaseName` has no Rust counterpart in the port
        // rust: this.base_name = FilenameUtils::get_base_name(file_name.clone());
        // TODO(fabric2seer): static `FilenameUtils.getExtension` has no Rust counterpart in the port
        // rust: this.extension = FilenameUtils::get_extension(file_name.clone());
        this
    }

}
