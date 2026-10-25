//! Ported from io.github.moulberry.repo.data.ItemOverlays (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::io::github::moulberry::repo::i_reloadable::IReloadable;
use crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile;
use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// java: public class ItemOverlays implements IReloadable
pub struct ItemOverlays {
    /// java: @Getter List<ItemOverlayFile> overlays; - replaced as a whole on each reload
    overlays: RwLock<Arc<Vec<ItemOverlayFile>>>,
}

/// java: @Value public static final class ItemOverlayFile
#[derive(Clone, Debug, PartialEq)]
pub struct ItemOverlayFile {
    /// java: NEURepoFile file;
    file: NEURepoFile,
    /// java: int version;
    version: i32,
    /// java: @NEUId String itemId;
    item_id: String,
}

impl ItemOverlayFile {
    /// java: public ItemOverlayFile(final NEURepoFile file, final int version, final String itemId)
    pub fn new(file: NEURepoFile, version: i32, item_id: String) -> Self {
        ItemOverlayFile { file, version, item_id }
    }

    /// java: public NEURepoFile getFile()
    pub fn get_file(&self) -> &NEURepoFile {
        &self.file
    }

    /// java: public int getVersion()
    pub fn get_version(&self) -> i32 {
        self.version
    }

    /// java: public String getItemId()
    pub fn get_item_id(&self) -> &str {
        &self.item_id
    }
}

impl std::fmt::Display for ItemOverlayFile {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemOverlays.ItemOverlayFile(file={:?}, version={}, itemId={})", self.file, self.version, self.item_id)
    }
}

/// java: private static final Pattern PATH_PATTERN = Pattern.compile("itemsOverlay/([0-9]+)/(.+)\\.snbt");
///
/// `PATH_PATTERN.matcher(path).matches()` (a whole-string match): the version digits and the item id, or `None`.
fn path_pattern_matches(path: &str) -> Option<(&str, &str)> {
    let rest = path.strip_prefix("itemsOverlay/")?;
    let slash = rest.find('/')?;
    let (digits, rest) = (&rest[..slash], &rest[slash + 1..]);
    // ([0-9]+)
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    // (.+)\.snbt - `.` does not match a line terminator
    let name = rest.strip_suffix(".snbt")?;
    if name.is_empty() || name.contains(['\n', '\r', '\u{85}', '\u{2028}', '\u{2029}']) {
        return None;
    }
    Some((digits, name))
}

impl ItemOverlays {
    fn new() -> Self {
        ItemOverlays { overlays: RwLock::new(Arc::new(Vec::new())) }
    }

    /// java: public static ItemOverlays forRepo(NEURepository repository)
    pub fn for_repo(repository: &Arc<NEURepository>) -> Arc<ItemOverlays> {
        let overlays = Arc::new(ItemOverlays::new());
        repository.register_reload_listener(overlays.clone());
        overlays
    }

    /// java: public Map<@NEUId String, ItemOverlayFile> getMostUpToDateCompatibleWith(int version)
    pub fn get_most_up_to_date_compatible_with(&self, version: i32) -> HashMap<String, ItemOverlayFile> {
        // java: overlays.stream().filter(it -> it.version <= version)
        //       .collect(Collectors.toMap(ItemOverlayFile::getItemId, Function.identity(), (a, b) -> a.version > b.version ? a : b));
        let mut map: HashMap<String, ItemOverlayFile> = HashMap::new();
        for it in self.get_overlays().iter().filter(|it| it.version <= version) {
            match map.get(&it.item_id) {
                Some(a) if a.version > it.version => {}
                _ => {
                    map.insert(it.item_id.clone(), it.clone());
                }
            }
        }
        map
    }

    /// java: public List<ItemOverlayFile> getOverlays()
    pub fn get_overlays(&self) -> Arc<Vec<ItemOverlayFile>> {
        self.overlays.read().unwrap_or_else(|p| p.into_inner()).clone()
    }
}

impl IReloadable for ItemOverlays {
    /// java: public void reload(NEURepository repository) throws NEURepositoryException
    fn reload(&self, repository: &Arc<NEURepository>) -> Result<(), NEURepositoryException> {
        // java: Stream<NEURepoFile> files = repository.tree("itemsOverlay");
        let files = repository.tree("itemsOverlay")?;
        // java: overlays = files.filter(NEURepoFile::isFile).map(file -> { Matcher matcher = PATH_PATTERN.matcher(file.getPath());
        //       if (matcher.matches()) { int version = Integer.parseInt(matcher.group(1)); String name = matcher.group(2);
        //       return new ItemOverlayFile(file, version, name); } return null; }).filter(Objects::nonNull).collect(Collectors.toList());
        let overlays: Vec<ItemOverlayFile> = files
            .filter(|file| file.is_file())
            .filter_map(|file| {
                let path = file.get_path();
                let (version, name) = path_pattern_matches(&path)?;
                let version: i32 = version.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", version));
                let name = name.to_string();
                Some(ItemOverlayFile::new(file, version, name))
            })
            .collect();
        *self.overlays.write().unwrap_or_else(|p| p.into_inner()) = Arc::new(overlays);
        Ok(())
    }
}
