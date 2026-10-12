//! Types the mod uses that neither the mod, the JDK mapping nor the port has (fabric2seer).
//! Each is a placeholder: give it the real type (a library crate, or port the class).

/// TODO(fabric2seer): java de.hysky.skyblocker.mixins.accessors.AbstractContainerScreenAccessor
pub struct AbstractContainerScreenAccessor;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.ints.AbstractInt2ObjectMap
pub struct AbstractInt2ObjectMap;
/// TODO(fabric2seer): java org.apache.logging.log4j.core.appender.AbstractOutputStreamAppender
pub struct AbstractOutputStreamAppender;
/// TODO(fabric2seer): java mezz.jei.api.recipe.category.AbstractRecipeCategory
pub struct AbstractRecipeCategory;
/// TODO(fabric2seer): java java.util.AbstractSet
pub struct AbstractSet;
/// TODO(fabric2seer): java dev.isxander.yacl3.gui.AbstractWidget
pub struct AbstractWidget;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.renderer.v1.render.AltModelBlockRenderer
pub struct AltModelBlockRenderer;
/// TODO(fabric2seer): java de.hysky.skyblocker.utils.Http.ApiResponse
pub struct ApiResponse;
/// TODO(fabric2seer): java org.apache.logging.log4j.core.Appender
pub struct Appender;
/// TODO(fabric2seer): java ApplicabilityResult
pub struct ApplicabilityResult;
/// TODO(fabric2seer): java java.lang.foreign.Arena
pub struct Arena;
/// TODO(fabric2seer): java java.util.concurrent.atomic.AtomicReference
pub struct AtomicReference;
/// TODO(fabric2seer): java java.math.BigDecimal
pub struct BigDecimal;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.controllers.BooleanController
pub struct BooleanController;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.booleans.BooleanPredicate
pub struct BooleanPredicate;
/// TODO(fabric2seer): java java.io.BufferedReader
pub struct BufferedReader;
/// TODO(fabric2seer): java java.io.BufferedWriter
pub struct BufferedWriter;
/// TODO(fabric2seer): java com.google.common.collect.ImmutableList.Builder, com.google.common.collect.ImmutableMap.Builder
pub struct Builder;
/// TODO(fabric2seer): java java.io.ByteArrayOutputStream
pub struct ByteArrayOutputStream;
/// TODO(fabric2seer): java java.nio.ByteBuffer
pub struct ByteBuffer;
/// TODO(fabric2seer): java net.azureaaron.networth.Calculation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Calculation;
/// TODO(fabric2seer): java net.minecraft.world.entity.animal.feline.CatSoundVariants
pub struct CatSoundVariants;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.category.CategoryIdentifier
pub struct CategoryIdentifier;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.category.CategoryRegistry
pub struct CategoryRegistry;
/// TODO(fabric2seer): java java.lang.Class
pub struct Class;
/// TODO(fabric2seer): java org.objectweb.asm.tree.ClassNode
pub struct ClassNode;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.gametest.v1.context.ClientGameTestContext
pub struct ClientGameTestContext;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.entry.CollapsibleEntryRegistry
pub struct CollapsibleEntryRegistry;
/// TODO(fabric2seer): java java.util.stream.Collector
pub struct Collector;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.controllers.ColourController
pub struct ColourController;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.entry.comparison.ComparisonContext
pub struct ComparisonContext;
/// TODO(fabric2seer): java java.util.concurrent.CompletionStage
pub struct CompletionStage;
/// TODO(fabric2seer): java dev.architectury.event.CompoundEventResult
pub struct CompoundEventResult;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.ConfigCategory
pub struct ConfigCategory;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.ConfigManager
pub struct ConfigManager;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.patching.ConfigPatch
pub struct ConfigPatch;
/// ModMenu's `com.terraformersmc.modmenu.api.ConfigScreenFactory<S extends Screen>`: `S create(Screen parent)`.
/// (ModMenu itself is another mod: nothing in the port asks for the factory yet.)
pub type ConfigScreenFactory = std::sync::Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>;
/// Dandelion's `net.azureaaron.dandelion.api.ConfigType`: which config library builds the config screen
/// (`YACL`, `MOUL_CONFIG`). Only the value is kept here; the libraries themselves are not ported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConfigType {
    /// java: YACL
    Yacl,
    /// java: MOUL_CONFIG
    MoulConfig,
}
/// TODO(fabric2seer): java net.minecraft.client.gui.components.events.ContainerEventHandler
pub struct ContainerEventHandler;
/// TODO(fabric2seer): java Context, net.fabricmc.fabric.api.client.rendering.v1.PictureInPictureRendererRegistry.Context
pub struct Context;
/// TODO(fabric2seer): java dev.isxander.yacl3.api.controller.ControllerBuilder
pub struct ControllerBuilder;
/// TODO(fabric2seer): java net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberFieldEditor
pub struct DandelionNumberFieldEditor;
/// TODO(fabric2seer): java net.minecraft.client.DeltaTracker
pub struct DeltaTracker;
/// TODO(fabric2seer): java dev.isxander.yacl3.api.utils.Dimension
pub struct Dimension;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.display.Display
pub struct Display;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.display.DisplayRegistry
pub struct DisplayRegistry;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.display.DisplaySerializer
pub struct DisplaySerializer;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.doubles.Double2ObjectFunction
pub struct Double2ObjectFunction;
/// TODO(fabric2seer): java java.util.function.DoubleConsumer
pub struct DoubleConsumer;
/// TODO(fabric2seer): java de.hysky.skyblocker.mixins.accessors.EditBoxAccessor
pub struct EditBoxAccessor;
/// TODO(fabric2seer): java dev.emi.emi.api.stack.EmiIngredient
pub struct EmiIngredient;
/// TODO(fabric2seer): java dev.emi.emi.api.recipe.EmiRecipeCategory
pub struct EmiRecipeCategory;
/// TODO(fabric2seer): java dev.emi.emi.api.EmiRegistry
pub struct EmiRegistry;
/// TODO(fabric2seer): java dev.emi.emi.api.stack.EmiStack
pub struct EmiStack;
/// TODO(fabric2seer): java net.minecraft.world.entity.EntitySelector
pub struct EntitySelector;
/// TODO(fabric2seer): java net.minecraft.world.entity.EntityTypes
pub struct EntityTypes;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.entry.EntryIngredient
pub struct EntryIngredient;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.entry.EntryRegistry
pub struct EntryRegistry;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.entry.EntryStack
pub struct EntryStack;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.controllers.EnumController
pub struct EnumController;
/// java net.azureaaron.hmapi.data.server.Environment (the hm-api library, ported here: it is three constants)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Environment {
    /// The main server network (e.g. mc.hypixel.net)
    Production,
    /// The alpha server (alpha.hypixel.net)
    Beta,
    /// Hypixel's internal developer server
    Test,
}
/// TODO(fabric2seer): java net.fabricmc.fabric.api.event.Event
pub struct Event;
/// TODO(fabric2seer): java java.lang.Exception
pub struct Exception;
/// TODO(fabric2seer): java com.operationpotato.itemlist.api.ExcludedScreensManager
pub struct ExcludedScreensManager;
/// TODO(fabric2seer): java com.operationpotato.itemlist.api.ExclusionZoneManager
pub struct ExclusionZoneManager;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.screen.ExclusionZones
pub struct ExclusionZones;
/// TODO(fabric2seer): java java.util.concurrent.ExecutorService
pub struct ExecutorService;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource
pub struct FabricClientCommandSource;
/// TODO(fabric2seer): java net.fabricmc.loader.api.FabricLoader
pub struct FabricLoader;
/// TODO(fabric2seer): java java.lang.reflect.Field
pub struct Field;
/// TODO(fabric2seer): java java.io.FileWriter
pub struct FileWriter;
/// TODO(fabric2seer): java de.hysky.skyblocker.mixins.accessors.FontAccessor
pub struct FontAccessor;
/// TODO(fabric2seer): java net.minecraft.client.gui.Font$GlyphVisitor
pub struct Font_GlyphVisitor;
/// TODO(fabric2seer): java java.util.zip.GZIPInputStream
pub struct GZIPInputStream;
/// TODO(fabric2seer): java java.util.zip.GZIPOutputStream
pub struct GZIPOutputStream;
/// TODO(fabric2seer): java org.eclipse.jgit.api.Git
pub struct Git;
/// TODO(fabric2seer): java com.google.gson.Gson
pub struct Gson;
/// TODO(fabric2seer): java net.azureaaron.dandelion.deps.moulconfig.gui.GuiOptionEditor
pub struct GuiOptionEditor;
/// TODO(fabric2seer): java com.operationpotato.itemlist.api.HoveredItemManager
pub struct HoveredItemManager;
/// TODO(fabric2seer): java java.net.http.HttpClient
pub struct HttpClient;
/// TODO(fabric2seer): java java.net.http.HttpHeaders
pub struct HttpHeaders;
/// TODO(fabric2seer): java java.net.http.HttpResponse
pub struct HttpResponse;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement
pub struct HudElement;
/// TODO(fabric2seer): java net.azureaaron.hmapi.network.packet.s2c.HypixelS2CPacket
pub struct HypixelS2CPacket;
/// TODO(fabric2seer): java mezz.jei.api.gui.ingredient.ICraftingGridHelper
pub struct ICraftingGridHelper;
/// TODO(fabric2seer): java mezz.jei.api.gui.drawable.IDrawableStatic
pub struct IDrawableStatic;
/// TODO(fabric2seer): java mezz.jei.api.recipe.IFocusGroup
pub struct IFocusGroup;
/// TODO(fabric2seer): java mezz.jei.api.registration.IGuiHandlerRegistration
pub struct IGuiHandlerRegistration;
/// TODO(fabric2seer): java mezz.jei.api.helpers.IGuiHelper
pub struct IGuiHelper;
/// TODO(fabric2seer): java org.spongepowered.asm.mixin.extensibility.IMixinInfo
pub struct IMixinInfo;
/// TODO(fabric2seer): java mezz.jei.api.registration.IRecipeCategoryRegistration
pub struct IRecipeCategoryRegistration;
/// TODO(fabric2seer): java mezz.jei.api.gui.builder.IRecipeLayoutBuilder
pub struct IRecipeLayoutBuilder;
/// TODO(fabric2seer): java mezz.jei.api.registration.IRecipeRegistration
pub struct IRecipeRegistration;
/// TODO(fabric2seer): java mezz.jei.api.gui.ingredient.IRecipeSlotsView
pub struct IRecipeSlotsView;
/// TODO(fabric2seer): java mezz.jei.api.recipe.types.IRecipeType
pub struct IRecipeType;
/// TODO(fabric2seer): java dev.isxander.yacl3.gui.controllers.string.IStringController
pub struct IStringController;
/// TODO(fabric2seer): java mezz.jei.api.registration.ISubtypeRegistration
pub struct ISubtypeRegistration;
/// TODO(fabric2seer): java com.mojang.blaze3d.platform.InputConstants
pub struct InputConstants;
/// TODO(fabric2seer): java java.io.InputStream
pub struct InputStream;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.ints.IntSortedSet
pub struct IntSortedSet;
/// TODO(fabric2seer): java net.azureaaron.dandelion.impl.controllers.IntegerControllerImpl
pub struct IntegerControllerImpl;
/// TODO(fabric2seer): java net.minecraft.client.server.IntegratedServer
pub struct IntegratedServer;
/// TODO(fabric2seer): java me.shedaniel.rei.api.common.entry.comparison.ItemComparatorRegistry
pub struct ItemComparatorRegistry;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.ResourcePack.ItemModels
pub struct ItemModels;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.ItemOverlays.ItemOverlayFile
pub struct ItemOverlayFile;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.ItemOverlays
pub struct ItemOverlays;
/// TODO(fabric2seer): java com.google.gson.JsonDeserializationContext
pub struct JsonDeserializationContext;
/// TODO(fabric2seer): java com.google.gson.JsonSerializationContext
pub struct JsonSerializationContext;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionContext
pub struct LevelExtractionContext;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderContext
pub struct LevelRenderContext;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.rendering.v1.level.LevelTerrainRenderContext
pub struct LevelTerrainRenderContext;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.Leveling
pub struct Leveling;
/// TODO(fabric2seer): java org.apache.commons.math3.geometry.euclidean.threed.Line, org.apache.commons.math3.geometry.euclidean.twod.Line
pub struct Line;
/// TODO(fabric2seer): java moe.nea.lisp.LispData
pub struct LispData;
/// TODO(fabric2seer): java moe.nea.lisp.LispExecutionContext
pub struct LispExecutionContext;
/// TODO(fabric2seer): java moe.nea.lisp.LispData.LispNumber
pub struct LispNumber;
/// TODO(fabric2seer): java moe.nea.lisp.LispData.LispString
pub struct LispString;
/// TODO(fabric2seer): java com.mojang.logging.LogUtils
pub struct LogUtils;
/// TODO(fabric2seer): java org.apache.logging.log4j.core.LoggerContext, org.apache.logging.log4j.spi.LoggerContext
pub struct LoggerContext;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.longs.LongPriorityQueue
pub struct LongPriorityQueue;
/// TODO(fabric2seer): java MethodHandles.Lookup
pub struct Lookup;
/// TODO(fabric2seer): java net.minecraft.world.level.saveddata.maps.MapDecorationTypes
pub struct MapDecorationTypes;
/// TODO(fabric2seer): java java.util.regex.MatchResult
pub struct MatchResult;
/// TODO(fabric2seer): java java.lang.foreign.MemorySegment
pub struct MemorySegment;
/// TODO(fabric2seer): java java.lang.invoke.MethodHandle
pub struct MethodHandle;
/// TODO(fabric2seer): java java.lang.invoke.MethodType
pub struct MethodType;
/// TODO(fabric2seer): java net.minecraft.server.MinecraftServer
pub struct MinecraftServer;
/// TODO(fabric2seer): java Month
pub struct Month;
/// TODO(fabric2seer): java net.azureaaron.dandelion.impl.moulconfig.MoulConfigDefinition
pub struct MoulConfigDefinition;
/// TODO(fabric2seer): java com.google.common.collect.Multimap
pub struct Multimap;
/// TODO(fabric2seer): java org.apache.commons.lang3.mutable.MutableInt
pub struct MutableInt;
/// TODO(fabric2seer): java org.apache.commons.lang3.tuple.MutableTriple
pub struct MutableTriple;
/// TODO(fabric2seer): java io.github.moulberry.repo.NEUConstants
pub struct NEUConstants;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUCraftingRecipe
pub struct NEUCraftingRecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUForgeRecipe
pub struct NEUForgeRecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUIngredient
pub struct NEUIngredient;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUItem
pub struct NEUItem;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUKatUpgradeRecipe
pub struct NEUKatUpgradeRecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUMobDropRecipe
pub struct NEUMobDropRecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUNpcShopRecipe
pub struct NEUNpcShopRecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEURecipe
pub struct NEURecipe;
/// TODO(fabric2seer): java io.github.moulberry.repo.NEURecipeCache
pub struct NEURecipeCache;
/// TODO(fabric2seer): java io.github.moulberry.repo.NEURepoFile
pub struct NEURepoFile;
/// TODO(fabric2seer): java io.github.moulberry.repo.NEURepository
pub struct NEURepository;
/// TODO(fabric2seer): java io.github.moulberry.repo.data.NEUTradeRecipe
pub struct NEUTradeRecipe;
/// TODO(fabric2seer): java NarrationPriority
pub struct NarrationPriority;
/// TODO(fabric2seer): java net.azureaaron.networth.NetworthResult
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NetworthResult;
/// TODO(fabric2seer): java java.io.ObjectInputStream
pub struct ObjectInputStream;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.objects.ObjectIterator
pub struct ObjectIterator;
/// TODO(fabric2seer): java it.unimi.dsi.fastutil.objects.ObjectSortedSet
pub struct ObjectSortedSet;
/// TODO(fabric2seer): java dev.isxander.yacl3.api.Option, net.azureaaron.dandelion.api.Option
pub struct Option;
/// TODO(fabric2seer): java net.azureaaron.dandelion.api.OptionGroup
pub struct OptionGroup;
/// TODO(fabric2seer): java java.io.OutputStream
pub struct OutputStream;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.networking.v1.PacketSender
pub struct PacketSender;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.PetLevelingBehaviourOverride
pub struct PetLevelingBehaviourOverride;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.PetLevelingData
pub struct PetLevelingData;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.PetNumbers - the NEU repo's per-rarity pet numbers (the
/// repo library is not ported: no instance is made until it is)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PetNumbers {
    /// `int lowLevel`
    pub low_level: i32,
    /// `int highLevel`
    pub high_level: i32,
}

