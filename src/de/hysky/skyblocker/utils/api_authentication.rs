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
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> updateToken());
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            client_command_registration_callback::EVENT.register(Arc::new(
                |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                    // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                    //   java: dispatcher.register(..);
                },
            ));
        }
    }

    /// java: private static void updateToken()
    pub fn update_token() {
        let mut profile_keys: ProfileKeyPairManager = compat::minecraft().profile_key_pair_manager;
        (profile_keys.prepare_key_pair().then_accept_async(|player_keypair_opt| {
            // TODO(fabric2seer): `.map(..)` on a value of unknown type
            //   java: boolean expired = playerKeypairOpt.map(keyPair -> keyPair.publicKey().data().hasExpired()).orElse(false);
            let mut expired: bool = todo!("fabric2seer: the initialiser of `expired` (TODO above)");
            // TODO(fabric2seer): the condition: `.isPresent(..)` on a value of unknown type
            //   java: playerKeypairOpt.isPresent() && !expired
            if todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): `.get(..)` on a value of unknown type
                //   java: ProfileKeyPair playerKeyPair = playerKeypairOpt.get();
                let mut player_key_pair: ProfileKeyPair = todo!("fabric2seer: the initialiser of `playerKeyPair` (TODO above)");
                // TODO(fabric2seer): `.encodeToString(..)` on a Result
                //   java: String publicKey = Base64.getMimeEncoder().encodeToString(playerKeyPair.publicKey().data().key().getEncoded());
                let mut public_key: String = todo!("fabric2seer: the initialiser of `publicKey` (TODO above)");
                let public_key_signature = player_key_pair.public_key().data().key_signature();
                // TODO(fabric2seer): `.toEpochMilli(..)` on a Instant
                //   java: long expiresAt = playerKeyPair.publicKey().data().expiresAt().toEpochMilli();
                let mut expires_at: i64 = todo!("fabric2seer: the initialiser of `expiresAt` (TODO above)");
                let mut key_pair_info: KeyPairInfo = KeyPairInfo::new(compat::minecraft().get_user().get_profile_id(), public_key.clone(), public_key_signature.clone(), expires_at);
                let mut signed_data: SignedData = ApiAuthentication::get_random_signed_data(player_key_pair.private_key()).unwrap();
                let mut token_request: TokenRequest = TokenRequest::new(key_pair_info.clone(), signed_data.clone(), skyblocker_mod::SKYBLOCKER_MOD.get_mut().metadata.id.clone(), MINECRAFT_VERSION.clone(), skyblocker_mod::VERSION.clone());
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: String request = SkyblockerMod.GSON.toJson(TokenRequest.CODEC.encodeStart(JsonOps.INSTANCE, tokenRequest).getOrThrow());
                let mut request: String = todo!("fabric2seer: the initialiser of `request` (TODO above)");
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): `Http.sendPostRequest` is not a static method of Http
                    //   java: tokenInfo = TokenInfo.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(Http.sendPostRequest(AUTH_URL, request, CONTENT_TYPE))).getOrThrow();
                    RenderHelper::run_on_render_thread(Arc::new(move || {
                        let mut refresh_at_ticks: i32 = (((((compat::lock(&TOKEN_INFO).unwrap().expires_at()).wrapping_sub(compat::lock(&TOKEN_INFO).unwrap().issued_at())).wrapping_div(1000)).wrapping_sub(300)) as i32).wrapping_mul(20);
                        logger().info("[Skyblocker Api Auth] Successfully refreshed api token.");
                        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { ApiAuthentication::update_token(); }), refresh_at_ticks) });
                    }));
                }));
                if let Err(e) = __result {
                    ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.authFailure"), 900i32.wrapping_mul(20), "[Skyblocker Api Auth] Failed to refresh the api token! Some features might not work :(", &vec![(std::sync::Arc::new(e.clone()) as compat::Object)]);
                }
            } else {
                ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.noProfileKeys"), (if expired { -1 } else { 300i32.wrapping_mul(20) }), "[Skyblocker Api Auth] Failed to fetch profile keys! Some features may not work temporarily :( (Has your game been open for more than 24 hours? If so restart.)", &vec![]);
            }
        }, skyblocker_mod::VIRTUAL_THREAD_EXECUTOR.get_mut().clone())).exceptionally(|throwable| {
            ApiAuthentication::log_error_and_schedule_retry(&component::translatable("skyblocker.api.token.authFailure"), 300i32.wrapping_mul(20), "[Skyblocker Api Auth] Encountered an unexpected exception while refreshing the api token!", &vec![throwable.clone()]);
            return Default::default(); // java: return null
        });
    }

    /// java: private static TokenRequest.SignedData getRandomSignedData(PrivateKey privateKey)
    pub fn get_random_signed_data(private_key: &PrivateKey) -> Option<SignedData> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut signature: Signature = Signature::get_instance(ALGORITHM).unwrap();
            let mut uuid: Uuid = Uuid::random_uuid();
            // TODO(fabric2seer): static call ByteBuffer.allocate(..)
            //   java: ByteBuffer buf = ByteBuffer.allocate(16)
            //   java: 					.putLong(uuid.getMostSignificantBits())
            //   java: 					.putLong(uuid.getLeastSignificantBits());
            let mut buf: crate::unresolved::ByteBuffer = todo!("fabric2seer: the initialiser of `buf` (TODO above)");
            // TODO(fabric2seer): `.initSign(..)` on a Signature
            //   java: signature.initSign(privateKey);
            // TODO(fabric2seer): `.update(..)` on a Signature
            //   java: signature.update(buf.array());
            // TODO(fabric2seer): `.sign(..)` on a Signature
            //   java: byte[] signedData = signature.sign();
            let mut signed_data: Vec<i8> = todo!("fabric2seer: the initialiser of `signedData` (TODO above)");
            // TODO(fabric2seer): `.array(..)` on a ByteBuffer
            //   java: return new TokenRequest.SignedData(buf.array(), signedData);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Api Auth] Failed to sign random data!"));
            }
        }
        return None;
    }

    /// java: private static void logErrorAndScheduleRetry(Component warningMessage, int retryAfter, String logMessage, Object... logArgs)
    pub fn log_error_and_schedule_retry(warning_message: &MutableComponent, mut retry_after: i32, log_message: &str, log_args: &Vec<compat::Object>) {
        RenderHelper::run_on_render_thread(Arc::new(move || {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.error(logMessage, logArgs);
            if retry_after != -1 {
                ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { ApiAuthentication::update_token(); }), retry_after) });
            }
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && !(compat::get_state(&SENT_WARNING_ONCE)) {
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(warning_message.clone()).to_string()); });
                compat::set_state(&SENT_WARNING_ONCE, true);
            }
        }));
    }

    /// java: public static @Nullable String getToken()
    pub fn get_token() -> Option<String> {
        return (if compat::get_state(&TOKEN_INFO).is_some() { Some(compat::lock(&TOKEN_INFO).unwrap().token()) } else { None });
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
    pub public_key_signature: Vec<i8>,
    /// java: long expiresAt
    pub expires_at: i64,
}

impl KeyPairInfo {
    /// java: the canonical record constructor
    pub fn new(mut uuid: Uuid, mut public_key: String, mut public_key_signature: Vec<i8>, mut expires_at: i64) -> Self {
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
    pub fn public_key_signature(&self) -> Vec<i8> {
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
    pub original: Vec<i8>,
    /// java: byte[] signed
    pub signed: Vec<i8>,
}

impl SignedData {
    /// java: the canonical record constructor
    pub fn new(mut original: Vec<i8>, mut signed: Vec<i8>) -> Self {
        Self { original, signed }
    }

    /// java: the record accessor `original()`
    pub fn original(&self) -> Vec<i8> {
        self.original.clone()
    }

    /// java: the record accessor `signed()`
    pub fn signed(&self) -> Vec<i8> {
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
