//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.PingDebugMonitorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.PingDebugMonitor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyArg skyblocker$onPingResult
///   java: @ModifyArg(method = "onPongReceived", at = @At(value = "INVOKE", target = "Lnet/minecraft/util/debugchart/LocalSampleLogger;logSample(J)V"))
/// target: net.minecraft.util.debugchart.AbstractSampleLogger#logSample  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.multiplayer.PingDebugMonitor#onPongReceived
/// note: `logSample` is inherited: hooked on net.minecraft.util.debugchart.AbstractSampleLogger (it fires for every subclass - test the object's type in the handler if net.minecraft.util.debugchart.LocalSampleLogger matters)
fn skyblocker_on_ping_result(v: &i64) -> i64 {
    let mut ping = v.clone();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_crimson() {
        crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager::DojoManager::on_ping_result(ping);
    }
    crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::update_ping(ping);
    return ping;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.multiplayer.PingDebugMonitor#onPongReceived", "net.minecraft.util.debugchart.AbstractSampleLogger#logSample").priority(1000).modify_arg::<i64>(0, skyblocker_on_ping_result));
}
