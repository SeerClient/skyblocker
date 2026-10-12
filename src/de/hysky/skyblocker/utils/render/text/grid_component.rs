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
        // rust: port gap - the port's `ComponentContents` is a closed enum of the seven vanilla contents types, so a
        // mod-defined `ComponentContents` implementation (this record) cannot be wrapped in a component (GAPS_batchA.md)
        let _contents = Contents::new(group.to_string(), components.to_vec());
        todo!("rust: port gap - ComponentContents is a closed enum; a mod cannot add GridComponent.Contents")
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
