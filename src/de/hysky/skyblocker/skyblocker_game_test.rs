//! Ported from de.hysky.skyblocker.SkyblockerGameTest by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @SuppressWarnings("UnstableApiUsage") public class SkyblockerGameTest implements FabricClientGameTest
#[derive(Clone)]
pub struct SkyblockerGameTest {}

impl SkyblockerGameTest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public void runTest(ClientGameTestContext context)
    pub fn run_test(&self, context: &crate::unresolved::ClientGameTestContext) {
        {
            // TODO(fabric2seer): `.worldBuilder(..)` on a ClientGameTestContext
            //   java: TestSingleplayerContext singleplayer = context.worldBuilder(..).adjustSettings(..).create(..);
            let mut singleplayer: crate::unresolved::TestSingleplayerContext = todo!("fabric2seer: the initialiser of `singleplayer` (TODO above)");
            // TODO(fabric2seer): `.getServer(..)` on a TestSingleplayerContext
            //   java: singleplayer.getServer().runCommand("/fill 180 63 -13 184 67 -17 air");
            // TODO(fabric2seer): `.getServer(..)` on a TestSingleplayerContext
            //   java: singleplayer.getServer().runCommand("/setblock 175 66 -4 minecraft:barrier");
            // TODO(fabric2seer): `.getServer(..)` on a TestSingleplayerContext
            //   java: singleplayer.getServer().runCommand("/tp @a 175 67 -4");
            // TODO(fabric2seer): `.runOnClient(..)` on a ClientGameTestContext
            //   java: context.runOnClient(client -> {
            //   java: 				assert client.player != null;
            //   java: 				client.player.setYRot(180);
            //   java: 				client.player.setXRot(20);
            //   java: 			});
            // TODO(fabric2seer): `.computeOnClient(..)` on a ClientGameTestContext
            //   java: var config = context.computeOnClient(_ -> {
            //   java: 				var curConfig = FancyStatusBars.INSTANCE.statusBars.entrySet().stream().map(e -> Pair.of(e.getKey(), e.getValue().toJson())).toList();
            //   java:
            //   java: 				int[] counts = new int[7];
            //   java: 				FancyStatusBars.INSTANCE.statusBars.forEach((type, bar) -> {
            //   java: 					bar.anchor = type.getDefaultAnchor();
            //   java: 					bar.gridY = type.getDefaultGridY();
            //   java: 					bar.gridX = counts[type.getDefaultAnchor().ordinal()]++;
            //   java: 				});
            //   java: 				FancyStatusBars.INSTANCE.placeBarsInPositioner();
            //   java: 				FancyStatusBars.INSTANCE.updatePositions(false);
            //   java: 				return curConfig;
            //   java: 			});
            // TODO(fabric2seer): `.getConnection(..)` on a TestSingleplayerContext
            //   java: singleplayer.getConnection().waitForChunksRender();
            // TODO(fabric2seer): `.assertScreenshotEquals(..)` on a ClientGameTestContext
            //   java: context.assertScreenshotEquals(TestScreenshotComparisonOptions.of("skyblocker_render").saveWithFileName("skyblocker_render"));
            // TODO(fabric2seer): `.runOnClient(..)` on a ClientGameTestContext
            //   java: context.runOnClient(_ -> {
            //   java: 				config.forEach(pair -> FancyStatusBars.INSTANCE.statusBars.get(pair.key()).loadFromJson(pair.value()));
            //   java: 				FancyStatusBars.INSTANCE.placeBarsInPositioner();
            //   java: 				FancyStatusBars.INSTANCE.updatePositions(false);
            //   java: 			});
        }
    }

}
