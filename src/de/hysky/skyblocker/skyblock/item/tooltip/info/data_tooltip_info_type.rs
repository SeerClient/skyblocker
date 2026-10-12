//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.info.DataTooltipInfoType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: public interface DataTooltipInfoType<T> extends TooltipInfoType, Runnable
pub trait DataTooltipInfoType<T>: TooltipInfoType {
    /// java: boolean isDataEnabled();
    fn is_data_enabled(&self) -> bool;

    /// java: @Nullable T getData();
    fn get_data(&self) -> Option<T>;

    /// java: Codec<T> getCodec();
    fn get_codec(&self) -> CodecRef<T>;

    /// java: boolean hasOrNullWarning(String memberName);
    fn has_or_null_warning(&self, member_name: &str) -> bool;

    /// java: default CompletableFuture<Void> downloadIfEnabled()
    fn download_if_enabled(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if self.is_data_enabled() {
            return self.download();
        }
        return CompletableFuture::completed_future(());
    }

    /// java: default CompletableFuture<Void> download()
    /// The default body is `CompletableFuture.runAsync(this, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)`: it hands the
    /// object itself to a worker, so it is written in `DataTooltipInfo`, the interface's one implementation, which
    /// holds the handle (`self_ref`) that `this` is.
    fn download(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()>;

    /// java: @Override void run();
    fn run(&mut self);

}
