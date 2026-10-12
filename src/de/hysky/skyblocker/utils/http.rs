//! Ported from de.hysky.skyblocker.utils.Http by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final String NAME_2_UUID = "https://api.minecraftservices.com/minecraft/profile/lookup/name/";
pub const NAME_2_UUID: &str = "https://api.minecraftservices.com/minecraft/profile/lookup/name/";
/// java: private static final String HYPIXEL_PROXY = "https://hysky.de/api/hypixel/v2/";
pub const HYPIXEL_PROXY: &str = "https://hysky.de/api/hypixel/v2/";
/// java: private static final Duration REQUEST_TIMEOUT = Duration.ofSeconds(60);
pub static REQUEST_TIMEOUT: compat::MainThread<std::time::Duration> = compat::MainThread::new(|| std::time::Duration::from_secs((60) as u64));
/// java: public static final String USER_AGENT = "Skyblocker/" + SkyblockerMod.VERSION + " (" + SharedConstants.getCurrentVersion().name() + ")";
pub static USER_AGENT: LazyLock<String> = LazyLock::new(|| format!("Skyblocker/{} ({})", *crate::entry::skyblocker_mod::VERSION, mcrust::net::minecraft::shared_constants::get_current_version().name()));

/// java: `java.net.http.HttpHeaders` - a response's headers, in the order the server sent them (names
/// case-insensitive)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HttpHeaders {
    pub headers: Vec<(String, String)>,
}

impl HttpHeaders {
    /// java: `HttpHeaders.firstValue(name)`
    pub fn first_value(&self, name: &str) -> Option<String> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone())
    }
}

/// java: public class Http
///
/// `HTTP_CLIENT` (connect timeout 10 s, the virtual-thread executor, `Redirect.NORMAL`, HTTP/2) is the port's
/// `java.net.http.http_client`: one blocking request per call on the calling thread (every caller is already on a
/// worker), redirects and the protocol version being the platform's.
#[derive(Clone)]
pub struct Http {}

impl Http {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// One request with the headers every `Http` request sends. Java asks for `gzip, deflate` and decodes the body
    /// in `getDecodedInputStream`; the port's client hands back the body bytes as sent, and this crate has no
    /// inflater, so the request asks for the identity encoding (the server then sends none - the `""` arm).
    fn send(method: &str, url: &str, accept: Option<&str>, extra: &[(&str, &str)], body: Option<&str>) -> mcrust::java::net::http::http_client::HttpResponse {
        let mut headers: Vec<(&str, &str)> = Vec::new();
        if let Some(accept) = accept {
            headers.push(("Accept", accept));
        }
        headers.push(("User-Agent", USER_AGENT.as_str()));
        headers.extend_from_slice(extra);
        mcrust::java::net::http::http_client::request_with_timeouts(method, url, &headers, body, Some((10_000, REQUEST_TIMEOUT_MILLIS)))
            .unwrap_or_else(|e| panic!("IOException: {e}"))
    }

    /// java: private static InputStream getDecodedInputStream(HttpResponse<InputStream> response) (read to a String)
    fn get_decoded_body(response: &mcrust::java::net::http::http_client::HttpResponse) -> String {
        let encoding: String = Http::get_content_encoding(&Http::headers_of(response));
        return match encoding.as_str() {
            "" => String::from_utf8_lossy(&response.bytes).into_owned(),
            // rust: port gap - no gzip / deflate decoder in this crate (the request asks for none, see `send`)
            _ => panic!("UnsupportedOperationException: The server sent content in an unexpected encoding: {encoding}"),
        };
    }

    fn headers_of(response: &mcrust::java::net::http::http_client::HttpResponse) -> HttpHeaders {
        HttpHeaders { headers: response.headers.clone() }
    }

    /// java: public static ApiResponse sendCacheableGetRequest(String url, @Nullable String token) throws IOException, InterruptedException
    pub fn send_cacheable_get_request(url: &str, token: Option<&str>) -> ApiResponse {
        let authorization: Option<(String, String)> = token.map(|token| {
            let header_name = if url.starts_with(HYPIXEL_PROXY) { "Hysky-Authorization" } else { "Authorization" };
            (header_name.to_string(), format!("Bearer {token}"))
        });
        let extra: Vec<(&str, &str)> = authorization.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        let response = Http::send("GET", url, Some("application/json"), &extra, None);
        let body: String = Http::get_decoded_body(&response);
        let headers = Http::headers_of(&response);
        return ApiResponse::new(body, response.status as i32, Http::get_cache_statuses(&headers), Http::get_age(&headers));
    }

