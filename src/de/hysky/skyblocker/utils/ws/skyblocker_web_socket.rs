//! Ported from de.hysky.skyblocker.utils.ws.SkyblockerWebSocket by fabric2seer.
//!
//! Java's `java.net.http.HttpClient` / `WebSocket` are a JDK client this crate does not have; the socket is the
//! `tungstenite` crate's RFC 6455 client over a `std::net::TcpStream` with `rustls` (webpki roots) for `wss://`. Java's
//! `WebSocket` is asynchronous (a listener called on the client's executor, `sendText` answering a future); the port's
//! `WebSocket` below is the same object over one I/O thread per connection: the thread owns the stream, writes what
//! `sendText` / `sendClose` queue, and reads frames into the `SocketListener` exactly where Java's client calls it.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

/// java: private static final String WS_URL = "wss://ws.hysky.de";
pub const WS_URL: &str = "wss://ws.hysky.de";

/// java: private static final HttpClient HTTP_CLIENT = HttpClient.newBuilder()
///   .connectTimeout(Duration.ofSeconds(10)).followRedirects(Redirect.NORMAL).executor(SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)
///   .version(Version.HTTP_2).build();
///
/// The client's one setting a WebSocket handshake reads is the connect timeout (a WebSocket upgrade is HTTP/1.1 and
/// never redirected, in Java's client as here); the executor is `compat::executor` (a thread per task).
pub static HTTP_CLIENT: LazyLock<HttpClient> = LazyLock::new(|| HttpClient { connect_timeout: Duration::from_secs(10) });

/// java: private static final ExecutorService MESSAGE_SEND_QUEUE = Executors.newSingleThreadExecutor(Thread.ofVirtual()
///   .name("Skyblocker WebSocket Message Send Queue").factory());
pub static MESSAGE_SEND_QUEUE: LazyLock<ExecutorService> = LazyLock::new(|| ExecutorService::new_single_thread_executor("Skyblocker WebSocket Message Send Queue"));

/// java: private static volatile @Nullable WebSocket socket;
/// (written by the connecting worker and the socket's listener, read by the render thread: a lock, not a
/// `MainThread` cell)
pub static SOCKET: LazyLock<Mutex<Option<WebSocket>>> = LazyLock::new(|| Mutex::new(None));

/// How long the I/O thread waits in a read before it looks at the send queue again.
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// `java.net.http.HttpClient` - the part of it a WebSocket uses.
pub struct HttpClient {
    /// `connectTimeout(Duration.ofSeconds(10))`
    pub connect_timeout: Duration,
}

impl HttpClient {
    /// java: HTTP_CLIENT.newWebSocketBuilder()
    pub fn new_web_socket_builder(&self) -> WebSocketBuilder {
        WebSocketBuilder { connect_timeout: self.connect_timeout, headers: Vec::new() }
    }
}

/// `java.net.http.WebSocket.Builder`
pub struct WebSocketBuilder {
    connect_timeout: Duration,
    headers: Vec<(String, String)>,
}

/// Why `buildAsync(..).get()` threw: Java's `ExecutionException` whose cause is a `WebSocketHandshakeException`
/// (the server answered the upgrade with an HTTP response) or another `IOException`.
pub enum WebSocketBuildError {
    /// `WebSocketHandshakeException` - `getResponse()`
    Handshake(WebSocketHandshakeException),
    /// any other cause
    Other(String),
}

/// `java.net.http.WebSocketHandshakeException`
pub struct WebSocketHandshakeException {
    response: HttpResponse,
}

impl WebSocketHandshakeException {
    /// java: wsHandshakeException.getResponse()
    pub fn get_response(&self) -> &HttpResponse {
        &self.response
    }
}

/// `java.net.http.HttpResponse<?>` of a refused upgrade
pub struct HttpResponse {
    status_code: i32,
    body: String,
}

