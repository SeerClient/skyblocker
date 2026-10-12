//! Ported from de.hysky.skyblocker.utils.Resettable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface Resettable extends ClientPlayConnectionEvents.Join
pub trait Resettable: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void reset(); (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>);

    /// java: @Override default void onPlayReady(ClientPacketListener handler, PacketSender sender, Minecraft client) (dispatch: the body for this class)
    fn on_play_ready(&self) -> fn(&compat::Shared<dyn Resettable>, &mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener, &crate::unresolved::PacketSender, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn Resettable>::on_play_ready_body
    }

}

impl dyn Resettable {
    /// java: @Override default void onPlayReady(ClientPacketListener handler, PacketSender sender, Minecraft client)
    pub fn on_play_ready_body(__this: &compat::Shared<dyn Resettable>, handler: &mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener, sender: &crate::unresolved::PacketSender, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(__this) });
    }

}
