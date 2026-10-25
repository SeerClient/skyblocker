//! Ported from de.hysky.skyblocker.utils.ApiAuthentication by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::java::security::private_key::PrivateKey;
use mcrust::java::security::signature::Signature;
use mcrust::java::util::base64;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::multiplayer::profile_key_pair_manager::ProfileKeyPairManager;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::shared_constants;
use mcrust::net::minecraft::world::entity::player::profile_key_pair::ProfileKeyPair;

/// java: private static final String MINECRAFT_VERSION = SharedConstants.getCurrentVersion().name();
pub static MINECRAFT_VERSION: LazyLock<String> = LazyLock::new(|| shared_constants::get_current_version().name().to_string());
/// java: private static final String AUTH_URL = "https://hysky.de/api/aaron/authenticate";
pub const AUTH_URL: &str = "https://hysky.de/api/aaron/authenticate";
/// java: private static final String CONTENT_TYPE = "application/json";
pub const CONTENT_TYPE: &str = "application/json";
/// java: private static final String ALGORITHM = "SHA256withRSA";
pub const ALGORITHM: &str = "SHA256withRSA";
/// java: private static @Nullable TokenInfo tokenInfo = null;
pub static TOKEN_INFO: LazyLock<Mutex<Option<TokenInfo>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static boolean sentWarningOnce;
pub static SENT_WARNING_ONCE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final Codec<TokenRequest> CODEC = RecordCodecBuilder.create(instance -> instance.group( KeyPairInfo.CODEC.fieldOf("keyPair").forGetter(TokenRequest::keyPairInfo), SignedData.CODEC.field
pub static TOKEN_REQUEST_CODEC: compat::MainThread<CodecRef<TokenRequest>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply5(record_codec_builder::of(|o: &TokenRequest| o.key_pair_info(), codec::field_of(KEY_PAIR_INFO_CODEC.get_mut().clone(), "keyPair")), record_codec_builder::of(|o: &TokenRequest| o.signed_data(), codec::field_of(SIGNED_DATA_CODEC.get_mut().clone(), "signedData")), record_codec_builder::of(|o: &TokenRequest| o.r#mod(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "mod")), record_codec_builder::of(|o: &TokenRequest| o.minecraft_version(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "minecraftVersion")), record_codec_builder::of(|o: &TokenRequest| o.mod_version(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "modVersion")), move |__a0, __a1, __a2, __a3, __a4| TokenRequest::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone()))));
/// java: private static final Codec<KeyPairInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( UUIDUtil.STRING_CODEC.fieldOf("uuid").forGetter(KeyPairInfo::uuid), Codec.STRING.fieldOf("publicKe
pub static KEY_PAIR_INFO_CODEC: compat::MainThread<CodecRef<KeyPairInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &KeyPairInfo| o.uuid(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), record_codec_builder::of(|o: &KeyPairInfo| o.public_key(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "publicKey")), record_codec_builder::of(|o: &KeyPairInfo| o.public_key_signature(), codec::field_of(mcrust::net::minecraft::util::extra_codecs::base64_string(), "publicKeySignature")), record_codec_builder::of(|o: &KeyPairInfo| o.expires_at(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "expiresAt")), move |__a0, __a1, __a2, __a3| KeyPairInfo::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));
/// java: private static final Codec<SignedData> CODEC = RecordCodecBuilder.create(instance -> instance.group( ExtraCodecs.BASE64_STRING.fieldOf("original").forGetter(SignedData::original), ExtraCodecs.BASE64_S
pub static SIGNED_DATA_CODEC: compat::MainThread<CodecRef<SignedData>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &SignedData| o.original(), codec::field_of(mcrust::net::minecraft::util::extra_codecs::base64_string(), "original")), record_codec_builder::of(|o: &SignedData| o.signed(), codec::field_of(mcrust::net::minecraft::util::extra_codecs::base64_string(), "signed")), move |__a0, __a1| SignedData::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<TokenInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("token").forGetter(TokenInfo::token), Codec.LONG.fieldOf("issuedAt").forGetter(
pub static TOKEN_INFO_CODEC: compat::MainThread<CodecRef<TokenInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &TokenInfo| o.token(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "token")), record_codec_builder::of(|o: &TokenInfo| o.issued_at(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "issuedAt")), record_codec_builder::of(|o: &TokenInfo| o.expires_at(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "expiresAt")), move |__a0, __a1, __a2| TokenInfo::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public class ApiAuthentication
#[derive(Clone)]
pub struct ApiAuthentication {}

impl ApiAuthentication {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> updateToken());
        // (CLIENT_STARTED fires once, on the client thread, after the `Minecraft` constructor - where the mods' entrypoints
        // run - has returned: `compat::execute` from an entrypoint is exactly that point)
        compat::execute(|| ApiAuthentication::update_token());
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            client_command_registration_callback::EVENT.register(Arc::new(
                |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                    //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("updateToken").executes(_ -> {
                    //   java: 		updateToken(); return Command.SINGLE_SUCCESS; })));
                    use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                    let mut update_token = client_command_registration_callback::literal("updateToken");
                    update_token.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                        ApiAuthentication::update_token();
                        Ok(1)
                    }));
                    let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                    root.then(&update_token);
                    dispatcher.dispatcher().register(&root);
                },
            ));
        }
    }

    /// java: private static void updateToken()
    pub fn update_token() {
        // java: ((MinecraftAccessor) CLIENT).getProfileKeyPairManagerField().prepareKeyPair()
        // (`Minecraft.profileKeyPairManager` is final and never null in Java; the port's slot is empty only before the
        // game installs it, where `prepareKeyPair` has no keys to answer: a completed empty future)
        let future = match compat::minecraft().get_profile_key_pair_manager().as_mut() {
            Some(manager) => manager.prepare_key_pair(),
            None => mcrust::net::minecraft::client::multiplayer::profile_key_pair_manager::KeyPairFuture::completed(None),
        };
        // java: CLIENT.getUser().getProfileId() - read here, on the render thread that owns the User (the same value
        // the async body reads: the user does not change while the game runs)
        let profile_id: Option<Uuid> = Uuid::from_string(&compat::minecraft().get_user().get_profile_id());
        // SkyblockerMod.SKYBLOCKER_MOD / VERSION are render-thread statics here (compat::MainThread): read them here too
        let mod_id: String = skyblocker_mod::SKYBLOCKER_MOD.get_mut().metadata.id.clone();
        let mod_version: String = skyblocker_mod::VERSION.clone();
        // the two codecs are render-thread statics too (compat::MainThread): taken here, used on the worker
        let token_request_codec: CodecRef<TokenRequest> = TOKEN_REQUEST_CODEC.get().clone();
        let token_info_codec: CodecRef<TokenInfo> = TOKEN_INFO_CODEC.get().clone();
        // java: .thenAcceptAsync(playerKeypairOpt -> {..}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).exceptionally(throwable -> {..})
        // (a panic in the action is the exceptional completion `exceptionally` handles)
        future.then_accept_async(move |player_keypair_opt| {
            let __outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut expired: bool = player_keypair_opt.as_ref().map(|key_pair| key_pair.public_key().data().has_expired()).unwrap_or(false);
                if player_keypair_opt.is_some() && !expired {
                    let player_key_pair = player_keypair_opt.as_ref().unwrap();
                    //The key header and footer can be sent but that doesn't matter to the server
                    // java: Base64.getMimeEncoder() is getMimeEncoder(76, "\r\n")
                    let mut public_key: String = base64::get_mime_encoder(76, b"\r\n").expect("Base64.getMimeEncoder()").encode_to_string(player_key_pair.public_key().data().key_encoded());
                    let public_key_signature: Vec<u8> = player_key_pair.public_key().data().key_signature().to_vec();
                    let mut expires_at: i64 = player_key_pair.public_key().data().expires_at().to_epoch_milli();
                    let mut key_pair_info: KeyPairInfo = KeyPairInfo::new(profile_id.clone().expect("Objects.requireNonNull(CLIENT.getUser().getProfileId())"), public_key.clone(), public_key_signature, expires_at);
                    let mut signed_data: SignedData = ApiAuthentication::get_random_signed_data(player_key_pair.private_key()).expect("Objects.requireNonNull(getRandomSignedData(..))");
                    let mut token_request: TokenRequest = TokenRequest::new(key_pair_info.clone(), signed_data.clone(), mod_id.clone(), MINECRAFT_VERSION.clone(), mod_version.clone());
                    // java: String request = SkyblockerMod.GSON.toJson(TokenRequest.CODEC.encodeStart(JsonOps.INSTANCE, tokenRequest).getOrThrow());
                    // (`GSON` is the pretty printer: `serde_json::to_string_pretty`, see `SkyblockerMod.GSON`)
                    let encoded = token_request_codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &token_request).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
                    let mut request: String = serde_json::to_string_pretty(&compat::json_from_port(&encoded)).expect("Gson.toJson");
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // java: tokenInfo = TokenInfo.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(Http.sendPostRequest(AUTH_URL, request, CONTENT_TYPE))).getOrThrow();
                        let response: String = crate::de::hysky::skyblocker::utils::http::Http::send_post_request(AUTH_URL, &request, CONTENT_TYPE);
                        let json: serde_json::Value = serde_json::from_str(&response).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                        let token_info: TokenInfo = token_info_codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
                        *compat::lock(&TOKEN_INFO) = Some(token_info);
                        RenderHelper::run_on_render_thread(Arc::new(move || {
                            let (expires_at, issued_at) = {
                                let token_info = compat::lock(&TOKEN_INFO);
                                let token_info = token_info.as_ref().expect("tokenInfo");
                                (token_info.expires_at(), token_info.issued_at())
                            };
                            let mut refresh_at_ticks: i32 = ((((expires_at.wrapping_sub(issued_at)).wrapping_div(1000)).wrapping_sub(300)) as i32).wrapping_mul(20);
                            logger().info("[Skyblocker Api Auth] Successfully refreshed api token.");
                            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { ApiAuthentication::update_token(); }), refresh_at_ticks) });
                        }));
                    }));
                    if let Err(e) = __result {
                        //Try again in 15 minutes
                        ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.authFailure"), 900i32.wrapping_mul(20), "[Skyblocker Api Auth] Failed to refresh the api token! Some features might not work :(", &vec![(std::sync::Arc::new(compat::panic_message(&e)) as compat::Object)]);
                    }
                } else {
                    ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.noProfileKeys"), (if expired { -1 } else { 300i32.wrapping_mul(20) }), "[Skyblocker Api Auth] Failed to fetch profile keys! Some features may not work temporarily :( (Has your game been open for more than 24 hours? If so restart.)", &vec![]);
                }
            }));
            if let Err(throwable) = __outcome {
                //Try again in 5 minutes
                ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.authFailure"), 300i32.wrapping_mul(20), "[Skyblocker Api Auth] Encountered an unexpected exception while refreshing the api token!", &vec![(std::sync::Arc::new(compat::panic_message(&throwable)) as compat::Object)]);
            }
        }, |task: Box<dyn FnOnce() + Send>| compat::executor(task));
    }

    /// java: private static TokenRequest.SignedData getRandomSignedData(PrivateKey privateKey)
    pub fn get_random_signed_data(private_key: &PrivateKey) -> Option<SignedData> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut signature: Signature = Signature::get_instance(ALGORITHM).unwrap();
            let mut uuid: Uuid = Uuid::random_uuid();
            // java: ByteBuffer buf = ByteBuffer.allocate(16).putLong(uuid.getMostSignificantBits()).putLong(uuid.getLeastSignificantBits());
            // (a ByteBuffer is big-endian)
            let mut buf: Vec<u8> = Vec::with_capacity(16);
            buf.extend_from_slice(&uuid.get_most_significant_bits().to_be_bytes());
            buf.extend_from_slice(&uuid.get_least_significant_bits().to_be_bytes());
            signature.init_sign(std::sync::Arc::clone(private_key.provider_handle_arc()));
            signature.update(&buf);
            let mut signed_data: Vec<u8> = signature.sign().unwrap();
            return Some(SignedData::new(buf.clone(), signed_data));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Api Auth] Failed to sign random data! {}", compat::panic_message(&e)));
            }
        }
        //This should never ever be the case, since we are signing data that is not invalid in any case
        return None;
    }

    /// SLF4J's `MessageFormatter.arrayFormat` as `LOGGER.error(String, Object...)` applies it: each `{}` takes the next
    /// argument (`\{}` escapes it); an argument left over after the last placeholder is the throwable, printed on
    /// the lines after the message.
    fn slf4j_format(message: &str, args: &[compat::Object]) -> String {
        let text = |arg: &compat::Object| -> String {
            if let Some(s) = arg.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = arg.downcast_ref::<&'static str>() {
                s.to_string()
            } else {
                "null".to_string()
            }
        };
        let mut out = String::with_capacity(message.len());
        let mut next = 0usize;
        let mut rest = message;
        while let Some(at) = rest.find("{}") {
            if next >= args.len() {
                break;
            }
            if rest[..at].ends_with('\\') {
                out.push_str(&rest[..at - 1]);
                out.push_str("{}");
            } else {
                out.push_str(&rest[..at]);
                out.push_str(&text(&args[next]));
                next += 1;
            }
            rest = &rest[at + 2..];
        }
        out.push_str(rest);
        if next < args.len() {
            out.push('\n');
            out.push_str(&text(&args[args.len() - 1]));
        }
        out
    }

    /// java: private static void logErrorAndScheduleRetry(Component warningMessage, int retryAfter, String logMessage, Object... logArgs)
    pub fn log_error_and_schedule_retry(warning_message: &MutableComponent, mut retry_after: i32, log_message: &str, log_args: &Vec<compat::Object>) {
        // the lambda keeps its own copies (Java's captured references)
        let warning_message: MutableComponent = warning_message.clone();
        let log_message: String = log_message.to_string();
        let log_args: Vec<compat::Object> = log_args.clone();
        RenderHelper::run_on_render_thread(Arc::new(move || {
            //   java: LOGGER.error(logMessage, logArgs);
            logger().error(&ApiAuthentication::slf4j_format(&log_message, &log_args));
            if retry_after != -1 {
                ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { ApiAuthentication::update_token(); }), retry_after) });
            }
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && !(compat::get_state(&SENT_WARNING_ONCE)) {
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(warning_message.clone())); });
                compat::set_state(&SENT_WARNING_ONCE, true);
            }
        }));
    }

    /// java: public static @Nullable String getToken()
    pub fn get_token() -> Option<String> {
        return (if compat::get_state(&TOKEN_INFO).is_some() { Some(compat::lock(&TOKEN_INFO).as_ref().unwrap().token()) } else { None });
    }

}