impl HttpResponse {
    /// java: response.statusCode()
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    /// java: response.body().toString()
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl WebSocketBuilder {
    /// java: .header(name, value)
    pub fn header(mut self, name: &str, value: &str) -> WebSocketBuilder {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    /// java: .buildAsync(URI.create(uri), listener).get() - connects, upgrades, and starts the listener's thread.
    pub fn build_async_get(self, uri: &str, listener: SocketListener) -> Result<WebSocket, WebSocketBuildError> {
        use tungstenite::client::IntoClientRequest as _;
        let mut request = uri.into_client_request().map_err(|e| WebSocketBuildError::Other(e.to_string()))?;
        for (name, value) in &self.headers {
            let name = tungstenite::http::header::HeaderName::from_bytes(name.as_bytes()).map_err(|e| WebSocketBuildError::Other(e.to_string()))?;
            let value = tungstenite::http::header::HeaderValue::from_str(value).map_err(|e| WebSocketBuildError::Other(e.to_string()))?;
            request.headers_mut().append(name, value);
        }
        let host = request.uri().host().map(|h| h.to_string()).ok_or_else(|| WebSocketBuildError::Other("no host in the WebSocket URI".to_string()))?;
        let port = request.uri().port_u16().unwrap_or(if request.uri().scheme_str() == Some("wss") { 443 } else { 80 });
        // `connectTimeout`: the first address that answers within it
        use std::net::ToSocketAddrs as _;
        let addresses = (host.as_str(), port).to_socket_addrs().map_err(|e| WebSocketBuildError::Other(e.to_string()))?;
        let mut last_error = String::from("no address for the WebSocket host");
        let mut stream = None;
        for address in addresses {
            match std::net::TcpStream::connect_timeout(&address, self.connect_timeout) {
                Ok(s) => {
                    stream = Some(s);
                    break;
                }
                Err(e) => last_error = e.to_string(),
            }
        }
        let stream = stream.ok_or(WebSocketBuildError::Other(last_error))?;
        // the handshake itself is bounded by the same timeout
        let _ = stream.set_read_timeout(Some(self.connect_timeout));
        let _ = stream.set_write_timeout(Some(self.connect_timeout));
        let (mut socket, _response) = match tungstenite::client_tls(request, stream) {
            Ok(pair) => pair,
            Err(tungstenite::HandshakeError::Failure(tungstenite::Error::Http(response))) => {
                let status_code = response.status().as_u16() as i32;
                let body = response.body().as_ref().map(|b| String::from_utf8_lossy(b).into_owned()).unwrap_or_default();
                return Err(WebSocketBuildError::Handshake(WebSocketHandshakeException { response: HttpResponse { status_code, body } }));
            }
            Err(e) => return Err(WebSocketBuildError::Other(e.to_string())),
        };
        // from here the I/O thread polls: a short read timeout, no write timeout
        {
            let tcp: &std::net::TcpStream = match socket.get_ref() {
                tungstenite::stream::MaybeTlsStream::Plain(s) => s,
                tungstenite::stream::MaybeTlsStream::Rustls(s) => &s.sock,
                _ => return Err(WebSocketBuildError::Other("unsupported WebSocket stream".to_string())),
            };
            let _ = tcp.set_read_timeout(Some(POLL_INTERVAL));
            let _ = tcp.set_write_timeout(None);
        }
        let (sender, receiver) = mpsc::channel::<Outgoing>();
        let input_closed = Arc::new(AtomicBool::new(false));
        let output_closed = Arc::new(AtomicBool::new(false));
        let web_socket = WebSocket { sender, input_closed: input_closed.clone(), output_closed: output_closed.clone() };
        let handle = web_socket.clone();
        std::thread::Builder::new()
            .name("Skyblocker WebSocket".to_string())
            .spawn(move || run_socket(&mut socket, receiver, listener, handle, &input_closed, &output_closed))
            .map_err(|e| WebSocketBuildError::Other(e.to_string()))?;
        Ok(web_socket)
    }
}

/// What `sendText` / `sendClose` hand the I/O thread; the reply is the completed (or failed) send future.
enum Outgoing {
    Text(String, mpsc::Sender<Result<(), String>>),
    Close(u16, String, mpsc::Sender<Result<(), String>>),
}

/// `java.net.http.WebSocket` - a handle on one connection (cloning it is another reference to the same socket).
#[derive(Clone)]
pub struct WebSocket {
    sender: mpsc::Sender<Outgoing>,
    input_closed: Arc<AtomicBool>,
    output_closed: Arc<AtomicBool>,
}

impl WebSocket {
    /// java: WebSocket.NORMAL_CLOSURE
    pub const NORMAL_CLOSURE: i32 = 1000;

    /// java: socket.isInputClosed()
    pub fn is_input_closed(&self) -> bool {
        self.input_closed.load(Ordering::Acquire)
    }

    /// java: socket.isOutputClosed()
    pub fn is_output_closed(&self) -> bool {
        self.output_closed.load(Ordering::Acquire)
    }

    /// java: socket.sendText(message, true).join() - `Err` is the future's exception.
    pub fn send_text_join(&self, message: &str) -> Result<(), String> {
        let (reply, result) = mpsc::channel();
        self.sender.send(Outgoing::Text(message.to_string(), reply)).map_err(|_| "Output closed".to_string())?;
        result.recv().unwrap_or_else(|_| Err("Output closed".to_string()))
    }

    /// java: socket.sendClose(statusCode, reason) (the future is not waited on)
    pub fn send_close(&self, status_code: i32, reason: &str) {
        let (reply, _result) = mpsc::channel();
        let _ = self.sender.send(Outgoing::Close(status_code as u16, reason.to_string(), reply));
    }

    /// The same connection (Java compares the `WebSocket` references).
    fn same(&self, other: &WebSocket) -> bool {
        Arc::ptr_eq(&self.input_closed, &other.input_closed)
    }
}

/// The connection's I/O thread: Java's client delivering frames to the `Listener` and writing queued sends.
fn run_socket(
    socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
    receiver: mpsc::Receiver<Outgoing>,
    mut listener: SocketListener,
    handle: WebSocket,
    input_closed: &AtomicBool,
    output_closed: &AtomicBool,
) {
    loop {
        // the send queue
        loop {
            match receiver.try_recv() {
                Ok(Outgoing::Text(text, reply)) => {
                    let result = if output_closed.load(Ordering::Acquire) {
                        Err("Output closed".to_string())
                    } else {
                        socket.send(tungstenite::Message::text(text)).map_err(|e| e.to_string())
                    };
                    let _ = reply.send(result);
                }
                Ok(Outgoing::Close(code, reason, reply)) => {
                    let result = socket
                        .close(Some(tungstenite::protocol::CloseFrame { code: code.into(), reason: reason.into() }))
                        .and_then(|_| socket.flush())
                        .map_err(|e| e.to_string());
                    output_closed.store(true, Ordering::Release);
                    let _ = reply.send(result);
                }
                Err(_) => break,
            }
        }
        match socket.read() {
            Ok(tungstenite::Message::Text(text)) => {
                // tungstenite hands a whole message: Java's last fragment
                listener.on_text(&handle, text.as_str().to_string(), true);
            }
            Ok(tungstenite::Message::Ping(payload)) => {
                // the pong is queued by tungstenite and written by the next flush (Java's client answers it too)
                listener.on_ping(&handle, &payload);
                let _ = socket.flush();
            }
            Ok(tungstenite::Message::Close(frame)) => {
                let (code, reason) = match frame {
                    Some(frame) => (u16::from(frame.code) as i32, frame.reason.as_str().to_string()),
                    // RFC 6455 7.1.5: no status code - Java reports 1005
                    None => (1005, String::new()),
                };
                input_closed.store(true, Ordering::Release);
                listener.on_close(&handle, code, &reason);
            }
            Ok(_) => {}
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                let _ = socket.flush();
            }
            Err(tungstenite::Error::ConnectionClosed) | Err(tungstenite::Error::AlreadyClosed) => {
                input_closed.store(true, Ordering::Release);
                output_closed.store(true, Ordering::Release);
                return;
            }
            Err(e) => {
                input_closed.store(true, Ordering::Release);
                output_closed.store(true, Ordering::Release);
                listener.on_error(&handle, &e.to_string());
                return;
            }
        }
    }
}

/// `Executors.newSingleThreadExecutor(..)`: one named thread running the submitted tasks in order.
pub struct ExecutorService {
    sender: Mutex<mpsc::Sender<Box<dyn FnOnce() + Send>>>,
}

impl ExecutorService {
    fn new_single_thread_executor(name: &str) -> ExecutorService {
        let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
        let _ = std::thread::Builder::new().name(name.to_string()).spawn(move || {
            while let Ok(task) = receiver.recv() {
                // an exception in a task does not stop the executor
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
            }
        });
        ExecutorService { sender: Mutex::new(sender) }
    }

