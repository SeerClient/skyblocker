//! Ported from io.github.moulberry.repo.NEURepoFile (neurepoparser 1.12.0).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// java: public class NEURepoFile
#[derive(Clone)]
pub struct NEURepoFile {
    /// java: private NEURepository repository;
    repository: Arc<NEURepository>,
    /// java: private Path fsPath;
    fs_path: PathBuf,
}

impl NEURepoFile {
    /// java: NEURepoFile(final NEURepository repository, final Path fsPath) (package-private, lombok)
    pub(crate) fn new(repository: Arc<NEURepository>, fs_path: PathBuf) -> Self {
        NEURepoFile { repository, fs_path }
    }

    /// java: public String getPath() { return repository.baseFolder.relativize(fsPath).toString().replace("\\", "/"); }
    pub fn get_path(&self) -> String {
        let relative: &Path = self.fs_path.strip_prefix(&self.repository.base_folder).unwrap_or(&self.fs_path);
        relative.to_string_lossy().replace('\\', "/")
    }

    /// java: public <T> T json(TypeToken<T> type) / json(Class<T> $class) - the file parsed by the repository's Gson.
    ///
    /// The JSON tree; `json_as` binds it to a class.
    pub fn json(&self) -> Result<serde_json::Value, NEURepositoryException> {
        // java: try (Reader reader = Files.newBufferedReader(fsPath, StandardCharsets.UTF_8))
        let text: String = std::fs::read_to_string(&self.fs_path)
            // java: catch (IOException | SecurityException e) { throw new NEURepositoryException(getPath(), "Could not read file", e); }
            .map_err(|e| NEURepositoryException::new(&self.get_path(), "Could not read file", Some(e.to_string())))?;
        // java: catch (JsonSyntaxException e) { throw new NEURepositoryException(getPath(), "Invalid Json Syntax", e); }
        serde_json::from_str::<serde_json::Value>(&text)
            .map_err(|e| NEURepositoryException::new(&self.get_path(), "Invalid Json Syntax", Some(e.to_string())))
    }

    /// java: public <T> T json(Class<T> $class) - `json()` bound by `bind` (the class's Gson binding, `X::from_json`).
    pub fn json_as<T>(&self, bind: impl FnOnce(&serde_json::Value) -> T) -> Result<T, NEURepositoryException> {
        let tree = self.json()?;
        Ok(bind(&tree))
    }

    /// java: public InputStream stream() throws NEURepositoryException
    pub fn stream(&self) -> Result<std::fs::File, NEURepositoryException> {
        // java: return Files.newInputStream(fsPath);
        std::fs::File::open(&self.fs_path)
            // java: catch (IOException | SecurityException e) { throw new NEURepositoryException(getPath(), "Could not read file", e); }
            .map_err(|e| NEURepositoryException::new(&self.get_path(), "Could not read file", Some(e.to_string())))
    }

    /// `new String(stream().readAllBytes())` / `new InputStreamReader(stream())` read to the end - how every
    /// Skyblocker caller of `stream()` consumes it (the platform charset is UTF-8 on the JVMs Minecraft ships).
    pub fn read_to_string(&self) -> Result<String, NEURepositoryException> {
        use std::io::Read;
        let mut text = String::new();
        self.stream()?
            .read_to_string(&mut text)
            .map_err(|e| NEURepositoryException::new(&self.get_path(), "Could not read file", Some(e.to_string())))?;
        Ok(text)
    }

    /// java: public boolean isFile() { return Files.isRegularFile(fsPath); }
    pub fn is_file(&self) -> bool {
        self.fs_path.is_file()
    }

    /// java: public NEURepository getRepository()
    pub fn get_repository(&self) -> &Arc<NEURepository> {
        &self.repository
    }

    /// java: public Path getFsPath()
    pub fn get_fs_path(&self) -> &Path {
        &self.fs_path
    }
}

impl std::fmt::Debug for NEURepoFile {
    /// `Object.toString()` stands in as the path (the class has no `@ToString`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NEURepoFile({})", self.fs_path.display())
    }
}

impl PartialEq for NEURepoFile {
    /// `Object.equals` (no `@EqualsAndHashCode`): the same repository object and the same path is the closest a
    /// value type gets to Java's identity.
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.repository, &other.repository) && self.fs_path == other.fs_path
    }
}