/// java: private record TokenRequest(KeyPairInfo keyPairInfo, SignedData signedData, String mod, String minecraftVersion, String modVersion)
#[derive(Clone, Debug, PartialEq)]
pub struct TokenRequest {
    /// java: KeyPairInfo keyPairInfo
    pub key_pair_info: KeyPairInfo,
    /// java: SignedData signedData
    pub signed_data: SignedData,
    /// java: String mod
    pub r#mod: String,
    /// java: String minecraftVersion
    pub minecraft_version: String,
    /// java: String modVersion
    pub mod_version: String,
}

impl TokenRequest {
    /// java: the canonical record constructor
    pub fn new(mut key_pair_info: KeyPairInfo, mut signed_data: SignedData, mut r#mod: String, mut minecraft_version: String, mut mod_version: String) -> Self {
        Self { key_pair_info, signed_data, r#mod, minecraft_version, mod_version }
    }

    /// java: the record accessor `keyPairInfo()`
    pub fn key_pair_info(&self) -> KeyPairInfo {
        self.key_pair_info.clone()
    }

    /// java: the record accessor `signedData()`
    pub fn signed_data(&self) -> SignedData {
        self.signed_data.clone()
    }

    /// java: the record accessor `mod()`
    pub fn r#mod(&self) -> String {
        self.r#mod.clone()
    }

