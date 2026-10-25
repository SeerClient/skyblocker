//! Ported from io.github.moulberry.repo.NEUItems (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::i_reloadable::IReloadable;
use crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile;
use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;
use crate::io::github::moulberry::repo::util::stream_it::StreamIt;

/// java: public class NEUItems implements IReloadable
pub struct NEUItems {
    /// java: @Getter Map<@NEUId String, NEUItem> items; - `null` until the first reload, then replaced as a whole
    items: RwLock<Option<Arc<HashMap<String, Arc<NEUItem>>>>>,
}

impl NEUItems {
    pub(crate) fn new() -> Self {
        NEUItems { items: RwLock::new(None) }
    }

    /// java: public @Nullable NEUItem getItemBySkyblockId(@NEUId String itemId)
    pub fn get_item_by_skyblock_id(&self, item_id: &str) -> Option<Arc<NEUItem>> {
        let items = self.items.read().unwrap_or_else(|p| p.into_inner());
        // java: if (items == null) return null;
        let items = items.as_ref()?;
        // java: return items.get(itemId.toUpperCase(Locale.ROOT));
        items.get(&item_id.to_uppercase()).cloned()
    }

    /// java: public Map<@NEUId String, NEUItem> getItems()
    ///
    /// Java answers `null` before the first reload; the port answers an empty map there (every Skyblocker caller only
    /// reads it after the load).
    pub fn get_items(&self) -> Arc<HashMap<String, Arc<NEUItem>>> {
        self.items.read().unwrap_or_else(|p| p.into_inner()).clone().unwrap_or_default()
    }
}

impl IReloadable for NEUItems {
    /// java: public void reload(NEURepository repository) throws NEURepositoryException
    fn reload(&self, repository: &Arc<NEURepository>) -> Result<(), NEURepositoryException> {
        let mut items: HashMap<String, Arc<NEUItem>> = HashMap::new();
        // java: try (Stream<NEURepoFile> itemSources = repository.tree("items").filter(NEURepoFile::isFile)
        //       .filter(it -> it.getFsPath().getFileName().toString().toLowerCase(Locale.ROOT).endsWith(".json")))
        let item_sources = repository.tree("items")?.filter(NEURepoFile::is_file).filter(|it| {
            it.get_fs_path().file_name().map(|n| n.to_string_lossy().to_lowercase().ends_with(".json")).unwrap_or(false)
        });
        // java: for (NEURepoFile rf : new StreamIt<>(itemSources)) { NEUItem item = rf.json(NEUItem.class); items.put(item.getSkyblockItemId(), item); }
        for rf in StreamIt::new(item_sources) {
            let item: Arc<NEUItem> = rf.json_as(NEUItem::from_json)?;
            items.insert(item.get_skyblock_item_id().to_string(), item);
        }
        // java: this.items = items;
        *self.items.write().unwrap_or_else(|p| p.into_inner()) = Some(Arc::new(items));
        Ok(())
    }
}