impl PetNumbers {
    /// `getLowLevel()`
    pub fn get_low_level(&self) -> i32 {
        self.low_level
    }

    /// `getHighLevel()`
    pub fn get_high_level(&self) -> i32 {
        self.high_level
    }
}
/// TODO(fabric2seer): java me.shedaniel.math.Point
pub struct Point;
/// TODO(fabric2seer): java net.minecraft.client.gui.components.PopupScreen$Builder
pub struct PopupScreen_Builder;
/// TODO(fabric2seer): java java.util.PriorityQueue
pub struct PriorityQueue;
/// TODO(fabric2seer): java net.azureaaron.dandelion.deps.moulconfig.processor.ProcessedOption
pub struct ProcessedOption;
/// TODO(fabric2seer): java moe.nea.lisp.LispAst.Program
pub struct Program;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter
pub struct QuadEmitter;
/// TODO(fabric2seer): java java.util.Random
pub struct Random;
/// java net.azureaaron.hmapi.data.rank.PlayerRank (hm-api)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerRank {
    Normal,
    Youtuber,
    GameMaster,
    Admin,
}

impl PlayerRank {
    pub const VALUES: [PlayerRank; 4] = [PlayerRank::Normal, PlayerRank::Youtuber, PlayerRank::GameMaster, PlayerRank::Admin];

