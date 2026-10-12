//! Ported from de.hysky.skyblocker.utils.ws.SkyblockerWebSocket by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;
use crate::entry::skyblocker_mod;
use mcrust::java::lang::throwable::Throwable;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: private static final String WS_URL = "wss://ws.hysky.de";
pub const WS_URL: &str = "wss://ws.hysky.de";
// TODO(fabric2seer): static `HttpClient.newBuilder` has no Rust counterpart in the port
//   java: private static final HttpClient HTTP_CLIENT = HttpClient.newBuilder()
//   java: 			.connectTimeout(Duration.ofSeconds(10))
//   java: 			.followRedirects(Redirect.NORMAL)
//   java: 			.executor(SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)
//   java: 			.version(Version.HTTP_2)
//   java: 			.build();
/// java: private static final HttpClient HTTP_CLIENT = HttpClient.newBuilder() .connectTimeout(Duration.ofSeconds(10)) .followRedirects(Redirect.NORMAL) .executor(SkyblockerMod.VIRTUAL_THREAD_EXECUTOR) .versio
pub static HTTP_CLIENT: compat::MainThread<crate::unresolved::HttpClient> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `HTTP_CLIENT` - see the TODO above"));
// TODO(fabric2seer): Executors.newSingleThreadExecutor: use std::thread / the port's executors
//   java: private static final ExecutorService MESSAGE_SEND_QUEUE = Executors.newSingleThreadExecutor(Thread.ofVirtual()
//   java: 			.name("Skyblocker WebSocket Message Send Queue")
//   java: 			.factory());
/// java: private static final ExecutorService MESSAGE_SEND_QUEUE = Executors.newSingleThreadExecutor(Thread.ofVirtual() .name("Skyblocker WebSocket Message Send Queue") .factory());
pub static MESSAGE_SEND_QUEUE: compat::MainThread<crate::unresolved::ExecutorService> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MESSAGE_SEND_QUEUE` - see the TODO above"));
/// java: private static volatile @Nullable WebSocket socket;
pub static SOCKET: compat::MainThread<Option<crate::unresolved::WebSocket>> = compat::MainThread::new(|| None);

/// java: public class SkyblockerWebSocket
#[derive(Clone)]
pub struct SkyblockerWebSocket {}

impl SkyblockerWebSocket {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::JOIN.register(Arc::new(move || {
            if !(SkyblockerWebSocket::is_connection_open()) {
                SkyblockerWebSocket::setup_socket();
            }
        }));
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> closeSocket());
    }

    /// java: private static CompletableFuture<Void> setupSocket()
    pub fn setup_socket() -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        return CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.header(..)` on a value of unknown type
                //   java: socket = HTTP_CLIENT.newWebSocketBuilder()
                //   java: 						.header("Authorization", "Bearer " + Objects.requireNonNull(ApiAuthentication.getToken(), "Token cannot be null"))
                //   java: 						.header("User-Agent", Http.USER_AGENT)
                //   java: 						.buildAsync(URI.create(WS_URL), new SocketListener())
                //   java: 						.get();
                logger().info("[Skyblocker WebSocket] Successfully connected to the Skyblocker WebSocket!");
            }));
            if let Err(e) = __result {
                // java: if (e.getCause() instanceof WebSocketHandshakeException wsHandshakeException) - the panic payload is the exception
                if let Some(ws_handshake_exception) = e.downcast_ref::<crate::unresolved::WebSocketHandshakeException>() {
                    // TODO(fabric2seer): `.getResponse(..)` on a WebSocketHandshakeException
                    //   java: HttpResponse<?> response = wsHandshakeException.getResponse();
                    let mut response: crate::unresolved::HttpResponse = todo!("fabric2seer: the initialiser of `response` (TODO above)");
                    // TODO(fabric2seer): `.body(..)` on a HttpResponse
                    //   java: String body = response.body().toString();
                    let mut body: String = todo!("fabric2seer: the initialiser of `body` (TODO above)");
                    // TODO(fabric2seer): `.statusCode(..)` on a HttpResponse
                    //   java: LOGGER.error("[Skyblocker WebSocket] Failed to setup WebSocket connection! HTTP {}: {}", response.statusCode(), body, wsHandshakeException);
                } else {
                    logger().error(&format!("[Skyblocker WebSocket] Failed to setup WebSocket connection!"));
                }
            }
        })(), compat::executor);
    }

    /// java: private static void closeSocket()
    pub fn close_socket() {
        if SkyblockerWebSocket::is_connection_open() {
            // TODO(fabric2seer): `.submit(..)` on a ExecutorService
            //   java: MESSAGE_SEND_QUEUE.submit(() -> {
            //   java: 				socket.sendClose(WebSocket.NORMAL_CLOSURE, "Minecraft closing");
            //   java: 			});
        }
    }

    /// java: private static boolean isConnectionOpen()
    pub fn is_connection_open() -> bool {
        // TODO(fabric2seer): `.isInputClosed(..)` on a WebSocket
        //   java: return socket != null && !socket.isInputClosed() && !socket.isOutputClosed();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: static void send(String message)
    pub fn send(message: &str) {
        if SkyblockerWebSocket::is_connection_open() {
            SkyblockerWebSocket::send_internal(message);
        } else {
            // TODO(fabric2seer): `CompletableFuture.thenRun` has no Rust method in the port (looked for `then_run` in CompletableFuture and JavaSrc)
            // rust: SkyblockerWebSocket::setup_socket().then_run(&Arc::new(move || SkyblockerWebSocket::send_internal(message)));
        }
    }

    /// java: private static void sendInternal(String message)
    pub fn send_internal(message: &str) {
        // TODO(fabric2seer): `.submit(..)` on a ExecutorService
        //   java: MESSAGE_SEND_QUEUE.submit(() -> {
        //   java: 			try {
        //   java: 				if (Debug.debugEnabled() && Debug.webSocketDebug()) {
        //   java: 					LOGGER.info("[Skyblocker WebSocket] Sending Message: {}", message);
        //   java: 				}
        //   java:
        //   java: 				socket.sendText(message, true).join();
        //   java: 			} catch (Exception e) {
        //   java: 				LOGGER.error("[Skyblocker WebSocket] Failed to send message!", e);
        //   java: 			}
        //   java: 		});
    }

}

