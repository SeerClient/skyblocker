//! Ported from de.hysky.skyblocker.utils.FileUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: public class FileUtils
#[derive(Clone)]
pub struct FileUtils {}

impl FileUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void recursiveDelete(Path dir) throws IOException
    pub fn recursive_delete(dir: &std::path::PathBuf) {
        if !dir.exists() {
            return;
        }
        // java: Files.isDirectory(dir) && !Files.isSymbolicLink(dir)
        let is_symbolic_link = std::fs::symlink_metadata(dir).map(|m| m.file_type().is_symlink()).unwrap_or(false);
        if dir.is_dir() && !is_symbolic_link {
            {
                let mut stream = std::fs::read_dir(dir).expect("IOException").filter_map(|e| e.ok()).map(|e| e.path());
                for child in stream {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        FileUtils::recursive_delete(&child);
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("[Skyblocker] Encountered an exception while deleting a file. Path: {}", std::path::absolute(&child).unwrap_or_else(|_| child.clone()).display()));
                    }
                }
            }
        }
        // java: !Files.isWritable(dir) && !dir.toFile().setWritable(true)
        let is_writable = std::fs::metadata(dir).map(|m| !m.permissions().readonly()).unwrap_or(false);
        let set_writable = || -> bool {
            match std::fs::metadata(dir) {
                Ok(m) => {
                    let mut permissions = m.permissions();
                    #[allow(clippy::permissions_set_readonly_false)]
                    permissions.set_readonly(false);
                    std::fs::set_permissions(dir, permissions).is_ok()
                }
                Err(_) => false,
            }
        };
        if !is_writable && !set_writable() {
            logger().error(&format!("[Skyblocker] Failed to make file writable. Path: {}", std::path::absolute(&dir).unwrap_or_else(|_| dir.clone()).display()));
        }
        // java: Files.delete(dir) - a file, a link or an (emptied) directory
        if dir.is_dir() && !is_symbolic_link {
            std::fs::remove_dir(dir).expect("IOException");
        } else {
            std::fs::remove_file(dir).expect("IOException");
        }
    }

    /// java: public static String normalizePath(Path path)
    pub fn normalize_path(path: &std::path::PathBuf) -> String {
        return compat::regex::replace_all(&path.to_string_lossy().to_string().to_lowercase(), "[^a-z0-9_.-]", "");
    }

}