    /// java: public int id()
    pub fn id(self) -> i32 {
        match self {
            PlayerRank::Normal => 1,
            PlayerRank::Youtuber => 2,
            PlayerRank::GameMaster => 3,
            PlayerRank::Admin => 4,
        }
    }

    /// java: BY_ID = ByIdMap.sparse(PlayerRank::id, values(), NORMAL)
    pub fn by_id(id: i32) -> PlayerRank {
        Self::VALUES.into_iter().find(|r| r.id() == id).unwrap_or(PlayerRank::Normal)
    }
}

/// java net.azureaaron.hmapi.data.rank.PackageRank (hm-api)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PackageRank {
    None,
    Vip,
    VipPlus,
    Mvp,
    MvpPlus,
}

impl PackageRank {
    pub const VALUES: [PackageRank; 5] = [PackageRank::None, PackageRank::Vip, PackageRank::VipPlus, PackageRank::Mvp, PackageRank::MvpPlus];

    /// java: public int id()
    pub fn id(self) -> i32 {
        match self {
            PackageRank::None => 1,
            PackageRank::Vip => 2,
            PackageRank::VipPlus => 3,
            PackageRank::Mvp => 4,
            PackageRank::MvpPlus => 5,
        }
    }

    /// java: BY_ID = ByIdMap.sparse(PackageRank::id, values(), NONE)
    pub fn by_id(id: i32) -> PackageRank {
        Self::VALUES.into_iter().find(|r| r.id() == id).unwrap_or(PackageRank::None)
    }
}

