//! Ported from io.github.moulberry.repo.IReloadable (neurepoparser 1.12.0).

use std::sync::Arc;

use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// java: public interface IReloadable
///
/// `Send + Sync` because the repository reloads on the loader thread and is read from every other one.
pub trait IReloadable: Send + Sync {
    /// java: void reload(NEURepository repository) throws NEURepositoryException;
    fn reload(&self, repository: &Arc<NEURepository>) -> Result<(), NEURepositoryException>;
}
