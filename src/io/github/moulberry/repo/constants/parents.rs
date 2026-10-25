//! Ported from io.github.moulberry.repo.constants.Parents (neurepoparser 1.12.0).

use std::collections::HashMap;

/// java: @Getter public class Parents
#[derive(Clone, Debug, Default)]
pub struct Parents {
    /// java: Map<String, List<String>> parents;
    parents: Option<HashMap<String, Vec<String>>>,
    /// java: Map<String, String> reverseParents = new HashMap<>();
    reverse_parents: HashMap<String, String>,
}

impl Parents {
    /// java: public Parents(Map<String, List<String>> parents)
    pub fn new(parents: Option<HashMap<String, Vec<String>>>) -> Self {
        let mut reverse_parents: HashMap<String, String> = HashMap::new();
        // java: if (parents == null) return;
        if let Some(parents) = &parents {
            for (parent, children) in parents {
                for child in children {
                    reverse_parents.insert(child.clone(), parent.clone());
                }
            }
        }
        Parents { parents, reverse_parents }
    }

    /// java: public List<String> getChildren(String itemId)
    pub fn get_children(&self, item_id: &str) -> Vec<String> {
        // java: if (reverseParents == null) return new ArrayList<>(); return parents.getOrDefault(itemId, new ArrayList<>());
        self.parents.as_ref().and_then(|parents| parents.get(item_id).cloned()).unwrap_or_default()
    }

    /// java: public Optional<String> getParent(String itemId)
    pub fn get_parent(&self, item_id: &str) -> Option<String> {
        self.reverse_parents.get(item_id).cloned()
    }

    /// java: public Map<String, List<String>> getParents()
    pub fn get_parents(&self) -> Option<&HashMap<String, Vec<String>>> {
        self.parents.as_ref()
    }
}