/// java net.azureaaron.hmapi.data.rank.MonthlyPackageRank (hm-api)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MonthlyPackageRank {
    None,
    /// MVP++
    Superstar,
}

impl MonthlyPackageRank {
    pub const VALUES: [MonthlyPackageRank; 2] = [MonthlyPackageRank::None, MonthlyPackageRank::Superstar];

    /// java: public int id()
    pub fn id(self) -> i32 {
        match self {
            MonthlyPackageRank::None => 1,
            MonthlyPackageRank::Superstar => 2,
        }
    }

    /// java: BY_ID = ByIdMap.sparse(MonthlyPackageRank::id, values(), NONE)
    pub fn by_id(id: i32) -> MonthlyPackageRank {
        Self::VALUES.into_iter().find(|r| r.id() == id).unwrap_or(MonthlyPackageRank::None)
    }
}

/// java net.azureaaron.hmapi.data.rank.RankType (hm-api): `sealed interface RankType permits MonthlyPackageRank,
/// PackageRank, PlayerRank` - a value of one of the three enums.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RankType {
    Player(PlayerRank),
    Package(PackageRank),
    Monthly(MonthlyPackageRank),
}

impl RankType {
    /// java: static RankType getEffectiveRank(PlayerRank baseRank, PackageRank packageRank, MonthlyPackageRank monthlyPackageRank)
    pub fn get_effective_rank(base_rank: PlayerRank, package_rank: PackageRank, monthly_package_rank: MonthlyPackageRank) -> RankType {
        match base_rank {
            // Staff ranks take precedence
            PlayerRank::Youtuber | PlayerRank::GameMaster | PlayerRank::Admin => RankType::Player(base_rank),
            PlayerRank::Normal => match monthly_package_rank {
                MonthlyPackageRank::None => RankType::Package(package_rank),
                MonthlyPackageRank::Superstar => RankType::Monthly(monthly_package_rank),
            },
        }
    }

