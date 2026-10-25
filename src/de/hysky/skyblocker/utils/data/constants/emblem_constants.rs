//! Ported from de.hysky.skyblocker.utils.data.constants.EmblemConstants by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Codec<EmblemConstants> CODEC = RecordCodecBuilder.create(instance -> instance.group( Emblem.LIST_CODEC.fieldOf("skills").forGetter(EmblemConstants::skills), Emblem.LIST_CODEC.field
pub static CODEC: compat::MainThread<CodecRef<EmblemConstants>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply7(record_codec_builder::of(|o: &EmblemConstants| o.skills(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "skills")), record_codec_builder::of(|o: &EmblemConstants| o.catacombs(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "catacombs")), record_codec_builder::of(|o: &EmblemConstants| o.levelling(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "levelling")), record_codec_builder::of(|o: &EmblemConstants| o.slayer(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "slayer")), record_codec_builder::of(|o: &EmblemConstants| o.achievement(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "achievement")), record_codec_builder::of(|o: &EmblemConstants| o.superstar(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "superstar")), record_codec_builder::of(|o: &EmblemConstants| o.special(), codec::field_of(EMBLEM_LIST_CODEC.get_mut().clone(), "special")), move |__a0, __a1, __a2, __a3, __a4, __a5, __a6| EmblemConstants::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone(), __a6.clone()))));
/// java: public static final EmblemConstants EMPTY = new EmblemConstants(List.of(), List.of(), List.of(), List.of(), List.of(), List.of(), List.of());
pub static EMPTY: compat::MainThread<EmblemConstants> = compat::MainThread::new(|| EmblemConstants::new(vec![], vec![], vec![], vec![], vec![], vec![], vec![]));
/// java: public static final Codec<Emblem> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("id").forGetter(Emblem::id), Codec.STRING.fieldOf("name").forGetter(Emblem::name), 
pub static EMBLEM_CODEC: compat::MainThread<CodecRef<Emblem>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &Emblem| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &Emblem| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Emblem| o.display(), codec::field_of(codec::xmap(mcrust::net::minecraft::network::chat::component_serialization::codec(), compat::component_of, |c: &MutableComponent| FormattedText::from(c.clone())), "display")), move |__a0, __a1, __a2| Emblem::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: public static final Codec<List<Emblem>> LIST_CODEC = CODEC.listOf();
pub static EMBLEM_LIST_CODEC: compat::MainThread<CodecRef<Vec<Emblem>>> = compat::MainThread::new(|| codec::list(EMBLEM_CODEC.get_mut().clone()));

/// java: public record EmblemConstants(List<Emblem> skills, List<Emblem> catacombs, List<Emblem> levelling, List<Emblem> slayer, List<Emblem> achievement, List<Emblem> superstar, List<Emblem> special)
#[derive(Clone, Debug)]
pub struct EmblemConstants {
    /// java: List<Emblem> skills
    pub skills: Vec<Emblem>,
    /// java: List<Emblem> catacombs
    pub catacombs: Vec<Emblem>,
    /// java: List<Emblem> levelling
    pub levelling: Vec<Emblem>,
    /// java: List<Emblem> slayer
    pub slayer: Vec<Emblem>,
    /// java: List<Emblem> achievement
    pub achievement: Vec<Emblem>,
    /// java: List<Emblem> superstar
    pub superstar: Vec<Emblem>,
    /// java: List<Emblem> special
    pub special: Vec<Emblem>,
}

impl EmblemConstants {
    /// java: the canonical record constructor
    pub fn new(mut skills: Vec<Emblem>, mut catacombs: Vec<Emblem>, mut levelling: Vec<Emblem>, mut slayer: Vec<Emblem>, mut achievement: Vec<Emblem>, mut superstar: Vec<Emblem>, mut special: Vec<Emblem>) -> Self {
        Self { skills, catacombs, levelling, slayer, achievement, superstar, special }
    }

    /// java: the record accessor `skills()`
    pub fn skills(&self) -> Vec<Emblem> {
        self.skills.clone()
    }

    /// java: the record accessor `catacombs()`
    pub fn catacombs(&self) -> Vec<Emblem> {
        self.catacombs.clone()
    }

    /// java: the record accessor `levelling()`
    pub fn levelling(&self) -> Vec<Emblem> {
        self.levelling.clone()
    }

    /// java: the record accessor `slayer()`
    pub fn slayer(&self) -> Vec<Emblem> {
        self.slayer.clone()
    }

    /// java: the record accessor `achievement()`
    pub fn achievement(&self) -> Vec<Emblem> {
        self.achievement.clone()
    }

    /// java: the record accessor `superstar()`
    pub fn superstar(&self) -> Vec<Emblem> {
        self.superstar.clone()
    }

    /// java: the record accessor `special()`
    pub fn special(&self) -> Vec<Emblem> {
        self.special.clone()
    }

    /// java: public List<Emblem> all()
    pub fn all(&self) -> Vec<Emblem> {
        // java: return List.of(this.skills(), this.catacombs(), this.levelling(), this.slayer(), this.achievement(), this.superstar(), this.special()).stream().flatMap(List::stream).toList();
        return vec![self.skills(), self.catacombs(), self.levelling(), self.slayer(), self.achievement(), self.superstar(), self.special()].into_iter().flatten().collect::<Vec<_>>();
    }

    /// java: public Optional<Emblem> fromId(String id)
    pub fn from_id(&self, id: &str) -> Option<Emblem> {
        return self.all().into_iter().filter(|emblem| (emblem.id() == id)).next();
    }

}

/// java: public record Emblem(String id, String name, Component display)
#[derive(Clone, Debug)]
pub struct Emblem {
    /// java: String id
    pub id: String,
    /// java: String name
    pub name: String,
    /// java: Component display
    pub display: MutableComponent,
}

impl Emblem {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut name: String, mut display: MutableComponent) -> Self {
        Self { id, name, display }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `display()`
    pub fn display(&self) -> MutableComponent {
        self.display.clone()
    }

}
