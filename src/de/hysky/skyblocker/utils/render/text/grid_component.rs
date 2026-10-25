//! Ported from de.hysky.skyblocker.utils.render.text.GridComponent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::network::chat::component_contents::ComponentContents;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class GridComponent
#[derive(Clone)]
pub struct GridComponent {}

impl GridComponent {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Component of(Component... components)
    pub fn of(components: &Vec<MutableComponent>) -> MutableComponent {
        return GridComponent::of_2("main", components);
    }

    /// java: public static Component of(String group, Component... components)
    pub fn of_2(group: &str, components: &Vec<MutableComponent>) -> MutableComponent {
        // java: return MutableComponent.create(new Contents(group, List.of(components)));
        // (a mod's `ComponentContents` class is the open variant `ComponentContents::Mod`)
        return MutableComponent::create(Arc::new(ComponentContents::Mod(Arc::new(Contents::new(group.to_string(), components.to_vec())))));
    }

}

/// java: public record Contents(String group, List<Component> components) implements ComponentContents
/// (Java's record equals/hashCode: by `group` and `components`)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Contents {
    /// java: String group
    pub group: String,
    /// java: List<Component> components
    pub components: Vec<MutableComponent>,
}

impl Contents {
    /// java: the canonical record constructor
    pub fn new(mut group: String, mut components: Vec<MutableComponent>) -> Self {
        Self { group, components }
    }

    /// java: the record accessor `group()`
    pub fn group(&self) -> String {
        self.group.clone()
    }

    /// java: the record accessor `components()`
    pub fn components(&self) -> Vec<MutableComponent> {
        self.components.clone()
    }

    /// java: @Override public MapCodec<? extends ComponentContents> codec()
    pub fn codec(&self) -> MapCodecRef<ComponentContents> {
        panic!("UnsupportedOperationException: {}", "Cannot serialize GridComponent.Contents");
    }

}

/// `record Contents(..) implements ComponentContents`: the interface's defaults (`visit` keeps going, `resolve` is
/// `MutableComponent.create(this)`), the record's `equals` / `hashCode` / `toString`, and its `codec()`.
impl mcrust::net::minecraft::network::chat::component_contents::ModComponentContents for Contents {
    /// java: @Override public MapCodec<? extends ComponentContents> codec()
    fn codec(&self) -> MapCodecRef<ComponentContents> {
        Contents::codec(self)
    }

    /// the record's `equals`: same class, `group` and `components` equal
    fn equals(&self, other: &dyn mcrust::net::minecraft::network::chat::component_contents::ModComponentContents) -> bool {
        other.as_any().downcast_ref::<Contents>().is_some_and(|other| other == self)
    }

    /// the record's `hashCode`: `31 * group.hashCode() + components.hashCode()`
    fn hash_code(&self) -> i32 {
        compat::string_hash_code(&self.group).wrapping_mul(31).wrapping_add(compat::hash_code(&self.components))
    }

    /// the record's `toString`
    fn to_string(&self) -> String {
        format!("Contents[group={}, components=[{}]]", self.group, self.components.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>().join(", "))
    }
}
