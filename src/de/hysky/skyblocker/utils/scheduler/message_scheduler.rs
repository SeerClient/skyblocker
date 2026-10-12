//! Ported from de.hysky.skyblocker.utils.scheduler.MessageScheduler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase;

/// java: private static final int MIN_DELAY = 200;
pub const MIN_DELAY: i32 = 200;
/// java: public static final MessageScheduler INSTANCE = new MessageScheduler();
pub static INSTANCE: compat::MainThread<compat::Shared<MessageScheduler>> = compat::MainThread::new(|| MessageScheduler::new());

/// java: public class MessageScheduler extends Scheduler
pub struct MessageScheduler {
    /// java: extends Scheduler
    pub base: SchedulerBase,
    /// java: private long lastMessage = 0;
    pub last_message: i64,
}

impl std::ops::Deref for MessageScheduler {
    type Target = SchedulerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MessageScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MessageScheduler {
    /// java: protected MessageScheduler()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SchedulerBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                last_message: ((0) as i64) as i64,
            });
            ((__h.clone() as compat::Shared<dyn Scheduler>), __h)
        });
        __this
    }

    /// java: public void sendMessageAfterCooldown(String message, boolean hide)
    pub fn send_message_after_cooldown(__this: &compat::Shared<Self>, message: &str, mut hide: bool) {
        if (compat::read(__this, |__t| __t.last_message)).wrapping_add(MIN_DELAY as i64) < compat::millis() {
            MessageScheduler::send_message(__this, message.to_string(), hide);
            { let __v = compat::millis(); compat::write(__this, |__t| __t.last_message = __v); }
        } else {
            MessageScheduler::queue_message(__this, message, hide, 0);
        }
    }

    /// java: private void sendMessage(String message, boolean hide)
    pub fn send_message(__this: &compat::Shared<Self>, mut message: String, mut hide: bool) {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            logger().error(&format!("[Skyblocker Message Scheduler] Tried to send a message while player is null: {}", message));
            return;
        }
        // java: message = StringUtil.trimChatMessage(StringUtils.normalizeSpace(message.trim()));
        message = mcrust::net::minecraft::util::string_util::trim_chat_message(&normalize_space(&compat::trim(&message)));
        if !hide {
            client.gui.hud.get_chat().add_recent_chat(&message);
        }
        if message.starts_with("/") {
            // java: client.player.connection.sendCommand(message.substring(1)); - the player's connection is the client's
            mcrust::net::minecraft::client::minecraft::get_connection(client).expect("client.player.connection").send_command(&compat::substring(&message, 1));
        } else {
            mcrust::net::minecraft::client::minecraft::get_connection(client).expect("client.player.connection").send_chat(&message);
        }
    }

    /// java: public void queueMessage(String message, boolean hide, int delay)
    pub fn queue_message(__this: &compat::Shared<Self>, message: &str, mut hide: bool, mut delay: i32) {
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let message: String = message.to_string();
        ({ let __f = compat::dispatch(__this, |__t| __t.schedule()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Scheduler>), Arc::new(move || { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { MessageScheduler::send_message((&__this_1), message.clone(), hide); } }), delay) });
    }

}

impl Scheduler for MessageScheduler {
    fn scheduler_base(&self) -> &SchedulerBase {
        &self.base
    }
    fn scheduler_base_mut(&mut self) -> &mut SchedulerBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected boolean runTask(Runnable task) (dispatch: the body for this class)
    fn run_task(&self) -> fn(&compat::Shared<dyn Scheduler>, Arc<dyn Fn()>) -> bool {
        MessageScheduler::run_task_dyn
    }

}

impl MessageScheduler {
    /// java: @Override protected boolean runTask(Runnable task)
    pub fn run_task_impl(__this: &compat::Shared<Self>, task: Arc<dyn Fn()>) -> bool {
        if (compat::read(__this, |__t| __t.last_message)).wrapping_add(MIN_DELAY as i64) < compat::millis() {
            (task)();
            { let __v = compat::millis(); compat::write(__this, |__t| __t.last_message = __v); }
            return true;
        }
        return false;
    }

    /// MessageScheduler.runTask as the dispatch of Scheduler.runTask
    pub fn run_task_dyn(__this: &compat::Shared<dyn Scheduler>, __p0: Arc<dyn Fn()>) -> bool {
        Self::run_task_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// `org.apache.commons.lang3.StringUtils.normalizeSpace(str)`: trims, and every run of whitespace
/// (`Character.isWhitespace`, which excludes the no-break spaces) becomes one ' '.
fn normalize_space(s: &str) -> String {
    let is_java_whitespace = |c: char| c.is_whitespace() && c != '\u{00A0}' && c != '\u{2007}' && c != '\u{202F}';
    let mut out = String::with_capacity(s.len());
    let mut pending_space = false;
    for c in s.chars() {
        if is_java_whitespace(c) {
            pending_space = !out.is_empty();
        } else {
            if pending_space {
                out.push(' ');
                pending_space = false;
            }
            out.push(c);
        }
    }
    out
}