    /// java: static int compare(RankType first, RankType second)
    pub fn compare(first: RankType, second: RankType) -> i32 {
        RankType::get_rank_weight(first).cmp(&RankType::get_rank_weight(second)) as i32
    }

    /// java: private static int getRankWeight(RankType rank)
    fn get_rank_weight(rank: RankType) -> i32 {
        match rank {
            RankType::Player(PlayerRank::Normal) | RankType::Monthly(MonthlyPackageRank::None) => -1,
            RankType::Package(PackageRank::None) => 0,
            RankType::Package(PackageRank::Vip) => 1,
            RankType::Package(PackageRank::VipPlus) => 2,
            RankType::Package(PackageRank::Mvp) => 3,
            RankType::Package(PackageRank::MvpPlus) => 4,
            RankType::Monthly(MonthlyPackageRank::Superstar) => 5,
            RankType::Player(PlayerRank::Youtuber) => 6,
            RankType::Player(PlayerRank::GameMaster) => 7,
            RankType::Player(PlayerRank::Admin) => 8,
        }
    }
}
/// java io.github.moulberry.repo.data.Rarity - an enum of the NEU repo library (the rest of the library is not
/// ported, GAPS_batchB L1). Its constants are the ones the mod names (`SkyblockItemRarity.toNeuRarity`, the pet
/// tables); the library's declaration order is not on disk, so the enum derives no ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Divine,
    Special,
    VerySpecial,
    Supreme,
    Unknown,
}
/// TODO(fabric2seer): java de.hysky.skyblocker.mixins.accessors.RecipeBookComponentAccessor
pub struct RecipeBookComponentAccessor;
/// TODO(fabric2seer): java me.shedaniel.math.Rectangle
pub struct Rectangle;
/// TODO(fabric2seer): java org.eclipse.jgit.lib.Ref
pub struct Ref;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.gui.Renderer
pub struct Renderer;
/// TODO(fabric2seer): java Result
pub struct Result;
/// TODO(fabric2seer): java meteordevelopment.discordipc.RichPresence
pub struct RichPresence;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.screen.ScreenRegistry
pub struct ScreenRegistry;
/// TODO(fabric2seer): java Season
pub struct Season;
/// TODO(fabric2seer): java net.minecraft.server.level.ServerPlayer
pub struct ServerPlayer;
/// TODO(fabric2seer): java org.apache.commons.math3.stat.regression.SimpleRegression
pub struct SimpleRegression;
/// TODO(fabric2seer): java net.azureaaron.networth.data.SkyblockItemData
pub struct SkyblockItemData;
/// TODO(fabric2seer): java net.azureaaron.networth.item.SkyblockItemStack
pub struct SkyblockItemStack;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.gui.widgets.Slot
pub struct Slot;
/// TODO(fabric2seer): java de.hysky.skyblocker.mixins.accessors.SlotAccessor
pub struct SlotAccessor;
/// TODO(fabric2seer): java net.minecraft.sounds.SoundEvents
pub struct SoundEvents;
/// TODO(fabric2seer): java moe.nea.lisp.StackFrame
pub struct StackFrame;
/// TODO(fabric2seer): java io.github.moulberry.repo.constants.PetNumbers.Stats
pub struct Stats;
/// TODO(fabric2seer): java dev.isxander.yacl3.gui.controllers.string.StringControllerElement
pub struct StringControllerElement;
/// TODO(fabric2seer): java net.minecraft.world.level.levelgen.structure.templatesystem.StructurePlaceSettings
pub struct StructurePlaceSettings;
/// TODO(fabric2seer): java net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate
pub struct StructureTemplate;
/// TODO(fabric2seer): java mezz.jei.library.ingredients.subtypes.SubtypeInterpreters
pub struct SubtypeInterpreters;
/// TODO(fabric2seer): java java.lang.foreign.SymbolLookup
pub struct SymbolLookup;
/// TODO(fabric2seer): java T
pub struct T;
/// TODO(fabric2seer): java com.google.common.collect.Table
pub struct Table;

