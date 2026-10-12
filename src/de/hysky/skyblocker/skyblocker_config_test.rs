//! Ported from de.hysky.skyblocker.SkyblockerConfigTest by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @SuppressWarnings("UnstableApiUsage") public class SkyblockerConfigTest implements FabricClientGameTest
#[derive(Clone)]
pub struct SkyblockerConfigTest {}

impl SkyblockerConfigTest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public void runTest(ClientGameTestContext clientGameTestContext)
    pub fn run_test(&self, client_game_test_context: &crate::unresolved::ClientGameTestContext) {
        // TODO(fabric2seer): `.computeOnClient(..)` on a ClientGameTestContext
        //   java: Screen currentScreen = clientGameTestContext.computeOnClient(client -> client.gui.screen());
        let mut current_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen = todo!("fabric2seer: the initialiser of `currentScreen` (TODO above)");
        // TODO(fabric2seer): `.runOnClient(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.runOnClient(client -> client.gui.setScreen(SkyblockerConfigManager.createGUI(client.gui.screen())));
        // TODO(fabric2seer): `.waitTicks(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.waitTicks(20);
        // TODO(fabric2seer): `.assertScreenshotEquals(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.assertScreenshotEquals(TestScreenshotComparisonOptions.of("skyblocker_config").saveWithFileName("skyblocker_config"));
        // TODO(fabric2seer): `.runOnClient(..)` on a ClientGameTestContext
        //   java: clientGameTestContext.runOnClient(client -> client.gui.setScreen(currentScreen));
    }

}
