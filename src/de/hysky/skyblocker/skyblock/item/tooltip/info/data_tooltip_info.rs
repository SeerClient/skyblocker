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
    pub contains: Arc<dyn Fn(&T, &str) -> bool>,
    /// java: private final Predicate<GeneralConfig.ItemTooltip> dataEnabled;
    pub data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>,
    /// java: private final @Nullable Consumer<T>[] callbacks;
    pub callbacks: Option<Vec<Arc<dyn Fn(&T)>>>,
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

impl<T: Clone + Send + 'static> DataTooltipInfo<T> {
    /// java: @SafeVarargs protected DataTooltipInfo(String address, Codec<T> codec, boolean cacheable, BiPredicate<T, String> contains, Predicate<GeneralConfig.ItemTooltip> tooltipEnabled, Predicate<GeneralConfig.
    pub fn new(mut address: String, mut codec: CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(&T, &str) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, mut callbacks: Vec<Arc<dyn Fn(&T)>>) -> compat::Shared<Self> {
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
                download: CompletableFuture::completed_future(()),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn SimpleTooltipInfo>), __h)
        });
        __this
    }

}

impl<T: Clone + Send + 'static> SimpleTooltipInfo for DataTooltipInfo<T> {
    fn simple_tooltip_info_base(&self) -> &SimpleTooltipInfoBase {
        &self.base
    }
    fn simple_tooltip_info_base_mut(&mut self) -> &mut SimpleTooltipInfoBase {
        &mut self.base
    }

}

impl<T: Clone + Send + 'static> TooltipInfoType for DataTooltipInfo<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isTooltipEnabled() (dispatch: the body for this class)
    fn is_tooltip_enabled(&self) -> fn(&compat::Shared<dyn TooltipInfoType>) -> bool {
        <dyn SimpleTooltipInfo>::is_tooltip_enabled_from::<Self>
    }

}

impl<T: Clone + Send + 'static> DataTooltipInfoType<T> for DataTooltipInfo<T> {
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
            return (self.contains)(self.data.as_ref().unwrap(), member_name);
        }
    }

    /// java: @Override public synchronized CompletableFuture<Void> download()
    /// Starts a download unless one is already in progress; the answer mirrors the active download (`copy()`).
    fn download(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if self.download.is_done() {
            // download = DataTooltipInfoType.super.download(); - CompletableFuture.runAsync(this, VIRTUAL_THREAD_EXECUTOR)
            self.download = self.run_async();
        }
        // return download.copy();
        let copy = CompletableFuture::<()>::new();
        let target = copy.clone();
        self.download.when_complete(Box::new(move |outcome| match outcome {
            Ok(_) => {
                target.complete(());
            }
            Err(exception) => {
                target.complete_exceptionally(exception.to_string());
            }
        }));
        return copy;
    }

    /// java: @Override public void run()
    /// `run()` on the calling thread (the body of `runAsync(this, ..)` without the hop).
    fn run(&mut self) {
        let fetched = DataTooltipInfo::<T>::fetch(&self.address, self.cacheable, self.hash, &self.codec);
        self.apply(fetched);
    }

}

/// What `run()` read on its worker: the new cache hash (`cacheable` addresses), and the data - `None` when `run()`
/// returned early (the hash was unchanged, or the response was HTML).
pub struct Fetched<T> {
    pub hash: Option<i64>,
    pub data: Option<T>,
}

impl<T: Clone + Send + 'static> DataTooltipInfo<T> {
    /// `CompletableFuture.runAsync(this, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)`: the network half of `run()` on a
    /// worker; what it writes into this object (`hash`, `data`, the callbacks) runs through the game's executor -
    /// the object is the mod's render-thread object - and the future completes after it, as `run()` would.
    fn run_async(&self) -> CompletableFuture<()> {
        let future = CompletableFuture::<()>::new();
        let done = future.clone();
        let (address, cacheable, hash, codec) = (self.address.clone(), self.cacheable, self.hash, self.codec.clone());
        let handle = compat::RenderThread::new(self.self_ref.clone());
        compat::executor(Box::new(move || {
            let fetched = DataTooltipInfo::<T>::fetch(&address, cacheable, hash, &codec);
            mcrust::mcrust::modding::game::execute(move || {
                if let Some(this) = handle.get().upgrade() {
                    compat::write(&this, |t| t.apply(fetched));
                }
                done.complete(());
            });
        }));
        future
    }

    /// The body of `run()` up to the data (the `try` block's network half): `Http.sendHeadRequest` / the
    /// ETag + Last-Modified hash for a cacheable address, `Http.sendGetRequest`, the HTML check and the codec parse.
    /// A failure is the `catch (Exception e)` arm: logged, nothing written.
    fn fetch(address: &str, cacheable: bool, previous_hash: i64, codec: &CodecRef<T>) -> Fetched<T> {
        // `this.hash = hash` happens before the GET, so a failure after it keeps the new hash, as Java's does
        let mut new_hash: Option<i64> = None;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if cacheable {
                let headers = crate::de::hysky::skyblocker::utils::http::Http::send_head_request(address);
                // long hash = Http.getEtag(headers).hashCode() + Http.getLastModified(headers).hashCode(); (an int sum)
                let hash: i64 = compat::string_hash_code(&crate::de::hysky::skyblocker::utils::http::Http::get_etag(&headers))
                    .wrapping_add(compat::string_hash_code(&crate::de::hysky::skyblocker::utils::http::Http::get_last_modified(&headers))) as i64;
                if previous_hash == hash {
                    return Fetched { hash: None, data: None };
                } else {
                    new_hash = Some(hash);
                }
            }
            let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request(address);
            if compat::trim(&response).starts_with("<!DOCTYPE") || compat::trim(&response).starts_with("<html") {
                logger().warn(&format!("[Skyblocker] Received HTML content for {}. Expected JSON.", address));
                return Fetched { hash: new_hash, data: None };
            }
            let data: T = (codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_or_throw().unwrap();
            Fetched { hash: new_hash, data: Some(data) }
        }));
        match outcome {
            Ok(fetched) => fetched,
            Err(e) => {
                logger().warn(&format!("[Skyblocker] Failed to download {} data! {}", address, compat::panic_message(&e)));
                Fetched { hash: new_hash, data: None }
            }
        }
    }

    /// The rest of `run()`: `this.hash = hash`, `data = ..`, then every callback with the data.
    fn apply(&mut self, fetched: Fetched<T>) {
        if let Some(hash) = fetched.hash {
            self.hash = hash;
        }
        if let Some(data) = fetched.data {
            self.data = Some(data);
            if self.callbacks.is_some() {
                for callback in self.callbacks.as_ref().unwrap().iter() {
                    (callback)(self.data.as_ref().unwrap());
                }
            }
        }
    }

}
