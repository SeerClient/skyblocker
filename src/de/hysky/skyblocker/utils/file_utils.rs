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
        // TODO(fabric2seer): the condition: Files.isSymbolicLink
        //   java: Files.isDirectory(dir) && !Files.isSymbolicLink(dir)
        if todo!("fabric2seer: the condition (TODO above)") {
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
        // TODO(fabric2seer): the condition: Files.isWritable
        //   java: !Files.isWritable(dir) && !dir.toFile().setWritable(true)
        if todo!("fabric2seer: the condition (TODO above)") {
            logger().error(&format!("[Skyblocker] Failed to make file writable. Path: {}", std::path::absolute(&dir).unwrap_or_else(|_| dir.clone()).display()));
        }
        std::fs::remove_file(dir).expect("IOException");
    }

    /// java: public static String normalizePath(Path path)
    pub fn normalize_path(path: &std::path::PathBuf) -> String {
        return compat::regex::replace_all(&path.to_string_lossy().to_string().to_lowercase(), "[^a-z0-9_.-]", "");
    }

}