/// Apache Commons Lang's `MutableTriple<L, M, R>`: three public fields, the right one settable.
#[derive(Clone, Debug, PartialEq)]
pub struct MutableTripleOf<L, M, R> {
    pub left: L,
    pub middle: M,
    pub right: R,
}

impl<L: Clone, M: Clone, R: Clone> MutableTripleOf<L, M, R> {
    /// `MutableTriple.of(left, middle, right)`.
    pub fn of(left: L, middle: M, right: R) -> Self {
        MutableTripleOf { left, middle, right }
    }
    pub fn get_left(&self) -> L {
        self.left.clone()
    }
    pub fn get_middle(&self) -> M {
        self.middle.clone()
    }
    pub fn get_right(&self) -> R {
        self.right.clone()
    }
    pub fn set_right(&mut self, right: R) {
        self.right = right;
    }
}

/// Guava's `HashBasedTable<R, C, V>` (`Table` created by `HashBasedTable.create()`): a map of rows to maps of
/// columns to values, both LINKED (insertion order, as Guava's `StandardTable` over `LinkedHashMap`s). The methods are
/// Guava's, with Java's null answers as `None`.
#[derive(Clone, Debug)]
pub struct HashBasedTable<R, C, V> {
    rows: Vec<(R, Vec<(C, V)>)>,
}