    /// java: MESSAGE_SEND_QUEUE.submit(task)
    pub fn submit(&self, task: impl FnOnce() + Send + 'static) {
        let sender = compat::lock(&self.sender).clone();
        let _ = sender.send(Box::new(task));
    }
}

/// The live socket, read without holding the lock across a call.
fn socket() -> Option<WebSocket> {
    compat::lock(&SOCKET).clone()
}

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
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> closeSocket()); - HEAD of `Minecraft#stop`
        report(hook::inject("net.minecraft.client.Minecraft#stop", At::Head, |_: &mut CallbackInfo<'_>| {
            SkyblockerWebSocket::close_socket();
        }));
    }

    /// java: private static CompletableFuture<Void> setupSocket()
    pub fn setup_socket() -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // (`ApiAuthentication.getToken()` is read on the worker, as Java reads it inside the task: its state is a lock)
        return CompletableFuture::<()>::run_async(move || {
            // java: try { socket = HTTP_CLIENT.newWebSocketBuilder()
            //   .header("Authorization", "Bearer " + Objects.requireNonNull(ApiAuthentication.getToken(), "Token cannot be null"))
            //   .header("User-Agent", Http.USER_AGENT).buildAsync(URI.create(WS_URL), new SocketListener()).get();
            let result = match crate::de::hysky::skyblocker::utils::api_authentication::ApiAuthentication::get_token() {
                None => Err(WebSocketBuildError::Other("java.lang.NullPointerException: Token cannot be null".to_string())),
                Some(token) => HTTP_CLIENT
                    .new_web_socket_builder()
                    .header("Authorization", &format!("Bearer {}", token))
                    .header("User-Agent", crate::de::hysky::skyblocker::utils::http::USER_AGENT.as_str())
                    .build_async_get(WS_URL, SocketListener::new()),
            };
            match result {
                Ok(web_socket) => {
                    *compat::lock(&SOCKET) = Some(web_socket);
                    logger().info("[Skyblocker WebSocket] Successfully connected to the Skyblocker WebSocket!");
                }
                // java: if (e.getCause() instanceof WebSocketHandshakeException wsHandshakeException)
                Err(WebSocketBuildError::Handshake(ws_handshake_exception)) => {
                    // java: HttpResponse<?> response = wsHandshakeException.getResponse();
                    let response: &HttpResponse = ws_handshake_exception.get_response();
                    // java: String body = response.body().toString();
                    let body: String = response.body().to_string();
                    // java: LOGGER.error("[Skyblocker WebSocket] Failed to setup WebSocket connection! HTTP {}: {}", response.statusCode(), body, wsHandshakeException);
                    logger().error(&format!("[Skyblocker WebSocket] Failed to setup WebSocket connection! HTTP {}: {} java.net.http.WebSocketHandshakeException", response.status_code(), body));
                }
                Err(WebSocketBuildError::Other(e)) => {
                    logger().error(&format!("[Skyblocker WebSocket] Failed to setup WebSocket connection! {}", e));
                }
            }
        }, compat::executor);
    }

    /// java: private static void closeSocket()
    pub fn close_socket() {
        if SkyblockerWebSocket::is_connection_open() {
            // java: MESSAGE_SEND_QUEUE.submit(() -> { socket.sendClose(WebSocket.NORMAL_CLOSURE, "Minecraft closing"); });
            MESSAGE_SEND_QUEUE.submit(|| {
                if let Some(socket) = socket() {
                    socket.send_close(WebSocket::NORMAL_CLOSURE, "Minecraft closing");
                }
            });
        }
    }

    /// java: private static boolean isConnectionOpen()
    pub fn is_connection_open() -> bool {
        //   java: return socket != null && !socket.isInputClosed() && !socket.isOutputClosed();
        match socket() {
            None => false,
            Some(socket) => !socket.is_input_closed() && !socket.is_output_closed(),
        }
    }

    /// java: static void send(String message)
    pub fn send(message: &str) {
        if SkyblockerWebSocket::is_connection_open() {
            SkyblockerWebSocket::send_internal(message);
        } else {
            //   java: setupSocket().thenRun(() -> sendInternal(message));
            // (the continuation runs on the connecting worker: the debug flags, render-thread config reads, are
            // taken here)
            let message: String = message.to_string();
            let debug = SkyblockerWebSocket::send_debug();
            SkyblockerWebSocket::setup_socket().then_run(move || SkyblockerWebSocket::submit_send(message, debug));
        }
    }

    /// java: private static void sendInternal(String message)
    pub fn send_internal(message: &str) {
        SkyblockerWebSocket::submit_send(message.to_string(), SkyblockerWebSocket::send_debug());
    }

    /// `Debug.debugEnabled() && Debug.webSocketDebug()` - config reads, taken on the render thread.
    fn send_debug() -> bool {
        crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug()
    }

    /// `sendInternal`'s task, with the debug flag read by the caller.
    fn submit_send(message: String, debug: bool) {
        // java: MESSAGE_SEND_QUEUE.submit(() -> { try { if (Debug..) LOGGER.info(..); socket.sendText(message, true).join(); }
        //   catch (Exception e) { LOGGER.error("[Skyblocker WebSocket] Failed to send message!", e); } });
        MESSAGE_SEND_QUEUE.submit(move || {
            if debug {
                logger().info(&format!("[Skyblocker WebSocket] Sending Message: {}", message));
            }
            let result = match socket() {
                // `socket.sendText` on a null socket: the NullPointerException the catch logs
                None => Err("java.lang.NullPointerException".to_string()),
                Some(socket) => socket.send_text_join(&message),
            };
            if let Err(e) = result {
                logger().error(&format!("[Skyblocker WebSocket] Failed to send message! {}", e));
            }
        });
    }

}