/// java: private static class SocketListener implements WebSocket.Listener
#[derive(Clone)]
pub struct SocketListener {
    /// java: private List<CharSequence> parts = new ArrayList<>();
    pub parts: Vec<String>,
    /// java: private CompletableFuture<?> accumulatedMessage = new CompletableFuture<>();
    pub accumulated_message: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Object>>,
}

impl SocketListener {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            parts: Vec::new(),
            accumulated_message: CompletableFuture::new(),
        };
        this
    }

    /// java: @Override public CompletionStage<?> onText(WebSocket webSocket, CharSequence message, boolean last)
    pub fn on_text(&mut self, web_socket: &crate::unresolved::WebSocket, mut message: String, mut last: bool) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Object>> {
        self.parts.push(message.clone());
        // TODO(fabric2seer): `.request(..)` on a WebSocket
        //   java: webSocket.request(1);
        if last {
            self.handle_whole_message(&self.parts);
            self.parts = Vec::new();
            self.accumulated_message.complete(None);
            // java: CompletionStage<?> future = accumulatedMessage; accumulatedMessage = new CompletableFuture<>(); return future;
            let future = std::mem::replace(&mut self.accumulated_message, CompletableFuture::new());
            return future;
        }
        return self.accumulated_message.clone();
    }

    /// java: @Override public CompletionStage<?> onPing(WebSocket webSocket, ByteBuffer message)
    pub fn on_ping(&self, web_socket: &crate::unresolved::WebSocket, message: &crate::unresolved::ByteBuffer) -> crate::unresolved::CompletionStage {
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug() {
            logger().info("[Skyblocker WebSocket] Received ping");
        }
        // TODO(fabric2seer): static field WebSocket.Listener
        //   java: return WebSocket.Listener.super.onPing(webSocket, message);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public CompletionStage<?> onClose(WebSocket webSocket, int statusCode, String reason)
    pub fn on_close(&self, web_socket: &crate::unresolved::WebSocket, mut status_code: i32, reason: &str) -> crate::unresolved::CompletionStage {
        logger().info(&format!("[Skyblocker WebSocket] Connection closing. Status Code: {}, Reason: {}", status_code, reason));
        *SOCKET.get_mut() = None;
        // TODO(fabric2seer): static field WebSocket.Listener
        //   java: return WebSocket.Listener.super.onClose(webSocket, statusCode, reason);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void onError(WebSocket webSocket, Throwable error)
    pub fn on_error(&self, web_socket: &crate::unresolved::WebSocket, error: &Throwable) {
        logger().error(&format!("[Skyblocker WebSocket] Encountered an error and closed the connection!"));
    }

    /// java: private void handleWholeMessage(List<CharSequence> parts)
    pub fn handle_whole_message(&self, parts: &Vec<String>) {
        let mut message: String = parts.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug() {
            logger().info(&format!("[Skyblocker WebSocket] Received Message: {}", message));
        }
        WsMessageHandler::handle_message(&message);
    }

}