impl<R, C, V> Default for HashBasedTable<R, C, V> {
    fn default() -> Self {
        HashBasedTable { rows: Vec::new() }
    }
}

impl<R: PartialEq + Clone, C: PartialEq + Clone, V: Clone> HashBasedTable<R, C, V> {
    /// `HashBasedTable.create()`.
    pub fn create() -> Self {
        Self::default()
    }

    /// `put(row, column, value)`: the previous value, if any.
    pub fn put(&mut self, row: R, column: C, value: V) -> std::option::Option<V> {
        let cells = match self.rows.iter().position(|(r, _)| *r == row) {
            Some(i) => &mut self.rows[i].1,
            None => {
                self.rows.push((row, Vec::new()));
                &mut self.rows.last_mut().unwrap().1
            }
        };
        match cells.iter().position(|(c, _)| *c == column) {
            Some(i) => Some(std::mem::replace(&mut cells[i].1, value)),
            None => {
                cells.push((column, value));
                None
            }
        }
    }

    /// `get(row, column)`.
    pub fn get(&self, row: &R, column: &C) -> std::option::Option<&V> {
        self.rows.iter().find(|(r, _)| r == row)?.1.iter().find(|(c, _)| c == column).map(|(_, v)| v)
    }

    /// `remove(row, column)`: the removed value; a row left empty is dropped, as Guava's is.
    pub fn remove(&mut self, row: &R, column: &C) -> std::option::Option<V> {
        let ri = self.rows.iter().position(|(r, _)| r == row)?;
        let ci = self.rows[ri].1.iter().position(|(c, _)| c == column)?;
        let (_, v) = self.rows[ri].1.remove(ci);
        if self.rows[ri].1.is_empty() {
            self.rows.remove(ri);
        }
        Some(v)
    }

    /// `containsRow(row)`.
    pub fn contains_row(&self, row: &R) -> bool {
        self.rows.iter().any(|(r, _)| r == row)
    }

    /// `contains(row, column)`.
    pub fn contains(&self, row: &R, column: &C) -> bool {
        self.get(row, column).is_some()
    }

    /// `row(row)`: the row's column -> value cells (a copy; empty for a missing row).
    pub fn row(&self, row: &R) -> Vec<(C, V)> {
        self.rows.iter().find(|(r, _)| r == row).map(|(_, cells)| cells.clone()).unwrap_or_default()
    }

    /// `column(column)`: the column's row -> value cells, in row order (a copy).
    pub fn column(&self, column: &C) -> Vec<(R, V)> {
        self.rows
            .iter()
            .filter_map(|(r, cells)| cells.iter().find(|(c, _)| c == column).map(|(_, v)| (r.clone(), v.clone())))
            .collect()
    }

    /// `values()`, row by row.
    pub fn values(&self) -> Vec<V> {
        self.rows.iter().flat_map(|(_, cells)| cells.iter().map(|(_, v)| v.clone())).collect()
    }

    /// `rowMap()`: every row with its cells (a copy).
    pub fn row_map(&self) -> Vec<(R, Vec<(C, V)>)> {
        self.rows.clone()
    }

    /// `size()`.
    pub fn size(&self) -> usize {
        self.rows.iter().map(|(_, cells)| cells.len()).sum()
    }