    /// java: the record accessor `minecraftVersion()`
    pub fn minecraft_version(&self) -> String {
        self.minecraft_version.clone()
    }

    /// java: the record accessor `modVersion()`
    pub fn mod_version(&self) -> String {
        self.mod_version.clone()
    }

}

/// java: private record KeyPairInfo(UUID uuid, String publicKey, byte[] publicKeySignature, long expiresAt)
#[derive(Clone, Debug, PartialEq)]
pub struct KeyPairInfo {
    /// java: UUID uuid
    pub uuid: Uuid,
    /// java: String publicKey
    pub public_key: String,
    /// java: byte[] publicKeySignature
    pub public_key_signature: Vec<u8>,
    /// java: long expiresAt
    pub expires_at: i64,
}

impl KeyPairInfo {
    /// java: the canonical record constructor
    pub fn new(mut uuid: Uuid, mut public_key: String, mut public_key_signature: Vec<u8>, mut expires_at: i64) -> Self {
        Self { uuid, public_key, public_key_signature, expires_at }
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    /// java: the record accessor `publicKey()`
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }

    /// java: the record accessor `publicKeySignature()`
    pub fn public_key_signature(&self) -> Vec<u8> {
        self.public_key_signature.clone()
    }

    /// java: the record accessor `expiresAt()`
    pub fn expires_at(&self) -> i64 {
        self.expires_at
    }

}

