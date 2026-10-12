//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.info.DataTooltipInfo by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::ItemTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::simple_tooltip_info::SimpleTooltipInfo;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::simple_tooltip_info::SimpleTooltipInfoBase;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public final class DataTooltipInfo<T> extends SimpleTooltipInfo implements DataTooltipInfoType<T>
pub struct DataTooltipInfo<T> {
    /// java: extends SimpleTooltipInfo
    pub base: SimpleTooltipInfoBase,
    /// java: private final String address;
    pub address: String,
    /// java: private final Codec<T> codec;
    pub codec: CodecRef<T>,
    /// java: private @Nullable T data;
    pub data: Option<T>,
    /// java: private final boolean cacheable;
    pub cacheable: bool,
    /// java: private long hash;
    pub hash: i64,
    /// java: private final BiPredicate<T, String> contains;
    pub contains: Arc<dyn Fn(T, String) -> bool>,
    /// java: private final Predicate<GeneralConfig.ItemTooltip> dataEnabled;
    pub data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>,
    /// java: private final @Nullable Consumer<T>[] callbacks;
    pub callbacks: Option<Vec<Arc<dyn Fn(T)>>>,
    /// java: private CompletableFuture<Void> download = CompletableFuture.completedFuture(null);
    pub download: mcrust::java::util::concurrent::completable_future::CompletableFuture<()>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: 'static> std::ops::Deref for DataTooltipInfo<T> {
    type Target = SimpleTooltipInfoBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for DataTooltipInfo<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> DataTooltipInfo<T> {
    /// java: @SafeVarargs protected DataTooltipInfo(String address, Codec<T> codec, boolean cacheable, BiPredicate<T, String> contains, Predicate<GeneralConfig.ItemTooltip> tooltipEnabled, Predicate<GeneralConfig.
    pub fn new(mut address: String, mut codec: CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(T, String) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, mut callbacks: Vec<Arc<dyn Fn(T)>>) -> compat::Shared<Self> {
        let __init_address = address.clone();
        let __init_codec = codec.clone();
        let __init_cacheable = cacheable;
        let __init_contains = contains.clone();
        let __init_data_enabled = data_enabled.clone();
        let __init_callbacks = Some(callbacks.clone());
        let (_, __this) = SimpleTooltipInfoBase::new_in(tooltip_enabled.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                address: __init_address,
                codec: __init_codec,
                data: None,
                cacheable: __init_cacheable,
                hash: 0,
                contains: __init_contains,
                data_enabled: __init_data_enabled,
                callbacks: __init_callbacks,
                download: CompletableFuture::completed_future(None),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn SimpleTooltipInfo>), __h)
        });
        __this
    }

}

impl<T: Clone + 'static> SimpleTooltipInfo for DataTooltipInfo<T> {
    fn simple_tooltip_info_base(&self) -> &SimpleTooltipInfoBase {
        &self.base
    }
    fn simple_tooltip_info_base_mut(&mut self) -> &mut SimpleTooltipInfoBase {
        &mut self.base
    }

}

impl<T: Clone + 'static> TooltipInfoType for DataTooltipInfo<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isTooltipEnabled() (dispatch: the body for this class)
    fn is_tooltip_enabled(&self) -> fn(&compat::Shared<dyn TooltipInfoType>) -> bool {
        <dyn SimpleTooltipInfo>::is_tooltip_enabled_from::<Self>
    }

}

impl<T: Clone + 'static> DataTooltipInfoType<T> for DataTooltipInfo<T> {
    /// java: @Override public boolean isDataEnabled()
    fn is_data_enabled(&self) -> bool {
        return (self.data_enabled)((item_tooltip::CONFIG.get_mut())());
    }

    /// java: @Override @Nullable public T getData()
    fn get_data(&self) -> Option<T> {
        return self.data.clone();
    }

    /// java: @Override public Codec<T> getCodec()
    fn get_codec(&self) -> CodecRef<T> {
        return self.codec.clone();
    }

    /// java: @Override public boolean hasOrNullWarning(String memberName)
    fn has_or_null_warning(&self, member_name: &str) -> bool {
        if self.data.is_none() {
            crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip::ItemTooltip::null_warning();
            return false;
        } else {
            return (self.contains)(self.data.clone(), member_name.to_string());
        }
    }

    /// java: @Override public synchronized CompletableFuture<Void> download()
    fn download(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if self.download.is_done() {
            // TODO(fabric2seer): static field DataTooltipInfoType.super
            //   java: download = DataTooltipInfoType.super.download();
        }
        // TODO(fabric2seer): `CompletableFuture.copy` has no Rust method in the port (looked for `copy` in CompletableFuture and JavaSrc)
        // rust: return self.download.copy();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void run()
    fn run(&mut self) {
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if self.cacheable {
                // TODO(fabric2seer): `Http.sendHeadRequest` is not a static method of Http
                //   java: HttpHeaders headers = Http.sendHeadRequest(address);
                let mut headers: crate::unresolved::HttpHeaders = todo!("fabric2seer: the initialiser of `headers` (TODO above)");
                // TODO(fabric2seer): `Http.getEtag` is not a static method of Http
                //   java: long hash = Http.getEtag(headers).hashCode() + Http.getLastModified(headers).hashCode();
                let mut hash: i64 = todo!("fabric2seer: the initialiser of `hash` (TODO above)");
                if self.hash == hash {
                    return;
                } else {
                    self.hash = hash;
                }
            }
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String response = Http.sendGetRequest(address);
            let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
            if compat::trim(&response).starts_with("<!DOCTYPE") || compat::trim(&response).starts_with("<html") {
                logger().warn(&format!("[Skyblocker] Received HTML content for {}. Expected JSON.", self.address));
                return;
            }
            self.data = Some((self.codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_or_throw().unwrap());
            if self.callbacks.is_some() {
                for callback in self.callbacks.as_mut().unwrap().iter_mut() {
                    (callback)(self.data.clone());
                }
            }
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			ItemTooltip.LOGGER.warn("[Skyblocker] Failed to download {} data!", this.address, e);
        //   java: 		}
    }

}