    /// `isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// `clear()`.
    pub fn clear(&mut self) {
        self.rows.clear();
    }
}
/// TODO(fabric2seer): java java.time.temporal.Temporal
pub struct Temporal;
/// TODO(fabric2seer): java java.time.temporal.TemporalAccessor
pub struct TemporalAccessor;
/// TODO(fabric2seer): java java.time.temporal.TemporalQuery
pub struct TemporalQuery;
/// TODO(fabric2seer): java java.time.temporal.TemporalUnit
pub struct TemporalUnit;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.client.gametest.v1.context.TestSingleplayerContext
pub struct TestSingleplayerContext;
/// TODO(fabric2seer): java java.util.concurrent.TimeUnit
pub struct TimeUnit;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.registry.transfer.TransferHandlerRegistry
pub struct TransferHandlerRegistry;
/// TODO(fabric2seer): java org.apache.logging.log4j.util.TriConsumer
pub struct TriConsumer;
/// TODO(fabric2seer): java net.fabricmc.fabric.api.util.TriState
pub struct TriState;
/// TODO(fabric2seer): java org.apache.commons.lang3.tuple.Triple
pub struct Triple;
/// TODO(fabric2seer): java java.lang.reflect.Type, net.azureaaron.networth.Calculation.Type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Type;

/// The placeholder's `toString()` (the library's `Calculation.Type` is an enum named by its constant).
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Type")
    }
}
/// TODO(fabric2seer): java net.minecraft.util.Util
pub struct Util;
/// TODO(fabric2seer): java java.time.temporal.ValueRange
pub struct ValueRange;
/// TODO(fabric2seer): java org.apache.commons.math3.geometry.euclidean.twod.Vector2D
pub struct Vector2D;
/// `org.joml.Vector2d` / its read-only view `Vector2dc` (JOML 1.10, a library the mod uses that the game crate
/// has no `Vector2d` of): the methods the mod calls, with JOML's bodies. `sub` / `div` / `mul` / `add` mutate
/// `this` and return it, as JOML's do.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2dc {
    pub x: f64,
    pub y: f64,
}

/// `org.joml.Vector2d` (the same class; `Vector2dc` is its interface).
pub type Vector2d = Vector2dc;

impl Vector2dc {
    /// `new Vector2d(x, y)`.
    pub fn new(x: f64, y: f64) -> Vector2dc {
        Vector2dc { x, y }
    }

    /// `x()`.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// `y()`.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// `sub(double x, double y)`: `this.x = this.x - x; this.y = this.y - y; return this`.
    pub fn sub(&mut self, x: f64, y: f64) -> &mut Vector2dc {
        self.x -= x;
        self.y -= y;
        self
    }

    /// `div(double scalar)` (joml 1.10.8 `div(D, Vector2d)`): `inv = 1.0 / scalar; x * inv, y * inv`.
    pub fn div(&mut self, scalar: f64) -> &mut Vector2dc {
        let inv = 1.0 / scalar;
        self.x *= inv;
        self.y *= inv;
        self
    }

    /// `mul(double scalar)`.
    pub fn mul(&mut self, scalar: f64) -> &mut Vector2dc {
        self.x *= scalar;
        self.y *= scalar;
        self
    }

    /// `add(double x, double y)`.
    pub fn add(&mut self, x: f64, y: f64) -> &mut Vector2dc {
        self.x += x;
        self.y += y;
        self
    }

    /// `distanceSquared(double x, double y)`: `dx * dx + dy * dy`.
    pub fn distance_squared(&self, x: f64, y: f64) -> f64 {
        let dx = self.x - x;
        let dy = self.y - y;
        dx * dx + dy * dy
    }
}
/// TODO(fabric2seer): java org.apache.commons.math3.geometry.euclidean.threed.Vector3D
pub struct Vector3D;
/// TODO(fabric2seer): java Visibility
pub struct Visibility;
/// TODO(fabric2seer): java java.lang.ref.WeakReference
pub struct WeakReference;
/// TODO(fabric2seer): java java.net.http.WebSocket
pub struct WebSocket;
/// TODO(fabric2seer): java java.net.http.WebSocketHandshakeException
pub struct WebSocketHandshakeException;
/// TODO(fabric2seer): java me.shedaniel.rei.api.client.gui.widgets.Widget
pub struct Widget;
/// TODO(fabric2seer): java dev.emi.emi.api.widget.WidgetHolder
pub struct WidgetHolder;
/// TODO(fabric2seer): java com.mojang.blaze3d.platform.Window
pub struct Window;
/// TODO(fabric2seer): java dev.isxander.yacl3.gui.YACLScreen
pub struct YACLScreen;
/// TODO(fabric2seer): java java.util.zip.ZipEntry
pub struct ZipEntry;
/// TODO(fabric2seer): java java.util.zip.ZipInputStream
pub struct ZipInputStream;