    /// java: public static InputStream downloadContent(String url) throws IOException, InterruptedException (the bytes)
    pub fn download_content(url: &str) -> Vec<u8> {
        let response = Http::send("GET", url, Some("*/*"), &[], None);
        let encoding = Http::get_content_encoding(&Http::headers_of(&response));
        if !encoding.is_empty() {
            panic!("UnsupportedOperationException: The server sent content in an unexpected encoding: {encoding}");
        }
        return response.bytes;
    }

    /// java: public static String sendGetRequest(String url) throws IOException, InterruptedException
    pub fn send_get_request(url: &str) -> String {
        return Http::send_cacheable_get_request(url, None).content();
    }

    /// java: public static HttpHeaders sendHeadRequest(String url) throws IOException, InterruptedException
    pub fn send_head_request(url: &str) -> HttpHeaders {
        let response = Http::send("HEAD", url, None, &[], None);
        return Http::headers_of(&response);
    }

    /// java: public static String sendPostRequest(String url, String requestBody, String contentType) throws IOException, InterruptedException
    pub fn send_post_request(url: &str, request_body: &str, content_type: &str) -> String {
        let response = Http::send("POST", url, Some(content_type), &[("Content-Type", content_type)], Some(request_body));
        return Http::get_decoded_body(&response);
    }

    /// java: public static ApiResponse sendName2UuidRequest(String name) throws IOException, InterruptedException
    pub fn send_name2_uuid_request(name: &str) -> ApiResponse {
        return Http::send_cacheable_get_request(&format!("{NAME_2_UUID}{name}"), None);
    }

    /// java: public static ApiResponse sendHypixelRequest(String endpoint, String query) throws IOException, InterruptedException
    pub fn send_hypixel_request(endpoint: &str, query: &str) -> ApiResponse {
        let token = crate::de::hysky::skyblocker::utils::api_authentication::ApiAuthentication::get_token();
        return Http::send_cacheable_get_request(&format!("{HYPIXEL_PROXY}{endpoint}{query}"), token.as_deref());
    }

    /// java: private static String getContentEncoding(HttpHeaders headers)
    fn get_content_encoding(headers: &HttpHeaders) -> String {
        return headers.first_value("Content-Encoding").unwrap_or_default();
    }

    /// java: public static String getEtag(HttpHeaders headers)
    pub fn get_etag(headers: &HttpHeaders) -> String {
        return headers.first_value("Etag").unwrap_or_default();
    }

    /// java: public static String getLastModified(HttpHeaders headers)
    pub fn get_last_modified(headers: &HttpHeaders) -> String {
        return headers.first_value("Last-Modified").unwrap_or_default();
    }

    /// java: private static String[] getCacheStatuses(HttpHeaders headers)
    fn get_cache_statuses(headers: &HttpHeaders) -> Vec<String> {
        return vec![headers.first_value("CF-Cache-Status").unwrap_or_else(|| "UNKNOWN".to_string()), headers.first_value("Local-Cache-Status").unwrap_or_else(|| "UNKNOWN".to_string())];
    }

    /// java: private static int getAge(HttpHeaders headers) - `Integer.parseInt` (no trim; a bad value throws)
    fn get_age(headers: &HttpHeaders) -> i32 {
        return headers.first_value("Age").unwrap_or_else(|| "-1".to_string()).parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException"));
    }
}

/// `REQUEST_TIMEOUT` in the port client's unit (it takes milliseconds).
const REQUEST_TIMEOUT_MILLIS: i32 = 60_000;

/// java: public record ApiResponse(String content, int statusCode, String[] cacheStatuses, int age) implements AutoCloseable
#[derive(Clone, Debug, PartialEq)]
pub struct ApiResponse {
    /// java: String content
    pub content: String,
    /// java: int statusCode
    pub status_code: i32,
    /// java: String[] cacheStatuses
    pub cache_statuses: Vec<String>,
    /// java: int age
    pub age: i32,
}

impl ApiResponse {
    /// java: the canonical record constructor
    pub fn new(content: String, status_code: i32, cache_statuses: Vec<String>, age: i32) -> Self {
        Self { content, status_code, cache_statuses, age }
    }

    /// java: the record accessor `content()`
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// java: the record accessor `statusCode()`
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    /// java: the record accessor `cacheStatuses()`
    pub fn cache_statuses(&self) -> Vec<String> {
        self.cache_statuses.clone()
    }

    /// java: the record accessor `age()`
    pub fn age(&self) -> i32 {
        self.age
    }

    /// java: public boolean ok()
    pub fn ok(&self) -> bool {
        return self.status_code == 200;
    }

    /// java: public boolean ratelimited()
    pub fn ratelimited(&self) -> bool {
        return self.status_code == 429;
    }

    /// java: public boolean cached()
    pub fn cached(&self) -> bool {
        return self.cache_statuses[0] == "HIT" || self.cache_statuses[1] == "HIT";
    }

    /// java: public void close() - "Maybe one day we'll have some resources to free"
    pub fn close(&self) {}
}
