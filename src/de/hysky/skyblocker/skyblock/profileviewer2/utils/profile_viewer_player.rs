//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.ProfileViewerPlayer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::entity_utils;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::client::player::remote_player::RemotePlayer;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::entity::player::player_model_part::PlayerModelPart;
use mcrust::net::minecraft::world::entity::player::player_skin::PlayerSkin;

/// java: public class ProfileViewerPlayer extends RemotePlayer
pub struct ProfileViewerPlayer {
    /// java: extends net.minecraft.client.player.RemotePlayer
    pub base: RemotePlayer,
}

impl std::ops::Deref for ProfileViewerPlayer {
    type Target = RemotePlayer;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileViewerPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileViewerPlayer {
    /// java: public ProfileViewerPlayer(GameProfile gameProfile)
    pub fn new(mut game_profile: GameProfile) -> Self {
        let mut this = Self {
            // java: super(Minecraft.getInstance().level, gameProfile); - the port's constructor takes the player's
            // standing dimensions (`Player.STANDING_DIMENSIONS`) where Java reads them through the level
            base: RemotePlayer::new(mcrust::net::minecraft::world::entity::entity_dimensions::scalable(0.6, 1.8), &game_profile),
        };
        this.base.base.base.base.base.set_custom_name_visible(false);
        this.base.base.base.base.base.set_id(entity_utils::PLACEHOLDER_ID);
        this
    }

    /// java: @Override public PlayerSkin getSkin()
    pub fn get_skin(&self) -> PlayerSkin {
        let mut player_info: PlayerInfo = PlayerInfo::new(self.base.base.base.get_game_profile().clone(), false);
        return player_info.get_skin();
    }

    /// java: @Override public boolean isModelPartShown(PlayerModelPart modelPart)
    pub fn is_model_part_shown(&self, model_part: &PlayerModelPart) -> bool {
        return (*model_part != PlayerModelPart::Cape);
    }

    /// java: @Override public boolean isInvisibleTo(Player player)
    pub fn is_invisible_to(&self, player: &dyn Player) -> bool {
        return true;
    }

}

// NOTE(fabric2seer): getSkin, isInvisibleTo, isModelPartShown override(s) of RemotePlayer - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
