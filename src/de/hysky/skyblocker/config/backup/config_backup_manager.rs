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
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> backupConfig());
        // (`CLIENT_STOPPING` is Fabric's HEAD injector on `Minecraft#stop`)
        let _ = mcrust::mcrust::modding::hook::inject("net.minecraft.client.Minecraft#stop", mcrust::mcrust::modding::hook::At::Head, |_: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            ConfigBackupManager::backup_config();
        })
        .map_err(|e| logger().error(&format!("[Skyblocker] Failed to hook the client stop: {:?}", e)));
    }

    /// java: public static void backupConfig()
    pub fn backup_config() {
        ConfigBackupManager::backup_config_1(ConfigType::Main);
    }

    /// java: public static void backupConfig(ConfigType type)
    pub fn backup_config_1(mut r#type: ConfigType) {
        // java: try {..} catch (Exception e) - an exception in the body is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !(((r#type).fields().path).exists()) {
                return;
            }
            std::fs::create_dir_all(BACKUP_DIR.get_mut()).expect("IOException");
            let mut backups: Vec<std::path::PathBuf> = ConfigBackupManager::list_backups_1(r#type);
            // java: Files.mismatch(type.path, backups.getFirst()) == -1 - the two files have the same bytes
            if !backups.is_empty() && std::fs::read(&(r#type).fields().path).expect("IOException") == std::fs::read(&backups[0]).expect("IOException") {
                return;
            }
            let mut backup: std::path::PathBuf = BACKUP_DIR.get_mut().join(&(r#type).get_backup_file_name());
            // java: Files.copy(type.path, backup, StandardCopyOption.REPLACE_EXISTING);
            std::fs::copy(&(r#type).fields().path, &backup).expect("IOException");
            ConfigBackupManager::clean_old_backups(r#type);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to backup config! {}", compat::panic_message(&e)));
        }
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
            return compat::sorted_by(stream.filter(|__a0| (r#type).is_of_type(__a0)), |a, b| b.cmp(a)).collect::<Vec<_>>();
        }
    }

    /// java: public static void restoreBackup(Path backup) throws IOException
    pub fn restore_backup(backup: &std::path::PathBuf) {
        if !(ConfigType::Main.is_of_type(backup)) {
            logger().error(&format!("[Skyblocker Config Backup Manager] Attempted to restore a backup that is not of type MAIN: {}", backup.display()));
            return;
        }
        // java: Files.copy(backup, SkyblockerConfigManager.getConfigPath(), StandardCopyOption.REPLACE_EXISTING);
        std::fs::copy(backup, SkyblockerConfigManager::get_config_path()).expect("IOException");
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
        // java: return baseName + "_" + FORMATTER.format(LocalDateTime.now()) + "." + extension;
        // (`LocalDateTime.now()` is the system zone's wall clock: the port's formatter reads a `ZonedDateTime` at that zone)
        return format!("{}_{}.{}", self.fields().base_name, FORMATTER.get().format(mcrust::java::time::zoned_date_time::ZonedDateTime::now()), self.fields().extension);
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
        // java: this.baseName = FilenameUtils.getBaseName(fileName); this.extension = FilenameUtils.getExtension(fileName);
        this.base_name = compat::filename_get_base_name(&file_name);
        this.extension = compat::filename_get_extension(&file_name);
        this
    }

}