/// java: private static class SocketListener implements WebSocket.Listener
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
    pub fn on_text(&mut self, web_socket: &WebSocket, mut message: String, mut last: bool) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Object>> {
        self.parts.push(message.clone());
        // java: webSocket.request(1); - the I/O thread reads the next frame as soon as this returns
        let _ = web_socket;
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
    pub fn on_ping(&self, web_socket: &WebSocket, message: &[u8]) {
        // (the debug flags read the config, a render-thread cell: the check and its log line run there)
        compat::execute(|| {
            if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug() {
                logger().info("[Skyblocker WebSocket] Received ping");
            }
        });
        // java: return WebSocket.Listener.super.onPing(webSocket, message); - the default body is `webSocket.request(1);
        // return null;` (the client itself answers the ping with a pong)
        let _ = (web_socket, message);
    }

    /// java: @Override public CompletionStage<?> onClose(WebSocket webSocket, int statusCode, String reason)
    pub fn on_close(&self, web_socket: &WebSocket, mut status_code: i32, reason: &str) {
        logger().info(&format!("[Skyblocker WebSocket] Connection closing. Status Code: {}, Reason: {}", status_code, reason));
        // java: socket = null;
        {
            let mut socket = compat::lock(&SOCKET);
            // (only this connection: a newer socket set up meanwhile stays)
            if socket.as_ref().is_some_and(|s| s.same(web_socket)) {
                *socket = None;
            }
        }
        // java: return WebSocket.Listener.super.onClose(webSocket, statusCode, reason); - the default body is `return null;`
    }

    /// java: @Override public void onError(WebSocket webSocket, Throwable error)
    pub fn on_error(&self, web_socket: &WebSocket, error: &str) {
        let _ = web_socket;
        logger().error(&format!("[Skyblocker WebSocket] Encountered an error and closed the connection! {}", error));
    }

    /// java: private void handleWholeMessage(List<CharSequence> parts)
    pub fn handle_whole_message(&self, parts: &Vec<String>) {
        let mut message: String = parts.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
        // `WsMessageHandler.handleMessage` reads the payload codec and the services' render-thread state (the mod's
        // `MainThread` cells, which panic off that thread): the whole message is handed to the client thread.
        compat::execute(move || {
            if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug() {
                logger().info(&format!("[Skyblocker WebSocket] Received Message: {}", message));
            }
            WsMessageHandler::handle_message(&message);
        });
    }

}