/// java: private record SignedData(byte[] original, byte[] signed)
#[derive(Clone, Debug, PartialEq)]
pub struct SignedData {
    /// java: byte[] original
    pub original: Vec<u8>,
    /// java: byte[] signed
    pub signed: Vec<u8>,
}

impl SignedData {
    /// java: the canonical record constructor
    pub fn new(mut original: Vec<u8>, mut signed: Vec<u8>) -> Self {
        Self { original, signed }
    }

    /// java: the record accessor `original()`
    pub fn original(&self) -> Vec<u8> {
        self.original.clone()
    }

    /// java: the record accessor `signed()`
    pub fn signed(&self) -> Vec<u8> {
        self.signed.clone()
    }

}

/// java: private record TokenInfo(String token, long issuedAt, long expiresAt)
#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo {
    /// java: String token
    pub token: String,
    /// java: long issuedAt
    pub issued_at: i64,
    /// java: long expiresAt
    pub expires_at: i64,
}

impl TokenInfo {
    /// java: the canonical record constructor
    pub fn new(mut token: String, mut issued_at: i64, mut expires_at: i64) -> Self {
        Self { token, issued_at, expires_at }
    }

    /// java: the record accessor `token()`
    pub fn token(&self) -> String {
        self.token.clone()
    }

    /// java: the record accessor `issuedAt()`
    pub fn issued_at(&self) -> i64 {
        self.issued_at
    }

    /// java: the record accessor `expiresAt()`
    pub fn expires_at(&self) -> i64 {
        self.expires_at
    }

}
