//! fabric2seer compat helpers. Generated; safe to edit.

/// Detaches a callback argument from `ci`, so the handler can use the object
/// and still call `ci.cancel()` / `ci.set_return_value(..)`.
///
/// SAFETY (the same rule Mixin has for injector arguments): the reference is
/// valid only during the callback that produced it. Never store it.
pub fn detach<'a, T: ?Sized>(r: Option<&mut T>) -> Option<&'a mut T> {
    r.map(|p| unsafe { &mut *(p as *mut T) })
}

/// `detach` for a shared reference.
pub fn detach_ref<'a, T: ?Sized>(r: Option<&T>) -> Option<&'a T> {
    r.map(|p| unsafe { &*(p as *const T) })
}

/// Reads a ported static / @Unique field. The lock is released before this
/// returns, so one statement may read and write the same field.
pub fn get_state<T: Clone>(m: &std::sync::Mutex<T>) -> T {
    m.lock().unwrap_or_else(|p| p.into_inner()).clone()
}

/// Writes a ported static / @Unique field.
pub fn set_state<T>(m: &std::sync::Mutex<T>, value: T) {
    *m.lock().unwrap_or_else(|p| p.into_inner()) = value;
}

/// `System.currentTimeMillis()`.
pub fn millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// `System.nanoTime()` (monotonic, arbitrary origin).
pub fn nanos() -> i64 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_nanos() as i64
}

/// `Math.random()`.
pub fn random() -> f64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static STATE: AtomicU64 = AtomicU64::new(0x9E37_79B9_7F4A_7C15);
    let mut x = STATE.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    STATE.store(x, Ordering::Relaxed);
    (x >> 11) as f64 / (1u64 << 53) as f64
}

// ===========================================================================
// Java semantics for translated classes (fabric2seer chunk 368)
// ===========================================================================

/// A Java `static` holding a game object (the port's render / GUI types hold
/// `Rc`, `RefCell` and non-`Send` trait objects, so neither `Mutex` nor
/// `LazyLock` accepts them). Java only touches such statics on the render
/// thread; this cell makes that the rule: it is created lazily on first use,
/// remembers that thread, and PANICS (naming the static's type) if any other
/// thread reaches it. `get` / `get_mut` hand out the value; as with the Java
/// field, a caller must not hold the `&mut` across a call that reads the same
/// static again.
pub struct MainThread<T> {
    init: fn() -> T,
    value: std::cell::UnsafeCell<Option<T>>,
    owner: std::sync::OnceLock<std::thread::ThreadId>,
}

// SAFETY: every access goes through `check`, which pins the cell to one thread.
unsafe impl<T> Sync for MainThread<T> {}
unsafe impl<T> Send for MainThread<T> {}

impl<T> MainThread<T> {
    pub const fn new(init: fn() -> T) -> MainThread<T> {
        MainThread { init, value: std::cell::UnsafeCell::new(None), owner: std::sync::OnceLock::new() }
    }

    fn check(&self) {
        let me = std::thread::current().id();
        let owner = *self.owner.get_or_init(|| me);
        if owner != me {
            panic!(
                "fabric2seer MainThread<{}>: a static first used on {:?} was reached from {:?} - Java only touches it on the render thread",
                std::any::type_name::<T>(),
                owner,
                me
            );
        }
    }

    /// The value (created on first use).
    pub fn get(&self) -> &T {
        self.get_mut()
    }

    /// The value, mutably (a Java field write / mutating call).
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        self.check();
        // SAFETY: one thread (checked above); no reference outlives the statement that asked for it in the
        // generated code.
        let slot = unsafe { &mut *self.value.get() };
        if slot.is_none() {
            *slot = Some((self.init)());
        }
        slot.as_mut().unwrap()
    }

    /// `with(|v| ..)`: read through a closure.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(self.get())
    }

    /// `with_mut(|v| ..)`: change through a closure.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(self.get_mut())
    }

    /// A clone of the value.
    pub fn get_clone(&self) -> T
    where
        T: Clone,
    {
        self.get().clone()
    }
}

/// `X.register(..)` on a main-thread static (a mod's own event): the value, on its thread.
impl<T> std::ops::Deref for MainThread<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}

/// `java.lang.Object` held by value: any shared value.
pub type Object = std::sync::Arc<dyn std::any::Any + Send + Sync>;

/// `new Object()` (a lock or a sentinel).
pub fn new_object() -> Object {
    std::sync::Arc::new(())
}

/// Locks a ported static (`static X: LazyLock<Mutex<T>>`) for one call:
/// `compat::lock(&X).insert(k, v)`. The guard is dropped at the end of the
/// statement - never hold it across a call that reads `X` again.
pub fn lock<T>(m: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

/// `String.valueOf(obj)` for a value with a `Display`.
pub fn jstring<T: std::fmt::Display + ?Sized>(v: &T) -> String {
    v.to_string()
}

/// `String.valueOf(x)` of a nullable value: "null" when absent.
pub fn opt_str<T: std::fmt::Display>(v: &Option<T>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

/// `Double.toString(d)` (and `Float`): Java always prints a fraction
/// (`1.0`), uses `E` notation outside 1e-3..1e7, and spells NaN / Infinity.
pub fn fstr(d: f64) -> String {
    if d.is_nan() {
        return "NaN".to_string();
    }
    if d.is_infinite() {
        return if d > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let a = d.abs();
    if a != 0.0 && (a < 1e-3 || a >= 1e7) {
        let s = format!("{:E}", d);
        let (m, e) = s.split_once('E').unwrap_or((&s, "0"));
        let m = if m.contains('.') { m.to_string() } else { format!("{m}.0") };
        return format!("{m}E{e}");
    }
    if d.fract() == 0.0 {
        format!("{:.1}", d)
    } else {
        format!("{}", d)
    }
}

/// `String.length()` (chars; see the module note on UTF-16).
pub fn str_len(s: &str) -> i32 {
    s.chars().count() as i32
}

/// `String.isBlank()`: only `Character.isWhitespace` characters.
pub fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

/// `String.trim()`: strips every char <= ' ' (not Unicode whitespace).
pub fn trim(s: &str) -> String {
    s.trim_matches(|c: char| c <= ' ').to_string()
}

/// `String.equalsIgnoreCase`.
pub fn equals_ignore_case(a: &str, b: &str) -> bool {
    a.chars().count() == b.chars().count()
        && a.chars().zip(b.chars()).all(|(x, y)| {
            x == y || x.to_uppercase().eq(y.to_uppercase()) || x.to_lowercase().eq(y.to_lowercase())
        })
}

fn char_to_byte(s: &str, i: i32) -> usize {
    if i <= 0 {
        return 0;
    }
    s.char_indices().nth(i as usize).map(|(b, _)| b).unwrap_or_else(|| {
        if s.chars().count() == i as usize {
            s.len()
        } else {
            panic!("StringIndexOutOfBoundsException: index {i}, length {}", s.chars().count())
        }
    })
}

fn byte_to_char(s: &str, b: usize) -> i32 {
    s[..b].chars().count() as i32
}

/// `String.substring(begin)`.
pub fn substring(s: &str, begin: i32) -> String {
    s[char_to_byte(s, begin)..].to_string()
}

/// `String.substring(begin, end)`.
pub fn substring2(s: &str, begin: i32, end: i32) -> String {
    if begin > end {
        panic!("StringIndexOutOfBoundsException: begin {begin}, end {end}");
    }
    s[char_to_byte(s, begin)..char_to_byte(s, end)].to_string()
}

/// `String.charAt(i)`.
pub fn char_at(s: &str, i: i32) -> char {
    s.chars().nth(i as usize).unwrap_or_else(|| panic!("StringIndexOutOfBoundsException: index {i}"))
}

/// `String.startsWith(prefix, offset)`.
pub fn starts_with_at(s: &str, prefix: &str, offset: i32) -> bool {
    offset >= 0 && offset as usize <= s.chars().count() && s[char_to_byte(s, offset)..].starts_with(prefix)
}

/// What `indexOf` / `contains` accept: a `&str` / `&String` or a `char`.
pub trait Needle {
    fn find_in(&self, hay: &str) -> Option<usize>;
    fn rfind_in(&self, hay: &str) -> Option<usize>;
}
impl Needle for &str {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(*self)
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(*self)
    }
}
impl Needle for &String {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(self.as_str())
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(self.as_str())
    }
}
impl Needle for char {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(*self)
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(*self)
    }
}

/// `String.indexOf(x)`: -1 when absent.
pub fn index_of(s: &str, n: impl Needle) -> i32 {
    n.find_in(s).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.indexOf(x, from)`.
pub fn index_of_from(s: &str, n: impl Needle, from: i32) -> i32 {
    let len = s.chars().count() as i32;
    if from >= len {
        return -1;
    }
    let start = char_to_byte(s, from.max(0));
    n.find_in(&s[start..]).map_or(-1, |b| byte_to_char(s, start + b))
}

/// `String.lastIndexOf(x)`.
pub fn last_index_of(s: &str, n: impl Needle) -> i32 {
    n.rfind_in(s).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.lastIndexOf(x, from)`.
pub fn last_index_of_from(s: &str, n: impl Needle, from: i32) -> i32 {
    if from < 0 {
        return -1;
    }
    let end = s.char_indices().nth(from as usize + 1).map_or(s.len(), |(b, _)| b);
    n.rfind_in(&s[..end]).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.compareTo`: UTF-16 code unit order (not UTF-8 byte order).
pub fn compare_to(a: &str, b: &str) -> i32 {
    let mut x = a.encode_utf16();
    let mut y = b.encode_utf16();
    loop {
        match (x.next(), y.next()) {
            (Some(p), Some(q)) if p != q => return p as i32 - q as i32,
            (Some(_), Some(_)) => {}
            (None, None) => return 0,
            (None, Some(_)) => return a.encode_utf16().count() as i32 - b.encode_utf16().count() as i32,
            (Some(_), None) => return a.encode_utf16().count() as i32 - b.encode_utf16().count() as i32,
        }
    }
}

/// `String.hashCode()` - Java's `s[0]*31^(n-1) + ...` over UTF-16 units.
pub fn string_hash_code(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

/// `Object.hashCode()` of a value with `Hash` (NOT Java's numbers: only equal-for-equal holds).
pub fn hash_code<T: std::hash::Hash + ?Sized>(v: &T) -> i32 {
    use std::hash::Hasher;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    v.hash(&mut h);
    h.finish() as i32
}

/// A comparison as Java's int (`compareTo` / `Comparator.compare`).
pub fn ord_int(o: std::cmp::Ordering) -> i32 {
    o as i32
}

/// `Math.round(float)`: floor(x + 0.5).
pub fn java_round_f32(x: f32) -> i32 {
    (x + 0.5).floor() as i32
}

/// `Math.round(double)`: floor(x + 0.5).
pub fn java_round_f64(x: f64) -> i64 {
    (x + 0.5).floor() as i64
}

/// `Math.floorDiv` / `floorMod` (rounding toward negative infinity).
pub fn floor_div_i32(a: i32, b: i32) -> i32 {
    let q = a.wrapping_div(b);
    if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q }
}
pub fn floor_mod_i32(a: i32, b: i32) -> i32 {
    a.wrapping_sub(floor_div_i32(a, b).wrapping_mul(b))
}
pub fn floor_div_i64(a: i64, b: i64) -> i64 {
    let q = a.wrapping_div(b);
    if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q }
}
pub fn floor_mod_i64(a: i64, b: i64) -> i64 {
    a.wrapping_sub(floor_div_i64(a, b).wrapping_mul(b))
}

/// `Mth.lerp` / `Math` helpers.
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

/// `Character.toUpperCase(c)` / `toLowerCase(c)` (one char in, one out).
pub fn char_upper(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}
pub fn char_lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// `List.remove(Object)`: removes the first equal element.
pub fn remove_item<T: PartialEq>(v: &mut Vec<T>, x: &T) -> bool {
    match v.iter().position(|e| e == x) {
        Some(i) => {
            v.remove(i);
            true
        }
        None => false,
    }
}

/// `StringBuilder.setLength(n)`.
pub fn set_length(s: &mut String, n: i32) {
    let b = char_to_byte(s, n);
    if b <= s.len() {
        s.truncate(b);
    }
    while (s.chars().count() as i32) < n {
        s.push('\0');
    }
}

/// `StringBuilder.insert(i, str)`.
pub fn insert_str(s: &mut String, i: i32, x: &str) {
    let b = char_to_byte(s, i);
    s.insert_str(b, x);
}

/// `StringBuilder.deleteCharAt(i)`.
pub fn delete_char_at(s: &mut String, i: i32) {
    let b = char_to_byte(s, i);
    s.remove(b);
}

/// `Arrays.copyOf(a, n)`.
pub fn copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
    let mut v: Vec<T> = a.iter().take(n).cloned().collect();
    v.resize(n, T::default());
    v
}

/// `Collections.shuffle(list)`.
pub fn shuffle<T>(v: &mut Vec<T>) {
    for i in (1..v.len()).rev() {
        let j = (random() * (i + 1) as f64) as usize;
        v.swap(i, j.min(i));
    }
}

/// `Stream.sorted()`.
pub fn sorted<T: Ord>(it: impl Iterator<Item = T>) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = it.collect();
    v.sort();
    v.into_iter()
}

/// `Stream.sorted(comparator)` (stable, as Java's).
pub fn sorted_by<T>(it: impl Iterator<Item = T>, cmp: impl FnMut(&T, &T) -> std::cmp::Ordering) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = it.collect();
    v.sort_by(cmp);
    v.into_iter()
}

/// `Stream.distinct()` (first occurrence kept, order kept).
pub fn distinct<T: PartialEq + Clone>(it: impl Iterator<Item = T>) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = Vec::new();
    for x in it {
        if !v.contains(&x) {
            v.push(x);
        }
    }
    v.into_iter()
}

/// `IntStream.average()`.
pub fn average(it: impl Iterator<Item = f64>) -> Option<f64> {
    let (mut n, mut s) = (0usize, 0.0);
    for x in it {
        n += 1;
        s += x;
    }
    if n == 0 { None } else { Some(s / n as f64) }
}

/// `Collectors.groupingBy(key)`.
pub fn grouping_by<T, K: std::hash::Hash + Eq>(
    it: impl Iterator<Item = T>,
    key: impl Fn(&T) -> K,
) -> std::collections::HashMap<K, Vec<T>> {
    let mut m: std::collections::HashMap<K, Vec<T>> = std::collections::HashMap::new();
    for x in it {
        m.entry(key(&x)).or_default().push(x);
    }
    m
}

/// `Collectors.toMap(k, v, merge)`.
pub fn to_map_merge<K: std::hash::Hash + Eq, V>(
    it: impl Iterator<Item = (K, V)>,
    merge: impl Fn(V, V) -> V,
) -> std::collections::HashMap<K, V> {
    let mut m = std::collections::HashMap::new();
    for (k, v) in it {
        let nv = match m.remove(&k) {
            Some(old) => merge(old, v),
            None => v,
        };
        m.insert(k, nv);
    }
    m
}

/// `Map.merge(k, v, f)`.
pub fn map_merge<K: std::hash::Hash + Eq, V: Clone>(
    m: &mut std::collections::HashMap<K, V>,
    k: K,
    v: V,
    f: impl Fn(V, V) -> V,
) -> V {
    let nv = match m.remove(&k) {
        Some(old) => f(old, v),
        None => v,
    };
    m.insert(k, nv.clone());
    nv
}

/// A caught panic's message (`e.getMessage()` in a translated `catch`).
pub fn panic_message(e: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "panic".to_string()
    }
}

/// `obj.getClass()` as a name.
pub fn type_name_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// `UUID.fromString` (the 128-bit value; the port's UUIDs are `u128` where it has them).
pub fn uuid_from_string(s: &str) -> u128 {
    u128::from_str_radix(&s.replace('-', ""), 16).expect("IllegalArgumentException: Invalid UUID string")
}

/// `UUID.randomUUID()` (version 4).
pub fn random_uuid() -> u128 {
    let a = (random() * u64::MAX as f64) as u128;
    let b = (random() * u64::MAX as f64) as u128;
    ((a << 64 | b) & !(0xF000u128 << 64) | (0x4000u128 << 64)) & !(0xC000u128 << 48) | (0x8000u128 << 48)
}

/// `TimeUnit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

/// `java.util.Random` (an xorshift generator; not Java's LCG sequence).
pub struct Random(u64);
impl Random {
    pub fn new() -> Random {
        Random((random() * u64::MAX as f64) as u64 | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn next_int(&mut self, bound: i32) -> i32 {
        (self.next() % bound.max(1) as u64) as i32
    }
    pub fn next_double(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn next_float(&mut self) -> f32 {
        self.next_double() as f32
    }
    pub fn next_boolean(&mut self) -> bool {
        self.next() & 1 == 1
    }
}

/// A Java `UUID` the port's entity code keeps as a `u128` (`Entity.get_uuid`), as the port's
/// `java::util::uuid::Uuid` (two signed halves, as Java's).
pub fn uuid_of(bits: u128) -> mcrust::java::util::uuid::Uuid {
    mcrust::java::util::uuid::Uuid::new((bits >> 64) as i64, bits as u64 as i64)
}

/// The other way: a `Uuid` for a port parameter typed `u128`.
pub fn uuid_bits(uuid: &mcrust::java::util::uuid::Uuid) -> u128 {
    ((uuid.get_most_significant_bits() as u64 as u128) << 64) | (uuid.get_least_significant_bits() as u64 as u128)
}

/// The executor `CompletableFuture.supplyAsync(.., executor)` / `runAsync` hand
/// their task to: a new thread per task (Java's virtual-thread executors).
pub fn executor(task: Box<dyn FnOnce() + Send>) {
    std::thread::spawn(task);
}

/// `Screen.font` / `Minecraft.font` as Java has it: a `Font` over the game's font set.
pub fn font() -> mcrust::net::minecraft::client::gui::font::Font<'static> {
    mcrust::net::minecraft::client::gui::font::Font { set: &minecraft().font }
}

/// A Component the port hands back as the `FormattedText` it is (`Component extends FormattedText`):
/// the component itself when it is one, else the same text as a literal component with its style.
pub fn component_of(
    text: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
    use mcrust::net::minecraft::network::chat::component;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    match text {
        FormattedText::Component(c) => *c,
        FormattedText::Empty => component::empty(),
        FormattedText::Plain(s) => component::literal(&s),
        FormattedText::Styled(s, style) => component::literal(&s).set_style(style),
        FormattedText::Composite(parts) => parts
            .into_iter()
            .fold(component::empty(), |acc, part| acc.append(component_of(part))),
    }
}

/// A Java `Component` the port holds as text in places (`Screen.title` is a `String`): either one, as a
/// component.
pub trait IntoComponent {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
}

impl IntoComponent for mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        self
    }
}

impl IntoComponent for String {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::literal(&self)
    }
}

impl IntoComponent for &str {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::literal(self)
    }
}

impl IntoComponent for mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        component_of(self)
    }
}

/// `new StringWidget(..)` as the port's own screens build one (confirm_link_screen.rs): an inactive
/// `AbstractWidget` of kind `StringWidget` whose message text is the component. A width of -1 is
/// Java's `font.width(message)`.
pub fn string_widget(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    message: impl IntoComponent,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
    let message = message.into_component();
    use mcrust::net::minecraft::network::chat::component::Component as _;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    let text = FormattedText::from(message.clone());
    let w = if width < 0 { font().width_of(&text) } else { width };
    let mut widget = AbstractWidget::new(x, y, w, height, message.get_string(), WidgetKind::StringWidget, OnPress::Close);
    widget.set_message_text(text);
    widget.active = false;
    widget
}

static MINECRAFT: std::sync::atomic::AtomicPtr<mcrust::net::minecraft::client::minecraft::Minecraft> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

/// Records the game's `Minecraft` on every client tick, for `minecraft()`.
/// The generated entrypoints call it first; it registers once.
pub fn install() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        mcrust::mcrust::modding::events::client_tick_events::START_CLIENT_TICK.register(std::sync::Arc::new(
            |mc: &mut mcrust::net::minecraft::client::minecraft::Minecraft| {
                MINECRAFT.store(mc as *mut _, std::sync::atomic::Ordering::Release);
            },
        ));
    });
}

/// `Minecraft.getInstance()`: the game's one Minecraft, as the tick events
/// hand it over (see `install`).
///
/// SAFETY (the rule Java's global has): the Minecraft lives as long as the
/// game; this is the same object the game is using, so do not hold the
/// reference across a call that hands you the Minecraft again.
pub fn minecraft() -> &'static mut mcrust::net::minecraft::client::minecraft::Minecraft {
    let p = MINECRAFT.load(std::sync::atomic::Ordering::Acquire);
    if p.is_null() {
        panic!("Minecraft.getInstance() before the first client tick (fabric2seer compat::minecraft)");
    }
    unsafe { &mut *p }
}


/// `java.util.regex` on `fancy-regex` (which has Java's look-around and
/// back-references). Indices are chars, as `compat::substring` expects.
pub mod regex {
    fn translate(java: &str) -> String {
        // Java's possessive quantifiers and atomic groups are fancy-regex syntax too; `\Q..\E` is not.
        if let (Some(a), Some(b)) = (java.find("\\Q"), java.find("\\E")) {
            if a < b {
                let lit = fancy_regex::escape(&java[a + 2..b]).into_owned();
                return format!("{}{}{}", &java[..a], lit, translate(&java[b + 2..]));
            }
        }
        java.to_string()
    }

    fn java_replacement(rep: &str) -> String {
        // Java: `$1` / `${name}` refer to groups, `\$` and `\\` are literal; fancy-regex: `$$` is a literal `$`
        let mut out = String::new();
        let mut it = rep.chars().peekable();
        while let Some(c) = it.next() {
            if c == '\\' {
                if let Some(n) = it.next() {
                    if n == '$' {
                        out.push_str("$$");
                    } else {
                        out.push(n);
                    }
                }
            } else if c == '$' && it.peek().is_some_and(|d| d.is_ascii_digit()) {
                out.push_str("${");
                while it.peek().is_some_and(|d| d.is_ascii_digit()) {
                    out.push(it.next().unwrap());
                }
                out.push('}');
            } else {
                out.push(c);
            }
        }
        out
    }

    /// `java.util.regex.Pattern`.
    #[derive(Clone, Debug)]
    pub struct Pattern {
        source: String,
        re: fancy_regex::Regex,
        full: fancy_regex::Regex,
        flags: i32,
    }

    pub const CASE_INSENSITIVE: i32 = 2;
    pub const MULTILINE: i32 = 8;
    pub const DOTALL: i32 = 32;

    impl Pattern {
        /// `Pattern.compile(regex)`; a bad pattern panics (Java throws PatternSyntaxException).
        pub fn compile(regex: &str) -> Pattern {
            Pattern::compile_with_flags(regex, 0)
        }

        pub fn compile_with_flags(regex: &str, flags: i32) -> Pattern {
            let mut pre = String::new();
            if flags & CASE_INSENSITIVE != 0 {
                pre.push('i');
            }
            if flags & MULTILINE != 0 {
                pre.push('m');
            }
            if flags & DOTALL != 0 {
                pre.push('s');
            }
            let body = translate(regex);
            let body = if pre.is_empty() { body } else { format!("(?{pre}){body}") };
            let re = fancy_regex::Regex::new(&body)
                .unwrap_or_else(|e| panic!("PatternSyntaxException: {e} in {regex:?}"));
            let full = fancy_regex::Regex::new(&format!("^(?:{body})$"))
                .unwrap_or_else(|e| panic!("PatternSyntaxException: {e} in {regex:?}"));
            Pattern { source: regex.to_string(), re, full, flags }
        }

        /// `Pattern.matches(regex, input)`.
        pub fn matches(regex: &str, input: &str) -> bool {
            Pattern::compile(regex).full.is_match(input).unwrap_or(false)
        }

        /// `Pattern.quote(s)`.
        pub fn quote(s: &str) -> String {
            fancy_regex::escape(s).into_owned()
        }

        pub fn pattern(&self) -> &str {
            &self.source
        }

        pub fn flags(&self) -> i32 {
            self.flags
        }

        /// `pattern.matcher(input)`.
        pub fn matcher(&self, input: &str) -> Matcher {
            Matcher { pattern: self.clone(), input: input.to_string(), caps: None, pos: 0 }
        }

        /// `asPredicate()`'s test: a match anywhere.
        pub fn find(&self, s: &str) -> bool {
            self.re.is_match(s).unwrap_or(false)
        }

        /// `asMatchPredicate()`'s test: the whole input.
        pub fn matches_all(&self, s: &str) -> bool {
            self.full.is_match(s).unwrap_or(false)
        }

        /// `pattern.split(input)` (trailing empty strings removed, as Java).
        pub fn split(&self, input: &str) -> Vec<String> {
            split_with(&self.re, input, 0)
        }
    }

    fn split_with(re: &fancy_regex::Regex, input: &str, limit: i32) -> Vec<String> {
        let mut out = Vec::new();
        let mut last = 0;
        for m in re.find_iter(input).flatten() {
            if limit > 0 && out.len() as i32 == limit - 1 {
                break;
            }
            if m.end() == 0 {
                continue; // a zero-width match at the start yields no leading empty string (Java 8+)
            }
            out.push(input[last..m.start()].to_string());
            last = m.end();
        }
        out.push(input[last..].to_string());
        if limit == 0 {
            while out.len() > 1 && out.last().is_some_and(|s| s.is_empty()) {
                out.pop();
            }
            if out.len() == 1 && out[0].is_empty() && !input.is_empty() {
                out.pop();
            }
        }
        out
    }

    /// `String.split(regex)`.
    pub fn split(input: &str, regex: &str) -> Vec<String> {
        split_with(&Pattern::compile(regex).re, input, 0)
    }

    /// `String.split(regex, limit)`.
    pub fn split_limit(input: &str, regex: &str, limit: i32) -> Vec<String> {
        split_with(&Pattern::compile(regex).re, input, limit)
    }

    /// `String.replaceAll(regex, replacement)`.
    pub fn replace_all(input: &str, regex: &str, rep: &str) -> String {
        Pattern::compile(regex).re.replace_all(input, java_replacement(rep).as_str()).into_owned()
    }

    /// `String.replaceFirst(regex, replacement)`.
    pub fn replace_first(input: &str, regex: &str, rep: &str) -> String {
        Pattern::compile(regex).re.replace(input, java_replacement(rep).as_str()).into_owned()
    }

    /// One match (`MatchResult`): the groups as found.
    #[derive(Clone, Debug)]
    pub struct MatchResult {
        groups: Vec<Option<(usize, usize)>>,
        names: Vec<Option<String>>,
        input: String,
    }

    impl MatchResult {
        pub fn group(&self, i: i32) -> Option<String> {
            self.groups.get(i as usize).copied().flatten().map(|(a, b)| self.input[a..b].to_string())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let i = self.names.iter().position(|n| n.as_deref() == Some(name))?;
            self.group(i as i32)
        }
        pub fn start(&self, i: i32) -> i32 {
            self.groups.get(i as usize).copied().flatten().map_or(-1, |(a, _)| self.input[..a].chars().count() as i32)
        }
        pub fn end(&self, i: i32) -> i32 {
            self.groups.get(i as usize).copied().flatten().map_or(-1, |(_, b)| self.input[..b].chars().count() as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.groups.len() as i32 - 1
        }
        /// `namedGroups().keySet()` - the pattern's group names.
        pub fn named_groups(&self) -> Vec<String> {
            self.names.iter().flatten().cloned().collect()
        }
    }

    /// `java.util.regex.Matcher`: stateful like Java's (`find` advances).
    #[derive(Clone, Debug)]
    pub struct Matcher {
        pattern: Pattern,
        input: String,
        caps: Option<MatchResult>,
        pos: usize,
    }

    impl Matcher {
        fn record(&mut self, full: bool, from: usize) -> bool {
            let re = if full { &self.pattern.full } else { &self.pattern.re };
            let names: Vec<Option<String>> = re.capture_names().map(|n| n.map(str::to_string)).collect();
            match re.captures_from_pos(&self.input, from) {
                Ok(Some(c)) => {
                    let groups: Vec<Option<(usize, usize)>> =
                        (0..c.len()).map(|i| c.get(i).map(|m| (m.start(), m.end()))).collect();
                    let end = groups[0].map_or(from, |(_, b)| b);
                    let start = groups[0].map_or(from, |(a, _)| a);
                    self.pos = if end == start { end + self.input[end..].chars().next().map_or(1, |c| c.len_utf8()) } else { end };
                    self.caps = Some(MatchResult { groups, names, input: self.input.clone() });
                    true
                }
                _ => {
                    self.caps = None;
                    false
                }
            }
        }

        /// `matches()`: the WHOLE input.
        pub fn matches(&mut self) -> bool {
            self.record(true, 0)
        }

        /// `lookingAt()`: a match at the start.
        pub fn looking_at(&mut self) -> bool {
            if self.record(false, 0) {
                if self.caps.as_ref().and_then(|c| c.groups[0]).is_some_and(|(a, _)| a == 0) {
                    return true;
                }
                self.caps = None;
            }
            false
        }

        /// `find()`: the next match.
        pub fn find(&mut self) -> bool {
            if self.pos > self.input.len() {
                self.caps = None;
                return false;
            }
            let from = self.pos;
            self.record(false, from)
        }

        /// `find(start)`.
        pub fn find_from(&mut self, start: i32) -> bool {
            let b = self.input.char_indices().nth(start.max(0) as usize).map_or(self.input.len(), |(b, _)| b);
            self.record(false, b)
        }

        /// `hasMatch()` (Java 20): whether the last match operation succeeded (a reset clears it).
        pub fn has_match(&self) -> bool {
            self.caps.is_some()
        }

        /// `hitEnd()` (approximation: whether the last search reached the end).
        pub fn hit_end(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn caps(&self) -> &MatchResult {
            self.caps.as_ref().expect("IllegalStateException: No match found")
        }

        /// `group(i)`: `None` for a group that did not take part (Java: null).
        pub fn group(&self, i: i32) -> Option<String> {
            self.caps().group(i)
        }

        /// `group(name)`.
        pub fn group_named(&self, name: &str) -> Option<String> {
            self.caps().group_named(name)
        }

        pub fn start(&self, i: i32) -> i32 {
            self.caps().start(i)
        }

        pub fn end(&self, i: i32) -> i32 {
            self.caps().end(i)
        }

        pub fn start_named(&self, name: &str) -> i32 {
            let c = self.caps();
            let i = c.names.iter().position(|n| n.as_deref() == Some(name)).map_or(-1, |i| i as i32);
            c.start(i)
        }

        pub fn end_named(&self, name: &str) -> i32 {
            let c = self.caps();
            let i = c.names.iter().position(|n| n.as_deref() == Some(name)).map_or(-1, |i| i as i32);
            c.end(i)
        }

        pub fn group_count(&self) -> i32 {
            self.pattern.re.captures_len() as i32 - 1
        }

        /// `reset()`.
        pub fn reset(&mut self) -> &mut Matcher {
            self.pos = 0;
            self.caps = None;
            self
        }

        /// `reset(input)`.
        pub fn reset_with(&mut self, input: &str) -> &mut Matcher {
            self.input = input.to_string();
            self.reset()
        }

        /// `usePattern(p)`.
        pub fn use_pattern(&mut self, p: &Pattern) -> &mut Matcher {
            self.pattern = p.clone();
            self
        }

        /// `replaceAll(replacement)`.
        pub fn replace_all(&self, rep: &str) -> String {
            self.pattern.re.replace_all(&self.input, java_replacement(rep).as_str()).into_owned()
        }

        /// `replaceFirst(replacement)`.
        pub fn replace_first(&self, rep: &str) -> String {
            self.pattern.re.replace(&self.input, java_replacement(rep).as_str()).into_owned()
        }

        /// `results()`: every match.
        pub fn results(&mut self) -> Vec<MatchResult> {
            self.reset();
            let mut out = Vec::new();
            while self.find() {
                out.push(self.caps().clone());
            }
            out
        }

        /// `Matcher.quoteReplacement(s)`.
        pub fn quote_replacement(s: &str) -> String {
            s.replace('\\', "\\\\").replace('$', "\\$")
        }
    }
}


/// Gson's `JsonObject` / `JsonElement` / `JsonArray` API on `serde_json`.
/// A missing member or a wrong type panics where Gson throws.
pub mod json {
    pub use serde_json::{Map, Value};

    /// Anything Gson calls a `JsonElement`.
    pub trait JsonRef {
        fn value(&self) -> Value;
    }
    impl JsonRef for Value {
        fn value(&self) -> Value {
            self.clone()
        }
    }
    impl JsonRef for Map<String, Value> {
        fn value(&self) -> Value {
            Value::Object(self.clone())
        }
    }
    impl JsonRef for Vec<Value> {
        fn value(&self) -> Value {
            Value::Array(self.clone())
        }
    }
    impl<T: JsonRef + ?Sized> JsonRef for &T {
        fn value(&self) -> Value {
            (**self).value()
        }
    }
    impl<T: JsonRef + ?Sized> JsonRef for &mut T {
        fn value(&self) -> Value {
            (**self).value()
        }
    }
    impl<T: JsonRef> JsonRef for Option<T> {
        fn value(&self) -> Value {
            self.as_ref().map_or(Value::Null, |v| v.value())
        }
    }

    fn member(j: &impl JsonRef, key: &str) -> Value {
        j.value().get(key).cloned().unwrap_or_else(|| panic!("NullPointerException: no JSON member {key:?}"))
    }

    pub fn has(j: &impl JsonRef, key: &str) -> bool {
        j.value().get(key).is_some()
    }
    pub fn get_as_json_object(j: &impl JsonRef) -> Map<String, Value> {
        match j.value() {
            Value::Object(m) => m,
            v => panic!("IllegalStateException: Not a JSON Object: {v}"),
        }
    }
    pub fn get_as_json_array(j: &impl JsonRef) -> Vec<Value> {
        match j.value() {
            Value::Array(a) => a,
            v => panic!("IllegalStateException: Not a JSON Array: {v}"),
        }
    }
    pub fn get_as_json_primitive(j: &impl JsonRef) -> Value {
        j.value()
    }
    pub fn get_as_string(j: &impl JsonRef) -> String {
        match j.value() {
            Value::String(s) => s,
            Value::Null => panic!("UnsupportedOperationException: JsonNull"),
            v => v.to_string(),
        }
    }
    pub fn get_as_double(j: &impl JsonRef) -> f64 {
        match j.value() {
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            Value::Bool(b) => panic!("UnsupportedOperationException: {b}"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_number(j: &impl JsonRef) -> f64 {
        get_as_double(j)
    }
    pub fn get_as_float(j: &impl JsonRef) -> f32 {
        get_as_double(j) as f32
    }
    pub fn get_as_int(j: &impl JsonRef) -> i32 {
        match j.value() {
            Value::Number(n) => n.as_i64().map_or_else(|| n.as_f64().unwrap_or(0.0) as i32, |x| x as i32),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_long(j: &impl JsonRef) -> i64 {
        match j.value() {
            Value::Number(n) => n.as_i64().unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_boolean(j: &impl JsonRef) -> bool {
        match j.value() {
            Value::Bool(b) => b,
            Value::String(s) => s.eq_ignore_ascii_case("true"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_json_object_of(j: &impl JsonRef, key: &str) -> Map<String, Value> {
        get_as_json_object(&member(j, key))
    }
    pub fn get_as_json_array_of(j: &impl JsonRef, key: &str) -> Vec<Value> {
        get_as_json_array(&member(j, key))
    }
    pub fn get_as_json_primitive_of(j: &impl JsonRef, key: &str) -> Value {
        member(j, key)
    }
    pub fn get_as_string_of(j: &impl JsonRef, key: &str) -> String {
        get_as_string(&member(j, key))
    }
    pub fn get_as_int_of(j: &impl JsonRef, key: &str) -> i32 {
        get_as_int(&member(j, key))
    }
    pub fn get_as_long_of(j: &impl JsonRef, key: &str) -> i64 {
        get_as_long(&member(j, key))
    }
    pub fn get_as_double_of(j: &impl JsonRef, key: &str) -> f64 {
        get_as_double(&member(j, key))
    }
    pub fn get_as_float_of(j: &impl JsonRef, key: &str) -> f32 {
        get_as_float(&member(j, key))
    }
    pub fn get_as_boolean_of(j: &impl JsonRef, key: &str) -> bool {
        get_as_boolean(&member(j, key))
    }
    pub fn is_object(j: &impl JsonRef) -> bool {
        j.value().is_object()
    }
    pub fn is_array(j: &impl JsonRef) -> bool {
        j.value().is_array()
    }
    pub fn is_primitive(j: &impl JsonRef) -> bool {
        matches!(j.value(), Value::String(_) | Value::Number(_) | Value::Bool(_))
    }
    pub fn is_null(j: &impl JsonRef) -> bool {
        j.value().is_null()
    }
    pub fn to_value(j: impl JsonRef) -> Value {
        j.value()
    }
    pub fn to_string(j: &impl JsonRef) -> String {
        j.value().to_string()
    }
}


/// A Gson tree of the mod (serde_json) for the port's `JsonOps`, whose tree is `lwjgl::json::Value`
/// (json_ops.rs header): the same JSON, re-read.
pub fn json_to_port(v: &serde_json::Value) -> mcrust::com::mojang::serialization::Obj {
    mcrust::com::mojang::serialization::obj(
        mcrust::lwjgl::json::parse(&v.to_string()).expect("a serde_json tree is valid JSON"),
    )
}

/// The other way: what `JsonOps` produced, as the mod's Gson tree.
pub fn json_from_port(o: &mcrust::com::mojang::serialization::Obj) -> serde_json::Value {
    match mcrust::com::mojang::serialization::cast::<mcrust::lwjgl::json::Value>(o) {
        Some(v) => serde_json::from_str(&mcrust::lwjgl::json::to_json(v)).unwrap_or(serde_json::Value::Null),
        None => serde_json::Value::Null,
    }
}



/// A Java REFERENCE to an object of the mod's own class hierarchy: shared by every holder (Java aliasing -
/// the same waypoint in a list and in a map, a widget returned after it was stored), mutable through any
/// of them. The lock is RE-ENTRANT, so a method that reaches its own object again on the same thread does
/// not hang; a conflicting borrow (a `&mut self` method re-entered) panics loudly instead.
pub type Shared<T: ?Sized> = std::sync::Arc<parking_lot::ReentrantMutex<std::cell::RefCell<T>>>;

/// `new X(..)` as a shared reference (cast to `Shared<dyn Trait>` where a trait object is held).
/// The object's own handle, kept by the object (Java's `this` handed to other objects).
pub type WeakShared<T: ?Sized> = std::sync::Weak<parking_lot::ReentrantMutex<std::cell::RefCell<T>>>;

/// Reads the object behind a shared reference: the borrow lasts exactly as long as `f`, never across a call
/// that may reach the same object again.
pub fn read<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&T) -> R) -> R {
    let guard = s.lock();
    let value = guard.borrow();
    f(&value)
}

/// Writes the object behind a shared reference, the borrow lasting exactly as long as `f`.
pub fn write<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&mut T) -> R) -> R {
    let guard = s.lock();
    let mut value = guard.borrow_mut();
    f(&mut value)
}

/// An object of the mod that keeps its own handle (`self_ref`): told it when it is put behind one.
pub trait HasSelfRef {
    fn set_self_ref(&mut self, handle: WeakShared<Self>);
}

/// `new X(..)` of a class the mod extends: the object behind its handle, knowing its handle.
pub fn shared_self<T: HasSelfRef>(value: T) -> Shared<T> {
    let handle = shared(value);
    let weak = std::sync::Arc::downgrade(&handle);
    write(&handle, |t| t.set_self_ref(weak));
    handle
}

/// Fetches what a call on a shared object needs (its trait method's body, a function pointer) under a
/// momentary borrow; the call itself then runs with nothing borrowed.
pub fn dispatch<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&T) -> R) -> R {
    read(s, f)
}

/// Every object of the mod's class hierarchy can be looked into as its concrete class (Java's checked cast);
/// a supertrait of each root trait of the mod, implemented for every type (no per-class code).
pub trait DynAny: std::any::Any {
    fn dyn_any(&self) -> &dyn std::any::Any;
    fn dyn_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: std::any::Any> DynAny for T {
    fn dyn_any(&self) -> &dyn std::any::Any {
        self
    }
    fn dyn_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Reads a field of the concrete class behind a trait-object handle (a per-statement downcast).
pub fn read_as<C: std::any::Any, D: ?Sized + DynAny, R>(s: &Shared<D>, f: impl FnOnce(&C) -> R) -> R {
    read(s, |t| f(t.dyn_any().downcast_ref::<C>().expect("compat::read_as: not an object of this class")))
}

/// Writes a field of the concrete class behind a trait-object handle (a per-statement downcast).
pub fn write_as<C: std::any::Any, D: ?Sized + DynAny, R>(s: &Shared<D>, f: impl FnOnce(&mut C) -> R) -> R {
    write(s, |t| f(t.dyn_any_mut().downcast_mut::<C>().expect("compat::write_as: not an object of this class")))
}

/// The same handle seen as another type (a trait object of a supertrait, the concrete class), borrowed from
/// the original: no allocation and no reference count taken. It lives no longer than the handle it views.
pub struct View<'a, T: ?Sized>(std::mem::ManuallyDrop<Shared<T>>, std::marker::PhantomData<&'a ()>);

impl<T: ?Sized> std::ops::Deref for View<'_, T> {
    type Target = Shared<T>;
    fn deref(&self) -> &Shared<T> {
        &self.0
    }
}

/// `h` as a handle of a supertrait (`|a| a as Shared<dyn Super>`, an unsizing coercion: the same allocation).
pub fn view<'a, A: ?Sized, B: ?Sized>(h: &'a Shared<A>, up: fn(Shared<A>) -> Shared<B>) -> View<'a, B> {
    // SAFETY: the Arc is rebuilt from the handle's own pointer without taking a count and never dropped
    // (ManuallyDrop); `up` is an unsizing coercion, so it is the same allocation, borrowed for 'a.
    let same = unsafe { std::sync::Arc::from_raw(std::sync::Arc::as_ptr(h)) };
    View(std::mem::ManuallyDrop::new(up(same)), std::marker::PhantomData)
}

/// `h` (a trait object) as a handle of its concrete class `C` (checked, as Java's cast), then `up` to the
/// handle type wanted (`|a| a` for `C` itself, `|a| a as Shared<dyn T>` for another trait `C` implements).
pub fn view_as<'a, C: std::any::Any, D: ?Sized + DynAny, B: ?Sized>(h: &'a Shared<D>, up: fn(Shared<C>) -> Shared<B>)
        -> View<'a, B> {
    assert!(read(h, |t| t.dyn_any().is::<C>()), "compat::view_as: not an object of this class");
    let p = std::sync::Arc::as_ptr(h) as *const parking_lot::ReentrantMutex<std::cell::RefCell<C>>;
    // SAFETY: the object behind the handle IS a `C` (checked above), so this is the allocation's own type;
    // no count is taken and the Arc is never dropped (ManuallyDrop); borrowed for 'a.
    let same = unsafe { std::sync::Arc::from_raw(p) };
    View(std::mem::ManuallyDrop::new(up(same)), std::marker::PhantomData)
}

/// `h` (a trait object) as its concrete class `C` when it is one (Java's `instanceof` + cast), borrowed from
/// the handle.
pub fn try_view<'a, C: std::any::Any, D: ?Sized + DynAny>(h: &'a Shared<D>) -> Option<View<'a, C>> {
    if !read(h, |t| t.dyn_any().is::<C>()) {
        return None;
    }
    Some(view_as::<C, D, C>(h, |a| a))
}

/// `h` (a trait object) as an owned handle of its concrete class `C` (Java's `this` handed on from a body
/// that runs on a trait object): the same object, its reference count taken.
pub fn handle_as<C: std::any::Any, D: ?Sized + DynAny>(h: &Shared<D>) -> Shared<C> {
    std::sync::Arc::clone(&view_as::<C, D, C>(h, |a| a))
}

/// A value a closure carries that the port types `Send + Sync` (a button's action, a scheduled task): the mod's
/// objects are not `Send`, and Java only touches them on the render thread - which this cell checks on every use.
pub struct RenderThread<T> {
    value: T,
    owner: std::thread::ThreadId,
}

// SAFETY: the value is reached only through `get`, which panics off the thread that made the cell.
unsafe impl<T> Send for RenderThread<T> {}
unsafe impl<T> Sync for RenderThread<T> {}

impl<T> RenderThread<T> {
    pub fn new(value: T) -> RenderThread<T> {
        RenderThread { value, owner: std::thread::current().id() }
    }

    pub fn get(&self) -> &T {
        assert!(std::thread::current().id() == self.owner, "a mod object was reached off the render thread");
        &self.value
    }
}

impl<T: Clone> Clone for RenderThread<T> {
    fn clone(&self) -> RenderThread<T> {
        RenderThread { value: self.get().clone(), owner: self.owner }
    }
}

/// A mod class that extends the game's `AbstractWidget` (or `AbstractButton`, ...): its widget half (`base`). The
/// game keeps the live widget - x, y, size, message, `active`, `visible` - in the screen's list and LENDS it to
/// the object for each of its methods (`WidgetAdapter::lend`), so `this.getX()` / `this.active = false` reach it.
pub trait WidgetHalf {
    fn widget_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
}

/// The fields of a widget the mod's code sets from outside the widget's own methods.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WidgetSnap {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    message: String,
    active: bool,
    visible: bool,
    alpha: f32,
}

impl WidgetSnap {
    pub fn of(w: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> WidgetSnap {
        WidgetSnap {
            x: w.get_x(),
            y: w.get_y(),
            width: w.width,
            height: w.height,
            message: w.message.clone(),
            active: w.active,
            visible: w.visible,
            alpha: w.alpha,
        }
    }

    /// What `from` changed since this snapshot, written into `to`.
    fn apply(&self, from: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
             to: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) {
        let now = WidgetSnap::of(from);
        if now.x != self.x {
            to.set_x(now.x);
        }
        if now.y != self.y {
            to.set_y(now.y);
        }
        if now.width != self.width {
            to.width = now.width;
        }
        if now.height != self.height {
            to.height = now.height;
        }
        if now.message != self.message {
            to.message = from.message.clone();
            to.message_text = from.message_text.clone();
        }
        if now.active != self.active {
            to.active = now.active;
        }
        if now.visible != self.visible {
            to.visible = now.visible;
        }
        if now.alpha != self.alpha {
            to.alpha = now.alpha;
        }
    }
}

/// A mod's widget object as the game's `ModWidget`: its handle (the same object the mod's code holds). Checked to
/// the render thread on every call (the mod's objects are not `Send`).
pub struct WidgetAdapter<C: ?Sized + 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
    /// the object's widget half as of the end of the last call: what the mod's code changes between calls is
    /// told apart from what the game changes (a layout moving the widget) and brought in at the next call
    snap: std::cell::RefCell<WidgetSnap>,
}

// SAFETY: every use goes through `check`, which pins the adapter to the thread that made it.
unsafe impl<C: ?Sized + 'static> Send for WidgetAdapter<C> {}

impl<C: WidgetHalf + ?Sized + 'static> WidgetAdapter<C> {
    pub fn new(object: Shared<C>) -> WidgetAdapter<C> {
        let snap = write(&object, |t| WidgetSnap::of(t.widget_half()));
        WidgetAdapter { object, owner: std::thread::current().id(), snap: std::cell::RefCell::new(snap) }
    }

    /// `new MyWidget(..)` handed to the game (`addRenderableWidget`, a layout): the game's widget, built from
    /// the object's own fields, with the object attached.
    pub fn widget(object: Shared<C>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
    where
        WidgetAdapter<C>: mcrust::mcrust::modding::widgets::ModWidget,
    {
        let mut w = write(&object, |t| t.widget_half().clone());
        w.kind = mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Mod(
            mcrust::mcrust::modding::widgets::ModWidgetSlot::new(Box::new(WidgetAdapter::new(object))),
        );
        w
    }

    /// The mod object.
    pub fn object(&self) -> &Shared<C> {
        self.check();
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod widget was reached off the render thread");
    }

    /// One of the object's methods with the game's widget lent to it (swapped into its `base`, back after - also
    /// on unwind); what the mod's code changed since the last call reaches the game's widget first.
    pub fn lend<R>(
        &self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        struct Back<'a, C: WidgetHalf + ?Sized>(
            &'a Shared<C>,
            &'a mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
            &'a std::cell::RefCell<WidgetSnap>,
        );
        impl<C: WidgetHalf + ?Sized> Drop for Back<'_, C> {
            fn drop(&mut self) {
                let lent = &mut *self.1;
                let snap = self.2;
                write(self.0, |t| {
                    std::mem::swap(t.widget_half(), lent);
                    // the object keeps a copy of the game's widget, but not the game's link to the object itself
                    let half = t.widget_half();
                    let kind = std::mem::replace(&mut half.kind, mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText);
                    *half = lent.clone();
                    half.kind = kind;
                    *snap.borrow_mut() = WidgetSnap::of(half);
                });
            }
        }
        write(&self.object, |t| {
            self.snap.borrow().apply(t.widget_half(), widget);
            std::mem::swap(t.widget_half(), widget);
        });
        let _back = Back(&self.object, widget, &self.snap);
        f(&self.object)
    }

    /// A method that only READS the game's widget (`extractWidgetRenderState`, narration): the object's half is
    /// brought up to date first, keeping what the mod's code changed and the game has not seen yet.
    pub fn lend_ref<R>(
        &self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let mut fresh = widget.clone();
            self.snap.borrow().apply(t.widget_half(), &mut fresh);
            let half = t.widget_half();
            fresh.kind = std::mem::replace(&mut half.kind, mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText);
            *half = fresh;
        });
        f(&self.object)
    }

    /// A method that does not touch the widget half (`isValidClickButton`).
    pub fn call<R>(&self, f: impl FnOnce(&Shared<C>) -> R) -> R {
        self.check();
        f(&self.object)
    }
}

/// A mod's `ObjectSelectionList` / `ContainerObjectSelectionList` subclass: the port's `ModSelectionList` it is
/// built on (`base`).
pub trait ListHalf {
    fn list_half(&mut self) -> &mut mcrust::mcrust::modding::widgets::ModSelectionList;
}

/// A mod's list subclass as the port's list CLASS (`widgets::ModList`): its handle, asked by the port's list
/// bodies at every statement Java calls an overridable method from (`getRowWidth()`, `scrollBarX()`, ...). The
/// list itself - the rows, the scroll, the box - is the game's widget (`attach_list`); the object keeps the
/// mod's own fields.
pub struct ListClass<L: 'static> {
    object: RenderThread<Shared<L>>,
}

thread_local! {
    static SUPER_ANSWER: std::cell::RefCell<Vec<Box<dyn std::any::Any>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `super.getRowWidth()` (& co) inside a list class's override: the answer of the class it extends, which the
/// port hands the override (`ListClass::ask`).
pub fn super_answer<T: Clone + 'static>() -> T {
    SUPER_ANSWER.with(|s| {
        s.borrow()
            .last()
            .and_then(|v| v.downcast_ref::<T>().cloned())
            .expect("super.x() of a list override outside the call that hands it its super answer")
    })
}

impl<L: ListHalf + 'static> ListClass<L> {
    pub fn new(object: Shared<L>) -> ListClass<L> {
        ListClass { object: RenderThread::new(object) }
    }

    /// The mod object (`instanceof MyList`: `list.class::<ListClass<MyList>>()`).
    pub fn object(&self) -> &Shared<L> {
        self.object.get()
    }

    /// One override, asked with the list's current state (the object's own view of its box - `this.getWidth()`
    /// inside the override - brought up to date first) and the base class's answer for `super.x()`.
    pub fn ask<T: Clone + 'static, R>(
        &self,
        list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
        super_answer: T,
        f: impl FnOnce(&Shared<L>) -> R,
    ) -> R {
        let object = self.object.get();
        write(object, |t| {
            let mut view = list.clone();
            view.set_class(None); // the object's copy holds no link back to the object
            t.list_half().list = view;
        });
        SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(super_answer)));
        struct Pop;
        impl Drop for Pop {
            fn drop(&mut self) {
                SUPER_ANSWER.with(|s| {
                    s.borrow_mut().pop();
                });
            }
        }
        let _pop = Pop;
        f(object)
    }
}

/// `addRenderableWidget(list)` (or a field the screen holds it in): the game's widget, the rows the object built
/// moved into it, the object attached as the list's class. From here on the LIVE list is the game's; the
/// object's code reaches it through its widget reference (`lend_list`).
pub fn attach_list<L: ListHalf + 'static>(object: Shared<L>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
where
    ListClass<L>: mcrust::mcrust::modding::widgets::ModList,
{
    let mut list = write(&object, |t| {
        let kind = t.list_half().kind;
        std::mem::replace(t.list_half(), mcrust::mcrust::modding::widgets::ModSelectionList::new(kind, 0, 0, 0, 0))
    });
    let (x, y, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
    list.list.set_class(Some(mcrust::mcrust::modding::widgets::ModListClass(std::sync::Arc::new(ListClass::new(object)))));
    mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, y, w, h, String::new(), Box::new(list))
}

/// One of the list object's own methods with the live list (the game's) lent to it for the call.
pub fn lend_list<L: ListHalf, R>(
    object: &Shared<L>,
    list: &mut mcrust::mcrust::modding::widgets::ModSelectionList,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, L: ListHalf>(&'a Shared<L>, &'a mut mcrust::mcrust::modding::widgets::ModSelectionList);
    impl<L: ListHalf> Drop for Back<'_, L> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| std::mem::swap(t.list_half(), lent));
        }
    }
    write(object, |t| std::mem::swap(t.list_half(), list));
    let _back = Back(object, list);
    f()
}

/// The mod list object attached to a game widget (`list` the screen holds by reference).
pub fn list_object<L: ListHalf + 'static>(
    widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
) -> Option<Shared<L>>
where
    ListClass<L>: mcrust::mcrust::modding::widgets::ModList,
{
    widget.mod_widget()?.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| {
        l.class::<ListClass<L>>().map(|c| c.object().clone())
    }).flatten()
}

/// The mod entry object at a row of the game's list (a `ModList` override's `list`).
pub fn row_object<E: ?Sized + 'static>(
    list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
    index: usize,
) -> Option<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    list.entry_object(index)?.with::<EntryAdapter<E>, _>(|a| a.object.clone())
}

/// The row of a mod entry object in the game's list (a `ModList` override's answer).
pub fn row_index<E: ?Sized + 'static>(
    list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
    entry: &Shared<E>,
) -> Option<usize>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).position(|i| row_object::<E>(list, i).is_some_and(|o| std::sync::Arc::ptr_eq(&o, entry)))
}

/// The rows of a mod list as the mod's entry objects (`children()`).
pub fn list_entries<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList) -> Vec<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).filter_map(|i| entry_at::<E>(list, i)).collect()
}

/// The row index of a mod entry object in its list (`removeEntry(e)`, `setSelected(e)`).
pub fn entry_index<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList, entry: &Shared<E>) -> Option<usize>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).position(|i| entry_at::<E>(list, i).is_some_and(|o| std::sync::Arc::ptr_eq(&o, entry)))
}

/// The mod entry object at a row (`getSelected()`).
pub fn entry_at<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList, index: usize) -> Option<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    list.entry_object(index)?.with::<EntryAdapter<E>, _>(|a| a.object.clone())
}

/// A mod's list row class (`ContainerObjectSelectionList.Entry` subclass): its row box (`base`) and the widgets
/// it holds (`__row`, a `Screen` as the port's container rows keep theirs, and their references' cache).
pub trait RowHalf {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut WidgetCache);
}

/// A mod's row object as the game's `ModListEntry`: its handle, and the row's widgets, which the game reaches
/// through `children()` - lent to the object for each of its methods.
pub struct EntryAdapter<E: ?Sized + 'static> {
    object: Shared<E>,
    pub row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    owner: std::thread::ThreadId,
}

// SAFETY: every use goes through `lend` / `call`, which check the thread.
unsafe impl<E: ?Sized + 'static> Send for EntryAdapter<E> {}

impl<E: RowHalf + ?Sized + 'static> EntryAdapter<E> {
    /// `addEntry(e)`: the row's widgets move to the adapter (the game draws and clicks them there).
    pub fn new(object: Shared<E>) -> EntryAdapter<E> {
        let row = write(&object, |t| {
            std::mem::replace(t.row_parts().0, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()))
        });
        EntryAdapter { object, row, owner: std::thread::current().id() }
    }

    pub fn object(&self) -> &Shared<E> {
        &self.object
    }

    /// One of the row object's methods with its widgets (and, when the game hands it, its row box) lent to it.
    pub fn lend<R>(
        &mut self,
        entry: Option<&mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry>,
        f: impl FnOnce(&Shared<E>) -> R,
    ) -> R {
        assert!(std::thread::current().id() == self.owner, "a mod list row was reached off the render thread");
        if let Some(entry) = entry {
            write(&self.object, |t| *t.entry_half() = entry.clone());
        }
        lend_row(&self.object, &mut self.row, || f(&self.object))
    }
}

/// The row's widgets lent to the row object for `f` (swapped into its `__row`, back after - also on unwind).
pub fn lend_row<E: RowHalf + ?Sized, R>(
    object: &Shared<E>,
    row: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, E: RowHalf + ?Sized>(&'a Shared<E>, &'a mut mcrust::net::minecraft::client::gui::screens::screen::Screen);
    impl<E: RowHalf + ?Sized> Drop for Back<'_, E> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| {
                let (screen, cache) = t.row_parts();
                cache.end(screen);
                std::mem::swap(screen, lent);
            });
        }
    }
    write(object, |t| {
        let (screen, cache) = t.row_parts();
        std::mem::swap(screen, row);
        cache.begin(screen);
    });
    let _back = Back(object, row);
    f()
}

/// A mod class that extends the game's `Slot`: its slot half (`base`).
pub trait SlotHalf {
    fn slot_half(&mut self) -> &mut mcrust::net::minecraft::world::inventory::slot::Slot;
}

/// A mod's `Slot` subclass as the port's slot class (`container_menus::ModSlot`): its handle, asked with the
/// slot's current state (copied into the object's `base`, so `this.y` reads it) and the base class's answer.
pub struct SlotClass<C: 'static> {
    object: RenderThread<Shared<C>>,
}

impl<C: SlotHalf + 'static> SlotClass<C> {
    pub fn new(object: Shared<C>) -> SlotClass<C> {
        SlotClass { object: RenderThread::new(object) }
    }

    pub fn object(&self) -> &Shared<C> {
        self.object.get()
    }

    pub fn ask<T: Clone + 'static, R>(
        &self,
        slot: &mcrust::net::minecraft::world::inventory::slot::Slot,
        super_answer: T,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        let object = self.object.get();
        write(object, |t| *t.slot_half() = slot.clone());
        SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(super_answer)));
        struct Pop;
        impl Drop for Pop {
            fn drop(&mut self) {
                SUPER_ANSWER.with(|s| {
                    s.borrow_mut().pop();
                });
            }
        }
        let _pop = Pop;
        f(object)
    }
}

/// `new MySlot(container, index, x, y)` handed to the game (`slots.set(i, slot)`): the game's slot, built from
/// the object's own fields, with the object attached as its class.
pub fn slot_with_class<C: SlotHalf + 'static>(object: Shared<C>) -> mcrust::net::minecraft::world::inventory::slot::Slot
where
    SlotClass<C>: mcrust::mcrust::modding::container_menus::ModSlot,
{
    let slot = write(&object, |t| t.slot_half().clone());
    slot.with_mod_slot(std::sync::Arc::new(SlotClass::new(object)))
}

/// An anonymous `new Slot(..) { boolean isActive() {..} }`: its overrides as closures over the slot (Java's
/// `this`) and the base class's answer.
#[derive(Default)]
pub struct SlotFns {
    pub is_active: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub is_highlightable: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub may_pickup: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub get_max_stack_size: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, i32) -> i32 + Send + Sync>>,
}

impl mcrust::mcrust::modding::container_menus::ModSlot for SlotFns {
    fn is_active(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.is_active.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn is_highlightable(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.is_highlightable.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn may_pickup(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.may_pickup.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn get_max_stack_size(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: i32) -> i32 {
        self.get_max_stack_size.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Mod menus (`class MyHandler extends ChestMenu`) and mod container screens (`extends AbstractContainerScreen`)
// ---------------------------------------------------------------------------------------------------------------

/// Java's `player.getInventory()` handed to a mod menu's or mod container screen's constructor that only hands it
/// on to the game's base (`super(type, id, playerInventory, ..)`): the port builds that base over the menu and
/// screen the game opened, which hold the local player's inventory already - so the argument names it, it is not a
/// copy of it. (`game::with_player` reaches the inventory itself.)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayerInventory;

/// A layout the mod holds, lent where the port's function takes `&mut LayoutElement` (`centerInRectangle`): the
/// element is the layout, written back into the mod's value when the borrow ends (Java moves the object itself).
pub struct LayoutBack<'a> {
    element: mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement,
    grid: Option<&'a mut mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout>,
    frame: Option<&'a mut mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout>,
}

impl<'a> LayoutBack<'a> {
    pub fn grid(layout: &'a mut mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout) -> LayoutBack<'a> {
        let element = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement::Grid(layout.clone());
        LayoutBack { element, grid: Some(layout), frame: None }
    }

    pub fn frame(layout: &'a mut mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout) -> LayoutBack<'a> {
        let element = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement::Frame(layout.clone());
        LayoutBack { element, grid: None, frame: Some(layout) }
    }
}

impl std::ops::Deref for LayoutBack<'_> {
    type Target = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl std::ops::DerefMut for LayoutBack<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl Drop for LayoutBack<'_> {
    fn drop(&mut self) {
        use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
        match &self.element {
            LayoutElement::Grid(g) => {
                if let Some(t) = self.grid.as_deref_mut() {
                    *t = g.clone();
                }
            }
            LayoutElement::Frame(f) => {
                if let Some(t) = self.frame.as_deref_mut() {
                    *t = f.clone();
                }
            }
            _ => {}
        }
    }
}

/// A `ChestMenu` a mod keeps (`this.handler = handler`): Java holds the menu object; the port's open menu is the
/// player's, reached each time through `game::with_chest_menu` while it is still the one whose container id was
/// recorded (`None` once the server closed it or opened another - Java would read the closed object).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LiveChestMenu {
    pub container_id: i32,
}

impl LiveChestMenu {
    pub fn of(menu: &mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu) -> LiveChestMenu {
        LiveChestMenu { container_id: menu.menu.container_id }
    }

    pub fn with<R>(&self, f: impl FnOnce(&mut mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu) -> R) -> Option<R> {
        let id = self.container_id;
        mcrust::mcrust::modding::game::with_chest_menu(|chest| (chest.menu.container_id == id).then(|| f(chest))).flatten()
    }

    /// The menu's slots with the containers their `Slot.getItem()` reads (`with_open_menu`), while it is still the
    /// open one; `None` as for `with`.
    pub fn with_slots<R>(
        &self,
        f: impl FnOnce(
            &mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu,
            &mcrust::net::minecraft::world::inventory::slot::Containers<'_>,
        ) -> R,
    ) -> Option<R> {
        let id = self.container_id;
        with_open_menu(|menu, containers| (menu.container_id == id).then(|| f(menu, containers))).flatten()
    }
}

/// A mod class that extends the game's container menu: its menu half (`base`), the LIVE menu while the game lends
/// it (the constructor's body, `ModMenu::init`).
pub trait MenuHalf {
    fn menu_half(&mut self) -> &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
}

thread_local! {
    /// the constructor bodies of mod menu objects built but not yet attached, keyed by the object
    static PENDING_MENUS: std::cell::RefCell<Vec<(usize, Box<dyn FnOnce()>)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `new MyHandler(..)`: Java runs the constructor's body on the menu `super(..)` built; the port's menu is the one
/// the server opened, lent to the object when it is attached (`ModMenu::init`), so the body is kept until then.
/// Bodies of a `this(..)` chain run in Java's order (the delegate's first).
pub fn defer_menu_init<M: 'static>(object: &Shared<M>, body: impl FnOnce(&Shared<M>) + 'static) {
    let key = std::sync::Arc::as_ptr(object) as *const () as usize;
    let handle = object.clone();
    PENDING_MENUS.with(|p| p.borrow_mut().push((key, Box::new(move || body(&handle)))));
}

fn take_menu_inits<M: 'static>(object: &Shared<M>) -> Vec<Box<dyn FnOnce()>> {
    let key = std::sync::Arc::as_ptr(object) as *const () as usize;
    PENDING_MENUS.with(|p| {
        let mut p = p.borrow_mut();
        let mut out = Vec::new();
        let mut i = 0;
        while i < p.len() {
            if p[i].0 == key {
                out.push(p.remove(i).1);
            } else {
                i += 1;
            }
        }
        out
    })
}

/// A mod menu class's overrides of the menu's own answers (`canDragTo`, `canTakeItemForPickAll`), on its handle;
/// `super.x(..)` inside them is the answer the port hands (`super_answer`). The defaults are the base's.
pub trait MenuOverrides: Sized + 'static {
    fn can_drag_to(this: &Shared<Self>, slot: &mcrust::net::minecraft::world::inventory::slot::Slot) -> Option<bool> {
        let _ = (this, slot);
        None
    }

    fn can_take_item_for_pick_all(
        this: &Shared<Self>,
        carried: &mcrust::net::minecraft::world::item::item_stack::ItemStack,
        target: &mcrust::net::minecraft::world::inventory::slot::Slot,
    ) -> Option<bool> {
        let _ = (this, carried, target);
        None
    }
}

/// An override that reads `super.x()`: the base's answer, for `compat::super_answer` while `f` runs.
pub fn with_super_answer<T: Clone + 'static, R>(answer: T, f: impl FnOnce() -> R) -> R {
    SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(answer)));
    struct Pop;
    impl Drop for Pop {
        fn drop(&mut self) {
            SUPER_ANSWER.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
    let _pop = Pop;
    f()
}

/// A mod menu object as the game's `ModMenu`: its handle; `init` lends it the menu and runs its constructor's
/// body.
pub struct MenuAdapter<M: 'static> {
    object: Shared<M>,
    inits: Vec<Box<dyn FnOnce()>>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use is checked to the thread that made it (the render thread, where the game opens menus).
unsafe impl<M: 'static> Send for MenuAdapter<M> {}

/// `player.containerMenu = handler` (the factory's answer): the object, its constructor kept for `init`.
pub fn mod_menu<M: MenuHalf + MenuOverrides + 'static>(object: Shared<M>) -> Box<dyn mcrust::mcrust::modding::container_menus::ModMenu> {
    let inits = take_menu_inits(&object);
    Box::new(MenuAdapter { object, inits, owner: std::thread::current().id() })
}

impl<M: MenuHalf + 'static> MenuAdapter<M> {
    pub fn object(&self) -> &Shared<M> {
        &self.object
    }
}

/// The object's code with the live menu lent to it (swapped into `base`, back after - also on unwind).
pub fn lend_menu<M: MenuHalf, R>(
    object: &Shared<M>,
    menu: &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, M: MenuHalf>(&'a Shared<M>, &'a mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu);
    impl<M: MenuHalf> Drop for Back<'_, M> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| std::mem::swap(t.menu_half(), lent));
        }
    }
    write(object, |t| std::mem::swap(t.menu_half(), menu));
    let _back = Back(object, menu);
    f()
}

impl<M: MenuHalf + MenuOverrides + 'static> mcrust::mcrust::modding::container_menus::ModMenu for MenuAdapter<M> {
    fn init(&mut self, menu: &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu) {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        let inits = std::mem::take(&mut self.inits);
        lend_menu(&self.object, menu, || {
            for init in inits {
                init();
            }
        });
    }

    fn can_drag_to(&mut self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        with_super_answer(super_answer, || M::can_drag_to(&self.object, slot)).unwrap_or(super_answer)
    }

    fn can_take_item_for_pick_all(
        &mut self,
        carried: &mcrust::net::minecraft::world::item::item_stack::ItemStack,
        target: &mcrust::net::minecraft::world::inventory::slot::Slot,
        super_answer: bool,
    ) -> bool {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        with_super_answer(super_answer, || M::can_take_item_for_pick_all(&self.object, carried, target))
            .unwrap_or(super_answer)
    }
}

/// A mod class that extends the game's `AbstractContainerScreen`: its screen half (`base`).
pub trait ContainerHalf {
    fn container_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
}

/// A mod container screen object as the game's `ModContainerScreen`: its handle; each of its methods runs with
/// the game's screen lent to it.
pub struct ContainerScreenAdapter<C: 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use is checked to the thread that made it.
unsafe impl<C: 'static> Send for ContainerScreenAdapter<C> {}

impl<C: ContainerHalf + 'static> ContainerScreenAdapter<C> {
    pub fn new(object: Shared<C>) -> ContainerScreenAdapter<C> {
        ContainerScreenAdapter { object, owner: std::thread::current().id() }
    }

    pub fn object(&self) -> &Shared<C> {
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod container screen was reached off the render thread");
    }

    /// One of the object's methods with the game's screen lent to it (swapped into `base`, back after).
    pub fn lend<R>(
        &self,
        screen: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        struct Back<'a, C: ContainerHalf>(&'a Shared<C>, &'a mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen);
        impl<C: ContainerHalf> Drop for Back<'_, C> {
            fn drop(&mut self) {
                let lent = &mut *self.1;
                write(self.0, |t| std::mem::swap(t.container_half(), lent));
            }
        }
        write(&self.object, |t| std::mem::swap(t.container_half(), screen));
        let _back = Back(&self.object, screen);
        f(&self.object)
    }

    /// A method that only READS the game's screen (the extract passes): its geometry brought into the object.
    pub fn lend_ref<R>(
        &self,
        screen: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let half = t.container_half();
            half.left_pos = screen.left_pos;
            half.top_pos = screen.top_pos;
            half.image_width = screen.image_width;
            half.image_height = screen.image_height;
            half.title_label_x = screen.title_label_x;
            half.title_label_y = screen.title_label_y;
            half.inventory_label_x = screen.inventory_label_x;
            half.inventory_label_y = screen.inventory_label_y;
            half.hovered_slot = screen.hovered_slot;
            half.screen.width = screen.screen.width;
            half.screen.height = screen.screen.height;
            half.screen.title = screen.screen.title.clone();
        });
        f(&self.object)
    }
}

thread_local! {
    static SUPER_CALLS: std::cell::RefCell<Vec<*mut (dyn FnMut() + 'static)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// The base class's body of an override the port hands as a closure (`super_extract_background`), for
/// `super.extractBackground(..)` inside the override (`call_super`).
pub fn with_super<R>(sup: &mut dyn FnMut(), f: impl FnOnce() -> R) -> R {
    let p: *mut (dyn FnMut() + '_) = sup;
    // SAFETY: the pointer is popped before `with_super` returns (also on unwind), so it never outlives `sup`.
    let p: *mut (dyn FnMut() + 'static) = unsafe { std::mem::transmute(p) };
    SUPER_CALLS.with(|s| s.borrow_mut().push(p));
    struct Pop;
    impl Drop for Pop {
        fn drop(&mut self) {
            SUPER_CALLS.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
    let _pop = Pop;
    f()
}

/// `super.extractBackground(..)` (& co) inside an override the port hands its base body.
pub fn call_super() {
    let p = SUPER_CALLS.with(|s| s.borrow().last().copied());
    if let Some(p) = p {
        // SAFETY: `with_super` keeps the closure alive while the pointer is on the stack.
        unsafe { (*p)() }
    }
}

/// What a `MenuScreens.fromPacket` interception decided (`container_screens::register`'s factory): the screen
/// to open and the menu object over the server's menu.
#[derive(Default)]
pub struct ContainerResult {
    screen: Option<Box<dyn mcrust::mcrust::modding::container_screens::ModContainerScreen>>,
    plain: Option<Box<dyn mcrust::mcrust::modding::screens::ModScreen>>,
    menu: Option<Box<dyn mcrust::mcrust::modding::container_menus::ModMenu>>,
}

impl ContainerResult {
    pub fn container_screen(&mut self, screen: Box<dyn mcrust::mcrust::modding::container_screens::ModContainerScreen>) {
        self.screen = Some(screen);
    }

    pub fn mod_screen(&mut self, screen: Box<dyn mcrust::mcrust::modding::screens::ModScreen>) {
        self.plain = Some(screen);
    }

    pub fn menu(&mut self, menu: Box<dyn mcrust::mcrust::modding::container_menus::ModMenu>) {
        self.menu = Some(menu);
    }

    /// `ci.cancel()`: the answer.
    pub fn finish(&mut self) -> Option<mcrust::mcrust::modding::container_screens::ModContainer> {
        use mcrust::mcrust::modding::container_screens::ModContainer;
        if let Some(plain) = self.plain.take() {
            return Some(match self.menu.take() {
                Some(m) => ModContainer::mod_screen_and_menu(plain, m),
                None => ModContainer::mod_screen(plain),
            });
        }
        match (self.screen.take(), self.menu.take()) {
            (Some(s), Some(m)) => Some(ModContainer::screen_and_menu(s, m)),
            (Some(s), None) => Some(ModContainer::screen(s)),
            (None, Some(m)) => Some(ModContainer::menu(m)),
            (None, None) => None,
        }
    }
}

/// A mod class that extends the game's `Screen`: its `Screen` half (`base`, or the base of its base). The game
/// keeps the live half - the widget tree, `width`, `height`, `title` - and LENDS it to the object for each of
/// its methods (`ScreenAdapter::lend`), so the object's code reaches it through `base` exactly as Java's
/// `this.width` / `this.addRenderableWidget(..)` reach the superclass's fields.
pub trait ScreenHalf {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen;

    /// The `Screen` half and the object's widget references' cache (`WidgetCache`), together.
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut WidgetCache>) {
        (self.screen_half(), None)
    }
}

type ScreenWidget = mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
type ScreenTree = mcrust::net::minecraft::client::gui::screens::screen::Screen;

/// Java's reference to a widget of a mod screen. Before the widget is added to the screen it is a value the
/// object holds; once added (`addRenderableWidget`, a layout's `addChild`) the SCREEN holds it, by index, and
/// every access through this reference reaches the screen's widget - the one the player sees and clicks.
#[derive(Clone, Debug)]
pub enum WidgetRef {
    Value(Box<ScreenWidget>),
    Screen(usize),
}

impl WidgetRef {
    pub fn value(widget: ScreenWidget) -> WidgetRef {
        WidgetRef::Value(Box::new(widget))
    }
}

/// The screen's widgets as the object last saw them, for code that runs while the game has NOT lent the object
/// its `Screen` half (a button's action runs inside the game's own widget pass): it reads that copy - brought up
/// to date at the end of every call the game makes on the object, including the click that runs the action -
/// and what it changes is written into the screen's widget at the start of the next call (every tick).
#[derive(Default, Debug)]
pub struct WidgetCache {
    lent: bool,
    shadow: std::collections::HashMap<usize, ScreenWidget>,
    dirty: std::collections::HashSet<usize>,
}

impl WidgetCache {
    /// The game lent the object its `Screen` half: the changes made since go into the screen's widgets.
    pub fn begin(&mut self, screen: &mut ScreenTree) {
        for i in std::mem::take(&mut self.dirty) {
            if let Some(w) = self.shadow.get(&i) {
                if i < screen.children().len() {
                    *screen.widget_mut(i) = w.clone();
                }
            }
        }
        self.lent = true;
    }

    /// The call is over: the copies are brought up to date.
    pub fn end(&mut self, screen: &ScreenTree) {
        self.lent = false;
        for (i, w) in self.shadow.iter_mut() {
            if *i < screen.children().len() {
                *w = screen.widget(*i).clone();
            }
        }
    }

    /// One access through a widget reference (`button.active = false`, `field.getValue()`).
    pub fn with<R>(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef, f: impl FnOnce(&mut ScreenWidget) -> R) -> R {
        match r {
            WidgetRef::Value(w) => f(w),
            WidgetRef::Screen(i) => {
                let i = *i;
                if self.lent && i < screen.children().len() {
                    let out = f(screen.widget_mut(i));
                    self.shadow.insert(i, screen.widget(i).clone());
                    out
                } else {
                    let w = self.shadow.get_mut(&i).expect("a mod screen's widget reached before the screen was shown");
                    self.dirty.insert(i);
                    f(w)
                }
            }
        }
    }

    /// `addRenderableWidget(w)` / `addWidget(w)`: the screen takes the widget; the reference now reaches it there.
    pub fn add(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef, renderable: bool) -> usize {
        match r {
            WidgetRef::Screen(i) => {
                if renderable && self.lent {
                    screen.mark_renderable(*i);
                }
                *i
            }
            WidgetRef::Value(w) => {
                let w = (**w).clone();
                let i = if renderable { screen.add_renderable_widget(w) } else { screen.add_widget(w) };
                self.shadow.insert(i, screen.widget(i).clone());
                *r = WidgetRef::Screen(i);
                i
            }
        }
    }

    /// `layout.addChild(w)`: the widget joins the screen's list (a layout holds it by index), and its size.
    pub fn add_to_layout(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef) -> (usize, i32, i32) {
        let i = self.add(screen, r, false);
        let w = screen.widget(i);
        (i, w.get_width(), w.get_height())
    }
}

/// A mod's screen object as the game's `ModScreen`: the object's handle (the same object the mod's other
/// references hold). The game calls it on the render thread only - checked on every call - which is why it
/// may be `Send` although the mod's objects are not.
/// The game's `Screen` half lent to a mod screen object for `f` - one of its methods the game calls, or a button
/// action, which the game runs with the whole screen in hand: swapped into the object's `base` for the call and
/// back after (also on unwind), the object's widget references reaching the live widgets meanwhile; the object
/// keeps the last size and title for code that runs between calls.
pub fn lend_screen<C: ScreenHalf, R>(
    object: &Shared<C>,
    screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, C: ScreenHalf>(&'a Shared<C>, &'a mut mcrust::net::minecraft::client::gui::screens::screen::Screen);
    impl<C: ScreenHalf> Drop for Back<'_, C> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| {
                if let (screen, Some(cache)) = t.screen_parts() {
                    cache.end(screen);
                }
                std::mem::swap(t.screen_half(), lent);
                let half = t.screen_half();
                half.width = lent.width;
                half.height = lent.height;
                half.title = lent.title.clone();
            });
        }
    }
    write(object, |t| {
        std::mem::swap(t.screen_half(), screen);
        if let (screen, Some(cache)) = t.screen_parts() {
            cache.begin(screen);
        }
    });
    let _back = Back(object, screen);
    f()
}

pub struct ScreenAdapter<C: 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use goes through `check`, which pins the adapter to the thread that made it (the render
// thread, the only one the game calls a screen on).
unsafe impl<C: 'static> Send for ScreenAdapter<C> {}

impl<C: ScreenHalf + 'static> ScreenAdapter<C> {
    pub fn new(object: Shared<C>) -> ScreenAdapter<C> {
        ScreenAdapter { object, owner: std::thread::current().id() }
    }

    /// The mod object (`minecraft.screen instanceof C c` binds this).
    pub fn object(&self) -> &Shared<C> {
        self.check();
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod screen was reached off the render thread");
    }

    /// One of the object's methods with the game's `Screen` half lent to it (swapped into its `base` for the
    /// call, swapped back after - also on unwind); the object keeps the last size and title for code that runs
    /// between calls (a button's action).
    pub fn lend<R>(
        &self,
        screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        lend_screen(&self.object, screen, || f(&self.object))
    }

    /// A method that only READS the game's half (`extractRenderState`): the size is brought up to date first.
    pub fn lend_ref<R>(
        &self,
        screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let half = t.screen_half();
            half.width = screen.width;
            half.height = screen.height;
            if let (_, Some(cache)) = t.screen_parts() {
                cache.end(screen);
            }
        });
        f(&self.object)
    }

    /// A method that does not touch the `Screen` half (`removed`, `onClose`, `shouldCloseOnEsc`).
    pub fn call<R>(&self, f: impl FnOnce(&Shared<C>) -> R) -> R {
        self.check();
        f(&self.object)
    }

    /// `getTitle()` - the title the constructor gave `super(title)`.
    pub fn title(&self) -> String {
        self.check();
        write(&self.object, |t| t.screen_half().title.clone())
    }
}

pub fn shared<T>(value: T) -> Shared<T> {
    std::sync::Arc::new(parking_lot::ReentrantMutex::new(std::cell::RefCell::new(value)))
}


/// `System.identityHashCode(stack)`: a number that belongs to the Java OBJECT, kept as a per-object attachment
/// (`ItemStack::duplicate` hands the same object on and shares it; a `copy()` is a new object and gets a new one).
/// Unique per object where Java's may collide - the readers only use it as a cache key.
pub fn identity_hash_code(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i32 {
    use mcrust::mcrust::modding::attach::{AttachmentTarget as _, AttachmentType};
    static IDENTITY: AttachmentType<i32> = AttachmentType::new("fabric2seer:identity_hash_code");
    static NEXT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);
    stack.get_attached_or_create(&IDENTITY, || NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

/// `Minecraft.getInstance().screen instanceof AbstractContainerScreen` from anywhere (a tooltip callback, a tick):
/// the kind the game publishes once a frame (`gui::current_screen_kind`), which names every container screen
/// (`InventoryScreen`, `ContainerScreen`, the furnaces, ..., `CreativeModeInventoryScreen` - all
/// `AbstractContainerScreen`s; a mod's container screen rides on the vanilla one it replaced).
pub fn container_screen_open() -> bool {
    use mcrust::net::minecraft::client::gui::gui::ScreenKind;
    // `gui.screen_kind` is the tag `setScreen` writes before the new screen's `init` (so `ScreenEvents.AFTER_INIT`
    // sees its own screen); before the first client tick only the published copy exists.
    let p = MINECRAFT.load(std::sync::atomic::Ordering::Acquire);
    let kind = if p.is_null() {
        mcrust::net::minecraft::client::gui::gui::current_screen_kind()
    } else {
        Some(minecraft().gui.screen_kind)
    };
    matches!(
        kind,
        Some(
            ScreenKind::Inventory
                | ScreenKind::Container
                | ScreenKind::Hopper
                | ScreenKind::ShulkerBox
                | ScreenKind::Dispenser
                | ScreenKind::BrewingStand
                | ScreenKind::Furnace
                | ScreenKind::BlastFurnace
                | ScreenKind::Smoker
                | ScreenKind::Grindstone
                | ScreenKind::Crafting
                | ScreenKind::Anvil
                | ScreenKind::Smithing
                | ScreenKind::Stonecutter
                | ScreenKind::Loom
                | ScreenKind::Enchantment
                | ScreenKind::Crafter
                | ScreenKind::Beacon
                | ScreenKind::Merchant
                | ScreenKind::HorseInventory
                | ScreenKind::NautilusInventory
                | ScreenKind::CartographyTable
                | ScreenKind::CreativeInventory
        )
    )
}

/// JOML's `toString()` of a vector (`Vector2i`, `Vector2d`, `Vector3d` ..., joml 1.10.8): `"(" + x + " " + y + ")"`
/// with each component in `Options.NUMBER_FORMAT` (`0.000E0`) as `Runtime.formatNumbers` writes it - a positive
/// number gets a leading space and the exponent its sign: `( 3.000E+0 -1.200E+1)`.
pub fn joml_string(components: &[f64]) -> String {
    let parts: Vec<String> = components
        .iter()
        .map(|v| {
            let s = format!("{:.3e}", v);
            let (mantissa, exponent) = s.split_once('e').unwrap_or((&s, "0"));
            let exponent: i32 = exponent.parse().unwrap_or(0);
            let mantissa = if *v >= 0.0 { format!(" {mantissa}") } else { mantissa.to_string() };
            format!("{mantissa}E{}{}", if exponent < 0 { "-" } else { "+" }, exponent.abs())
        })
        .collect();
    format!("({})", parts.join(" "))
}

/// `player.containerMenu`'s slots together with the containers their `Slot.getItem()` reads (the port's
/// `Slot::get_item` takes them): `f(menu, containers)`. `None` where `game::with_player` answers `None` (no
/// player; or this thread holds the player - inside a `ModMenu` method or a container screen's `slot_clicked`).
pub fn with_open_menu<R>(
    f: impl FnOnce(
        &mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu,
        &mcrust::net::minecraft::world::inventory::slot::Containers<'_>,
    ) -> R,
) -> Option<R> {
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    mcrust::mcrust::modding::game::with_player(|p| {
        let (menu, inventory_menu, inventory) = p.player_mut().get_container_menu_and_inventory();
        let (menu, containers) = menu.slots_and_containers(inventory_menu, inventory);
        f(menu, &containers)
    })
}

/// `slot.getItem()` of a slot of the player's open menu: the SAME stack (`duplicate` keeps Java's identity, so
/// `stack == slot.getItem()` holds). EMPTY where `with_open_menu` answers `None`.
pub fn slot_item(
    slot: &mcrust::net::minecraft::world::inventory::slot::Slot,
) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
    with_open_menu(|_, containers| slot.get_item(containers).duplicate())
        .unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty)
}

/// `slot.hasItem()` of a slot of the player's open menu (see `slot_item`).
pub fn slot_has_item(slot: &mcrust::net::minecraft::world::inventory::slot::Slot) -> bool {
    with_open_menu(|_, containers| slot.has_item(containers)).unwrap_or(false)
}

/// A key category by id: vanilla's, or the mod's own (registered on first use,
/// `KeyMapping.Category.register`).
pub fn category(id: &str) -> mcrust::net::minecraft::client::key_mapping::Category {
    use mcrust::net::minecraft::client::key_mapping::Category;
    let Some(ident) = mcrust::net::minecraft::resources::identifier::Identifier::try_parse(id) else {
        return Category::Misc;
    };
    Category::by_id(&ident).unwrap_or_else(|| {
        crate::prelude::key_binding_helper::register_category(id).unwrap_or(Category::Misc)
    })
}

/// One argument of `String.format` (`java.util.Formatter`): an integral, a floating-point or any other value (its
/// `toString`). A `float` widens to `double`, as `Formatter.print(float, Locale)` does.
#[derive(Clone, Debug)]
pub enum FormatArg {
    Int(i64),
    Float(f64),
    Str(String),
}

impl From<i32> for FormatArg {
    fn from(v: i32) -> Self {
        FormatArg::Int(v as i64)
    }
}
impl From<i64> for FormatArg {
    fn from(v: i64) -> Self {
        FormatArg::Int(v)
    }
}
impl From<f32> for FormatArg {
    fn from(v: f32) -> Self {
        FormatArg::Float(v as f64)
    }
}
impl From<f64> for FormatArg {
    fn from(v: f64) -> Self {
        FormatArg::Float(v)
    }
}
impl From<&str> for FormatArg {
    fn from(v: &str) -> Self {
        FormatArg::Str(v.to_string())
    }
}
impl From<String> for FormatArg {
    fn from(v: String) -> Self {
        FormatArg::Str(v)
    }
}

/// `String.format(Locale.ENGLISH / Locale.US / Locale.ROOT, format, args)` - `java.util.Formatter`'s
/// `%[argument_index$][flags][width][.precision]conversion` for the conversions `d s f e g % n` and the flags
/// `- + space 0 ,`. A floating-point conversion rounds the value's shortest decimal digits (`Double.toString`'s)
/// HALF_UP, as `sun.misc.FormattedFloatingDecimal` does - not the binary value, as C's `printf` would.
pub fn string_format(format: &str, args: &[FormatArg]) -> String {
    let chars: Vec<char> = format.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut ordinary = 0usize;
    while i < chars.len() {
        if chars[i] != '%' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        i += 1;
        // argument_index$
        let mut j = i;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
        let mut index: Option<usize> = None;
        if j > i && j < chars.len() && chars[j] == '$' {
            index = chars[i..j].iter().collect::<String>().parse::<usize>().ok().map(|n| n - 1);
            i = j + 1;
        }
        let mut flags = String::new();
        while i < chars.len() && "-+ 0,#(".contains(chars[i]) {
            flags.push(chars[i]);
            i += 1;
        }
        let mut width: Option<usize> = None;
        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        if i > start {
            width = chars[start..i].iter().collect::<String>().parse().ok();
        }
        let mut precision: Option<usize> = None;
        if i < chars.len() && chars[i] == '.' {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            precision = chars[start..i].iter().collect::<String>().parse().ok();
        }
        let conversion = chars.get(i).copied().expect("UnknownFormatConversionException");
        i += 1;
        if conversion == '%' {
            out.push('%');
            continue;
        }
        if conversion == 'n' {
            out.push('\n');
            continue;
        }
        let arg = match index {
            Some(n) => args.get(n),
            None => {
                ordinary += 1;
                args.get(ordinary - 1)
            }
        }
        .expect("MissingFormatArgumentException");
        let grouping = flags.contains(',');
        let body = match (conversion.to_ascii_lowercase(), arg) {
            ('d', FormatArg::Int(v)) => format_sign(&group_digits(&v.unsigned_abs().to_string(), grouping), *v < 0, &flags),
            ('f', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| format_sign(&group_digits_fixed(&fixed_digits(d, precision.unwrap_or(6)), grouping), neg, &flags)),
            ('e', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| format_sign(&scientific_digits(d, precision.unwrap_or(6)), neg, &flags)),
            ('g', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| {
                let p = match precision {
                    None => 6,
                    Some(0) => 1,
                    Some(p) => p,
                };
                format_sign(&general_digits(d, p, grouping), neg, &flags)
            }),
            ('s', FormatArg::Str(s)) => s.clone(),
            ('s', FormatArg::Int(v)) => v.to_string(),
            ('s', FormatArg::Float(v)) => mcrust::javacompat::java_double_to_string(*v),
            _ => panic!("IllegalFormatConversionException: {} != {:?}", conversion, arg),
        };
        let body = if conversion.is_ascii_uppercase() { body.to_uppercase() } else { body };
        let body: String = match precision {
            Some(p) if conversion.to_ascii_lowercase() == 's' => body.chars().take(p).collect(),
            _ => body,
        };
        match width {
            Some(w) if body.chars().count() < w => {
                let pad = w - body.chars().count();
                if flags.contains('-') {
                    out.push_str(&body);
                    out.push_str(&" ".repeat(pad));
                } else if flags.contains('0') {
                    let sign_len = if body.starts_with(['+', '-', ' ']) { 1 } else { 0 };
                    out.push_str(&body[..sign_len]);
                    out.push_str(&"0".repeat(pad));
                    out.push_str(&body[sign_len..]);
                } else {
                    out.push_str(&" ".repeat(pad));
                    out.push_str(&body);
                }
            }
            _ => out.push_str(&body),
        }
    }
    out
}

/// The shortest decimal digits of `|v|` (`Double.toString`'s) and the decimal exponent of the first:
/// `|v| = d1.d2d3.. * 10^exp`.
fn shortest_digits(v: f64) -> (Vec<u8>, i32) {
    let s = format!("{:e}", v.abs());
    let (mantissa, exp) = s.split_once('e').unwrap();
    let digits: Vec<u8> = mantissa.bytes().filter(|b| b.is_ascii_digit()).map(|b| b - b'0').collect();
    (digits, exp.parse::<i32>().unwrap())
}

/// Rounds `digits` HALF_UP to `keep` digits; a carry out of the first digit raises `exp`. `keep < 0` is zero.
fn round_half_up(mut digits: Vec<u8>, mut exp: i32, keep: i32) -> (Vec<u8>, i32) {
    if keep < 0 {
        return (vec![0], exp);
    }
    let keep = keep as usize;
    if digits.len() > keep {
        let round_up = digits[keep] >= 5;
        digits.truncate(keep);
        if round_up {
            let mut k = keep;
            loop {
                if k == 0 {
                    digits.insert(0, 1);
                    exp += 1;
                    break;
                }
                k -= 1;
                if digits[k] == 9 {
                    digits[k] = 0;
                } else {
                    digits[k] += 1;
                    break;
                }
            }
        }
    }
    (digits, exp)
}

fn format_float(v: f64, flags: &str, finite: impl Fn(f64, bool) -> String) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return format_sign("Infinity", v < 0.0, flags);
    }
    finite(v, v.is_sign_negative() && v != 0.0)
}

fn format_sign(body: &str, negative: bool, flags: &str) -> String {
    if negative {
        format!("-{}", body)
    } else if flags.contains('+') {
        format!("+{}", body)
    } else if flags.contains(' ') {
        format!(" {}", body)
    } else {
        body.to_string()
    }
}

/// `%.{precision}f` of `|v|`.
fn fixed_digits(v: f64, precision: usize) -> String {
    if v == 0.0 {
        return if precision == 0 { "0".to_string() } else { format!("0.{}", "0".repeat(precision)) };
    }
    let (digits, exp) = shortest_digits(v);
    // exp + 1 digits sit before the point; keep those and `precision` more
    let (digits, exp) = round_half_up(digits, exp, exp + 1 + precision as i32);
    let mut int_part = String::new();
    for k in 0..(exp + 1).max(0) {
        int_part.push((b'0' + digits.get(k as usize).copied().unwrap_or(0)) as char);
    }
    if int_part.is_empty() {
        int_part.push('0');
    }
    let mut frac_part = String::new();
    for k in 0..precision as i32 {
        let pos = exp + 1 + k;
        let d = if pos >= 0 { digits.get(pos as usize).copied().unwrap_or(0) } else { 0 };
        frac_part.push((b'0' + d) as char);
    }
    if precision == 0 { int_part } else { format!("{}.{}", int_part, frac_part) }
}

/// `%.{precision}e` of `|v|`: `d.ddde+xx`.
fn scientific_digits(v: f64, precision: usize) -> String {
    let (digits, exp) = if v == 0.0 {
        (vec![0], 0)
    } else {
        let (d, e) = shortest_digits(v);
        round_half_up(d, e, precision as i32 + 1)
    };
    let mut s = String::new();
    s.push((b'0' + digits[0]) as char);
    if precision > 0 {
        s.push('.');
        for k in 1..=precision {
            s.push((b'0' + digits.get(k).copied().unwrap_or(0)) as char);
        }
    }
    s.push('e');
    s.push(if exp < 0 { '-' } else { '+' });
    s.push_str(&format!("{:02}", exp.abs()));
    s
}

/// `%.{precision}g` of `|v|`: rounded to `precision` significant digits, then fixed when
/// `10^-4 <= x < 10^precision`, scientific otherwise.
fn general_digits(v: f64, precision: usize, grouping: bool) -> String {
    if v == 0.0 {
        return fixed_digits(0.0, precision - 1);
    }
    let (digits, exp) = shortest_digits(v);
    let (_, rounded_exp) = round_half_up(digits, exp, precision as i32);
    if rounded_exp >= -4 && rounded_exp < precision as i32 {
        group_digits_fixed(&fixed_digits(v, (precision as i32 - 1 - rounded_exp).max(0) as usize), grouping)
    } else {
        scientific_digits(v, precision - 1)
    }
}

/// The `,` flag: the integer digits in groups of three (`Locale.ENGLISH`'s separator).
fn group_digits(int_part: &str, grouping: bool) -> String {
    if !grouping {
        return int_part.to_string();
    }
    let chars: Vec<char> = int_part.chars().collect();
    let mut out = String::new();
    for (k, c) in chars.iter().enumerate() {
        if k > 0 && (chars.len() - k) % 3 == 0 {
            out.push(',');
        }
        out.push(*c);
    }
    out
}

fn group_digits_fixed(fixed: &str, grouping: bool) -> String {
    match fixed.split_once('.') {
        Some((int_part, frac)) => format!("{}.{}", group_digits(int_part, grouping), frac),
        None => group_digits(fixed, grouping),
    }
}

/// `Integer.decode(nm)`: an optional sign, then `0x` / `0X` / `#` (hexadecimal), a leading `0` (octal) or decimal
/// digits. A malformed string is Java's `NumberFormatException`.
pub fn integer_decode(nm: &str) -> i32 {
    let (negative, rest) = match nm.as_bytes().first() {
        Some(b'-') => (true, &nm[1..]),
        Some(b'+') => (false, &nm[1..]),
        _ => (false, nm),
    };
    let (radix, digits) = if let Some(d) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")).or_else(|| rest.strip_prefix('#')) {
        (16, d)
    } else if rest.len() > 1 && rest.starts_with('0') {
        (8, &rest[1..])
    } else {
        (10, rest)
    };
    if digits.starts_with(['-', '+']) {
        panic!("NumberFormatException: Sign character in wrong position");
    }
    let signed = if negative { format!("-{}", digits) } else { digits.to_string() };
    i32::from_str_radix(&signed, radix).unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", nm))
}

// ---------------------------------------------------------------------------------------------------------------
// batch C (tabhud / profileviewer) helpers
// ---------------------------------------------------------------------------------------------------------------

/// `builder.create(x, y, width, height, name, (button, value) -> ..)` of a mod's `CycleButton`: the game's widget, of
/// the `CycleButton` kind, whose press steps the value (`OnPress::Cycle`, `CycleButton.onPress`) and then runs the
/// listener (`cycleValue:100`). The port's cycle button holds the value LABELS, so the listener is handed the new
/// value's index in the builder's value list (`withValues(..)`'s order) and maps it back to its value.
pub fn cycle_button_widget(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    name: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent,
    builder: mcrust::net::minecraft::client::gui::components::cycle_button::Builder,
    on_value_change: impl Fn(usize) + 'static,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
    use mcrust::net::minecraft::client::gui::components::cycle_button::{CycleButton, OnValueChange, ValueChangeLambda, GENERIC_VALUE_KEY};
    use mcrust::net::minecraft::network::chat::component::Component as _;
    let listener = RenderThread::new(on_value_change);
    let generic_value_format = mcrust::net::minecraft::locale::language::get_or_default(GENERIC_VALUE_KEY);
    let name = name.get_string();
    let button = builder
        .create(
            name.clone(),
            generic_value_format,
            OnValueChange::Lambda(ValueChangeLambda::new(move |button: &mut CycleButton, _value: &str| {
                (listener.get())(button.index())
            })),
        )
        .unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
    AbstractWidget::new(x, y, width, height, name, WidgetKind::CycleButton(button), OnPress::Cycle)
}

// ===========================================================================
// batch D (mixins, dwarven, waypoint)
// ===========================================================================

/// A hooked method's parameter whose type carries a lifetime (`containers: &mut Containers<'_>`,
/// `host: &mut dyn ContainerScreenHost`), which the safe `ci.arg_mut` cannot hand out (MODDING.md section 8,
/// "Typed access rules": read it with `arg_unchecked_mut`), detached from `ci` as `detach` does.
///
/// SAFETY (`detach`'s rule plus `arg_unchecked_mut`'s): `T` must be the parameter's real pointee type, and the
/// reference is valid only during the callback that produced it. Never store it.
pub fn hook_arg_mut<'a, T: ?Sized>(
    ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>,
    index: usize,
) -> Option<&'a mut T> {
    unsafe { ci.arg_unchecked_mut::<T>(index) }.map(|p| unsafe { &mut *(p as *mut T) })
}

/// `hook_arg_mut` for a shared parameter (`lines: &[Box<dyn ClientTooltipComponent>]`).
pub fn hook_arg<'a, T: ?Sized>(ci: &mcrust::mcrust::modding::hook::CallbackInfo<'_>, index: usize) -> Option<&'a T> {
    unsafe { ci.arg_unchecked::<T>(index) }.map(|p| unsafe { &*(p as *const T) })
}

/// `(Object) menu instanceof ChestMenu` for the `AbstractContainerMenu` half a hook hands over (the port flattens
/// `ChestMenu.menu`): a chest menu's grid is the only one whose slots name `SlotContainer::Chest`
/// (`ChestMenu:10`'s container; every other menu kind has its own tag since chunk 350), and its slot 0 is a grid slot.
pub fn is_chest_menu(menu: &mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu) -> bool {
    menu.slots
        .first()
        .is_some_and(|slot| slot.container == mcrust::net::minecraft::world::inventory::slot::SlotContainer::Chest)
}

/// java: `FlexibleItemStack implements ItemInstance` - `typeHolder()` (`TypedInstance`) and `count()`, the two members the
/// interface leaves abstract (`ItemUtils.getIcon(ItemInstance)` reads them off a `FlexibleItemStack`).
impl mcrust::net::minecraft::core::typed_instance::TypedInstance for crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack {
    fn type_holder(&self) -> mcrust::net::minecraft::core::holder::Holder {
        self.item.clone().expect("FlexibleItemStack.item is final and set by every constructor")
    }
}

impl mcrust::net::minecraft::world::item::item_instance::ItemInstance for crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack {
    fn count(&self) -> i32 {
        self.count
    }
}

thread_local! {
    /// The mod's `PlayerInfo` objects, one per online profile, kept across updates (Java keeps the connection's own
    /// objects; see `online_player_infos`).
    static ONLINE_PLAYER_INFOS: std::cell::RefCell<std::collections::HashMap<mcrust::java::util::uuid::Uuid, Shared<mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// `connection.getOnlinePlayers().stream().sorted(PlayerTabOverlayAccessor.getOrdering()).toList()` on the render
/// thread. Java hands out the connection's live `PlayerInfo` objects; the port's `playerInfoMap` belongs to the network
/// side and the render thread sees its mirrors (`online_players`, `tab_list_players`), so each online profile has ONE
/// `PlayerInfo` here, kept across calls (the same object while the player stays online, as Java's is) and brought up
/// to date from the mirrors at every call: the tab-list display name, order, game mode, latency and hat flag. An
/// unlisted player has no tab-list row and keeps its last values. The order is `PlayerTabOverlay.PLAYER_COMPARATOR`
/// (`:55-58`), the team name read off the connection's scoreboard as the overlay reads it.
pub fn online_player_infos(
    sinks: &mcrust::net::minecraft::client::multiplayer::client_packet_listener::PlayListenerSinks,
) -> Vec<Shared<mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo>> {
    use mcrust::net::minecraft::client::gui::components::player_tab_overlay::{compare_players, PlayerSortKey};
    use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
    let mut keyed: Vec<(PlayerSortKey, Shared<PlayerInfo>)> = Vec::new();
    {
        let online = sinks.online_players.lock().unwrap_or_else(|p| p.into_inner());
        let listed = sinks.tab_list_players.lock().unwrap_or_else(|p| p.into_inner());
        ONLINE_PLAYER_INFOS.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.retain(|id, _| online.iter().any(|e| e.profile.id() == *id));
            for entry in online.iter() {
                let id = entry.profile.id();
                let info = cache
                    .entry(id)
                    .or_insert_with(|| shared(PlayerInfo::new(entry.profile.clone(), false)))
                    .clone();
                let tab = listed.iter().find(|t| t.profile_id == id);
                let (order, game_mode) = write(&info, |i| {
                    match tab {
                        Some(t) => {
                            i.set_tab_list_display_name(t.tab_list_display_name.clone());
                            i.set_tab_list_order(t.tab_list_order);
                            i.set_game_mode(t.game_mode);
                            i.set_latency(t.latency);
                            i.set_show_hat(t.show_hat);
                        }
                        None => i.set_game_mode(entry.skin.get_game_mode()),
                    }
                    (i.get_tab_list_order(), i.get_game_mode())
                });
                keyed.push((
                    PlayerSortKey {
                        tab_list_order: order,
                        game_mode,
                        team_name: String::new(),
                        profile_name: entry.profile.name().to_string(),
                    },
                    info,
                ));
            }
        });
    }
    {
        // :57 - `Optionull.mapOrDefault(info.getTeam(), PlayerTeam::getName, "")`
        let scoreboard = sinks.scoreboard.lock().unwrap_or_else(|p| p.into_inner());
        for (key, _) in keyed.iter_mut() {
            if let Some(team) = scoreboard.get_players_team(&key.profile_name) {
                key.team_name = team.get_name().to_string();
            }
        }
    }
    keyed.sort_by(|a, b| compare_players(&a.0, &b.0));
    keyed.into_iter().map(|(_, info)| info).collect()
}

/// `java.util.Random` with Java's own sequence: the 48-bit linear congruential generator (`new Random(seed)`), for
/// code whose output depends on the seed (a seeded preview).
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    const MULTIPLIER: i64 = 0x5DEECE66D;
    const ADDEND: i64 = 0xB;
    const MASK: i64 = (1i64 << 48) - 1;

    /// `new Random(seed)` - `initialScramble(seed)`.
    pub fn new_seeded(seed: i64) -> JavaRandom {
        JavaRandom { seed: (seed ^ Self::MULTIPLIER) & Self::MASK }
    }

    /// `next(bits)`.
    fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(Self::MULTIPLIER).wrapping_add(Self::ADDEND)) & Self::MASK;
        (self.seed >> (48 - bits)) as i32
    }

    /// `nextInt()`.
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    /// `ints(streamSize, origin, bound)` - each value `RandomSupport.boundedNextInt(this, origin, bound)` (JDK 17+).
    pub fn ints(&mut self, stream_size: usize, origin: i32, bound: i32) -> Vec<i32> {
        (0..stream_size).map(|_| self.bounded_next_int(origin, bound)).collect()
    }

    /// `RandomSupport.boundedNextInt(rng, origin, bound)`.
    fn bounded_next_int(&mut self, origin: i32, bound: i32) -> i32 {
        let mut r = self.next_int();
        if origin < bound {
            let n = bound.wrapping_sub(origin);
            let m = n.wrapping_sub(1);
            if n & m == 0 {
                r = (r & m).wrapping_add(origin);
            } else if n > 0 {
                let mut u = ((r as u32) >> 1) as i32;
                loop {
                    r = u % n;
                    if u.wrapping_add(m).wrapping_sub(r) >= 0 {
                        break;
                    }
                    u = ((self.next_int() as u32) >> 1) as i32;
                }
                r = r.wrapping_add(origin);
            } else {
                while r < origin || r >= bound {
                    r = self.next_int();
                }
            }
        }
        r
    }
}

/// `game::with_entity` for an entity the hook hands over as its own handle (`ClientLevel.addEntity(Entity)`,
/// `&Shared<StoredEntity>`), before or without an id lookup: runs `f` on it through the game's TRACKED lock, and
/// answers `None` - as `with_entity` does - when this thread already holds it (a second lock would deadlock).
/// The game has `with_entity` by id only (its by-handle body `with_entity_in` is private; GAPS_batchD).
pub fn with_entity_handle<R>(
    entity: &mcrust::net::minecraft::world::level::entity::uniquely_identifyable::Shared<
        mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity,
    >,
    f: impl FnOnce(&mut mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity) -> R,
) -> Option<R> {
    if mcrust::mcrust::modding::game::is_held_here(&**entity) {
        return None;
    }
    let mut guard = mcrust::mcrust::modding::game::lock_entity(&**entity, "skyblocker compat::with_entity_handle");
    Some(f(&mut *guard))
}

/// `EntityTypes.X` by its registry path (`"armor_stand"`): the port's types are a table, not constants.
pub fn entity_type(name: &str) -> std::sync::Arc<mcrust::net::minecraft::world::entity::entity_type::EntityType> {
    mcrust::net::minecraft::world::entity::entity_types::entity_types()
        .by_name(name)
        .unwrap_or_else(|| panic!("no entity type `{name}`"))
}

/// `entity.getType() == EntityTypes.X` (identity, as Java's `==` on the registered type).
pub fn is_entity_type(entity: &dyn mcrust::net::minecraft::world::entity::entity::Entity, name: &str) -> bool {
    entity.base().get_type().is_some_and(|t| std::sync::Arc::ptr_eq(t, &entity_type(name)))
}

/// `minecraft.execute(task)` for a task that holds the mod's objects (not `Send`): Java's `BlockableEventLoop.execute`
/// - inline on the render thread outside a running task, else queued for it (`game::execute`). Called on the render
/// thread (the task's cell is checked to it).
pub fn execute_local(task: impl FnOnce() + 'static) {
    let cell: RenderThread<std::cell::RefCell<Option<Box<dyn FnOnce()>>>> = RenderThread::new(std::cell::RefCell::new(Some(Box::new(task))));
    mcrust::mcrust::modding::game::execute(move || {
        let task = cell.get().borrow_mut().take();
        if let Some(task) = task {
            task();
        }
    });
}

/// A `CompletableFuture<T>` over the mod's own objects, which are not `Send` (the port's `CompletableFuture` needs
/// `Send` values). It is completed and continued on the render thread: the `*Async(.., minecraft)` stages run through
/// `execute_local` (the Minecraft executor), and `supply_local` runs the supplier there too - where Java would use a
/// worker thread for work over these objects, it runs deferred on the render thread instead.
pub struct LocalFuture<T: 'static> {
    state: std::rc::Rc<std::cell::RefCell<LocalFutureState<T>>>,
}

struct LocalFutureState<T> {
    result: Option<Result<T, String>>,
    callbacks: Vec<Box<dyn FnOnce(&Result<T, String>)>>,
}

impl<T: 'static> Clone for LocalFuture<T> {
    fn clone(&self) -> Self {
        LocalFuture { state: self.state.clone() }
    }
}

impl<T: Clone + 'static> LocalFuture<T> {
    /// `new CompletableFuture<>()`
    pub fn new() -> LocalFuture<T> {
        LocalFuture { state: std::rc::Rc::new(std::cell::RefCell::new(LocalFutureState { result: None, callbacks: Vec::new() })) }
    }

    /// `CompletableFuture.completedFuture(value)`
    pub fn completed_future(value: T) -> LocalFuture<T> {
        let future = LocalFuture::new();
        future.complete(value);
        future
    }

    /// `CompletableFuture.supplyAsync(supplier, executor)` over mod objects: the supplier runs on the render thread
    /// (`execute_local`); a panic is the exceptional completion
    pub fn supply_local(supplier: impl FnOnce() -> T + 'static) -> LocalFuture<T> {
        let future = LocalFuture::new();
        let target = future.clone();
        execute_local(move || match std::panic::catch_unwind(std::panic::AssertUnwindSafe(supplier)) {
            Ok(value) => {
                target.complete(value);
            }
            Err(e) => {
                target.complete_exceptionally(panic_message(&e));
            }
        });
        future
    }

    fn finish(&self, result: Result<T, String>) -> bool {
        let callbacks = {
            let mut state = self.state.borrow_mut();
            if state.result.is_some() {
                return false;
            }
            state.result = Some(result);
            std::mem::take(&mut state.callbacks)
        };
        let result = self.state.borrow().result.clone().expect("completed above");
        for callback in callbacks {
            callback(&result);
        }
        true
    }

    /// `complete(value)`
    pub fn complete(&self, value: T) -> bool {
        self.finish(Ok(value))
    }

    /// `completeExceptionally(ex)`
    pub fn complete_exceptionally(&self, error: String) -> bool {
        self.finish(Err(error))
    }

    /// `isDone()`
    pub fn is_done(&self) -> bool {
        self.state.borrow().result.is_some()
    }

    /// `getNow(valueIfAbsent)` (an exceptional completion is the `CompletionException` panic)
    pub fn get_now(&self, value_if_absent: T) -> T {
        match self.state.borrow().result.clone() {
            None => value_if_absent,
            Some(Ok(v)) => v,
            Some(Err(e)) => panic!("CompletionException: {e}"),
        }
    }

    /// `join()` of a completed future (the render thread cannot wait for itself: an incomplete one panics)
    pub fn join(&self) -> T {
        match self.state.borrow().result.clone() {
            None => panic!("LocalFuture.join() on the render thread before the future completed"),
            Some(Ok(v)) => v,
            Some(Err(e)) => panic!("CompletionException: {e}"),
        }
    }

    fn on_complete(&self, callback: impl FnOnce(&Result<T, String>) + 'static) {
        let done = self.state.borrow().result.clone();
        match done {
            Some(result) => callback(&result),
            None => self.state.borrow_mut().callbacks.push(Box::new(callback)),
        }
    }

    /// `thenApplyAsync(fn, minecraft)`
    pub fn then_apply_async<U: Clone + 'static>(&self, f: impl FnOnce(T) -> U + 'static) -> LocalFuture<U> {
        let next = LocalFuture::<U>::new();
        let target = next.clone();
        self.on_complete(move |result| {
            let result = result.clone();
            execute_local(move || match result {
                Ok(v) => match std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || f(v))) {
                    Ok(u) => {
                        target.complete(u);
                    }
                    Err(e) => {
                        target.complete_exceptionally(panic_message(&e));
                    }
                },
                Err(e) => {
                    target.complete_exceptionally(e);
                }
            });
        });
        next
    }

    /// `thenAcceptAsync(action, minecraft)`
    pub fn then_accept_async(&self, f: impl FnOnce(T) + 'static) -> LocalFuture<()> {
        self.then_apply_async(f)
    }

    /// `exceptionallyAsync(fn, minecraft)`: the value, or `fn(throwable)`'s answer for an exceptional completion
    pub fn exceptionally_async(&self, f: impl FnOnce(String) -> T + 'static) -> LocalFuture<T> {
        let next = LocalFuture::<T>::new();
        let target = next.clone();
        self.on_complete(move |result| {
            let result = result.clone();
            execute_local(move || match result {
                Ok(v) => {
                    target.complete(v);
                }
                Err(e) => {
                    target.complete(f(e));
                }
            });
        });
        next
    }
}

impl<T: Clone + 'static> LocalFuture<T> {
    /// `thenCombineAsync(other, fn)`: both values, once both futures completed (either's failure is the result's)
    pub fn then_combine_async<U: Clone + 'static, V: Clone + 'static>(&self, other: &LocalFuture<U>, f: impl FnOnce(T, U) -> V + 'static) -> LocalFuture<V> {
        let other = other.clone();
        let next = LocalFuture::<V>::new();
        let target = next.clone();
        self.on_complete(move |first| {
            let first = first.clone();
            other.on_complete(move |second| {
                let second = second.clone();
                execute_local(move || match (first, second) {
                    (Ok(a), Ok(b)) => match std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || f(a, b))) {
                        Ok(v) => {
                            target.complete(v);
                        }
                        Err(e) => {
                            target.complete_exceptionally(panic_message(&e));
                        }
                    },
                    (Err(e), _) | (_, Err(e)) => {
                        target.complete_exceptionally(e);
                    }
                });
            });
        });
        next
    }
}

/// `String.valueOf(optional)` / `"" + optional`: `Optional.toString()` - `Optional[value]` or `Optional.empty`.
pub fn optional_string<T: std::fmt::Display>(v: &Option<T>) -> String {
    match v {
        Some(v) => format!("Optional[{}]", v),
        None => "Optional.empty".to_string(),
    }
}

/// `LivingEntity.hasLineOfSight(Entity target)` (LivingEntity:3339-3355): the port has no body for it (GAPS_batchD),
/// so this is Java's, over the game's own parts - `ClipContext(from, to, COLLIDER, NONE, this)` and
/// `level.clip(..)` on the client's one level. `target.level() != this.level()` cannot be true on a client.
pub fn has_line_of_sight(
    this: &dyn mcrust::net::minecraft::world::entity::entity::Entity,
    target: &dyn mcrust::net::minecraft::world::entity::entity::Entity,
) -> bool {
    use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
    use mcrust::net::minecraft::world::level::clip_context::{self, ClipContext};
    use mcrust::net::minecraft::world::phys::vec3::Vec3;
    let Some(level) = mcrust::mcrust::modding::game::level() else {
        return false;
    };
    // `hasLineOfSight(target, COLLIDER, NONE, target.getEyeY())`
    let eye_height = target.base().get_eye_y();
    let from = Vec3::new(this.base().get_x(), this.base().get_eye_y(), this.base().get_z());
    let to = Vec3::new(target.base().get_x(), eye_height, target.base().get_z());
    if to.distance_to(&from) > 128.0 {
        return false;
    }
    let collision_context = mcrust::net::minecraft::world::phys::shapes::collision_context::of(this, &*level);
    let context = ClipContext::new(from, to, clip_context::Block::Collider, clip_context::Fluid::None, &*collision_context);
    level.clip(&context).get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Miss
}

/// Apache `WordUtils.capitalizeFully(str)`: every whitespace-delimited word lower-cased, then its first character
/// title-cased.
pub fn capitalize_fully(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut capitalize_next = true;
    for c in s.to_lowercase().chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            out.push(c);
        } else if capitalize_next {
            out.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// `ParticleTypes.X.equals(options.getType())`: the port erases `getType()` (`Obj`), so the registered type is
/// compared through `Object.equals` (identity for a `SimpleParticleType`, as in Java).
pub fn particle_type_is(
    options: &dyn mcrust::net::minecraft::core::particles::particle_options::ParticleOptions,
    particle_type: &std::sync::Arc<mcrust::net::minecraft::core::particles::simple_particle_type::SimpleParticleType>,
) -> bool {
    options.get_type().java_equals(&**particle_type)
}

/// `java.util.PriorityQueue<E>` with a `Comparator`: the JDK's array heap (`offer` = `siftUpUsingComparator`,
/// `poll` = `siftDownUsingComparator`), so iterating it (`new ArrayList<>(queue)`, `for (E e : queue)`) visits the
/// elements in the JDK's own array order.
pub struct JavaPriorityQueue<E> {
    queue: Vec<E>,
    comparator: Box<dyn Fn(&E, &E) -> i32>,
}

impl<E> JavaPriorityQueue<E> {
    /// `new PriorityQueue<>(comparator)`
    pub fn new(comparator: impl Fn(&E, &E) -> i32 + 'static) -> JavaPriorityQueue<E> {
        JavaPriorityQueue { queue: Vec::new(), comparator: Box::new(comparator) }
    }

    /// `offer(e)` / `add(e)`
    pub fn offer(&mut self, x: E) -> bool {
        self.queue.push(x);
        let mut k = self.queue.len() - 1;
        while k > 0 {
            let parent = (k - 1) >> 1;
            if (self.comparator)(&self.queue[k], &self.queue[parent]) >= 0 {
                break;
            }
            self.queue.swap(k, parent);
            k = parent;
        }
        true
    }

    /// `poll()` - the least element, or null when empty
    pub fn poll(&mut self) -> Option<E> {
        if self.queue.is_empty() {
            return None;
        }
        let n = self.queue.len() - 1;
        // `result = es[0]; x = es[n]; es[n] = null; siftDown(0, x)`
        self.queue.swap(0, n);
        let result = self.queue.pop();
        let mut k = 0usize;
        let half = n >> 1;
        while k < half {
            let mut child = (k << 1) + 1;
            let right = child + 1;
            if right < n && (self.comparator)(&self.queue[child], &self.queue[right]) > 0 {
                child = right;
            }
            if (self.comparator)(&self.queue[k], &self.queue[child]) <= 0 {
                break;
            }
            self.queue.swap(k, child);
            k = child;
        }
        result
    }

    /// `size()`
    pub fn size(&self) -> i32 {
        self.queue.len() as i32
    }

    /// iteration (`toArray()` order: the heap array)
    pub fn iter(&self) -> std::slice::Iter<'_, E> {
        self.queue.iter()
    }
}

/// `state.getValue(DIRECTION_PROPERTY)` (`FACING`, `HORIZONTAL_FACING`, ...): the port's state holds the erased
/// `Comparable`; a `Direction` property's value is its enum constant, by ordinal (`EnumProperty.getInternalIndex`).
pub fn direction_value(
    value: mcrust::net::minecraft::world::level::block::state::properties::property::Comparable,
) -> mcrust::net::minecraft::core::direction::Direction {
    use mcrust::net::minecraft::world::level::block::state::properties::property::Comparable;
    match value {
        Comparable::Enum(e) if e.class_name == "Direction" => mcrust::net::minecraft::core::direction::VALUES[e.ordinal as usize],
        other => panic!("ClassCastException: {} is not a Direction", other.to_java_string()),
    }
}

thread_local! {
    /// The panel of the last `AbstractContainerScreen` the game drew (`extractRenderState` HEAD, `register_hooks`):
    /// `leftPos`, `topPos`, `imageWidth`, `imageHeight`, and whether it is a `ContainerScreen` (the chest screen).
    static CONTAINER_PANEL: std::cell::Cell<Option<(i32, i32, i32, i32, bool)>> = const { std::cell::Cell::new(None) };
}

/// The hooks the compat layer itself needs (the `preLaunch` entrypoint calls this, with the mixins).
pub fn register_hooks() {
    use mcrust::mcrust::modding::hook::{self, At, InjectOptions, InjectorKind};
    // `AbstractContainerScreenAccessor.getX/getY/getImageWidth/getImageHeight` read by a widget the screen draws: the
    // screen object is the game's and is lent to the frame while it draws, so the panel is recorded as it starts.
    let result = hook::register_injector(
        "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractRenderState",
        InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000),
        std::sync::Arc::new(|ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            let Some(state) = ci.find_arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>() else {
                return;
            };
            let is_container_screen = ci
                .find_arg::<mcrust::net::minecraft::client::minecraft::Minecraft>()
                .is_some_and(|mc| mc.gui.screen_kind == mcrust::net::minecraft::client::gui::gui::ScreenKind::Container);
            CONTAINER_PANEL.set(Some((state.left_pos, state.top_pos, state.image_width, state.image_height, is_container_screen)));
        }),
    );
    if let Err(e) = result {
        mcrust::mcrust::modding::log::warn_once("skyblocker-compat-container-panel", &e.to_string());
    }
}

/// `ScreenUtils.getUnderlyingScreen() instanceof AbstractContainerScreen<?> s` and its accessor's
/// `getX()`, `getY()`, `getImageWidth()`, `getImageHeight()`, plus `s instanceof ContainerScreen`: the panel of the
/// container screen that is open (`None` when none is).
pub fn container_screen_panel() -> Option<(i32, i32, i32, i32, bool)> {
    if !container_screen_open() {
        return None;
    }
    CONTAINER_PANEL.get()
}

/// `AbstractButton.extractDefaultSprite(graphics)` + `extractDefaultLabel(..)` - what a plain `Button` draws - for a
/// mod's `Button` subclass that draws itself (`extractContents` calling the two defaults). The game draws a button of
/// `WidgetKind::Button` with exactly those bodies, so a one-widget `Screen` holding such a button, brought up to the
/// mod widget's box and state before each draw, is drawn through the game's own button pass. Kept in the object (no
/// allocation per frame unless the message changed).
pub struct ButtonLook {
    screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl std::fmt::Debug for ButtonLook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ButtonLook")
    }
}

impl Default for ButtonLook {
    fn default() -> ButtonLook {
        ButtonLook::new()
    }
}

impl ButtonLook {
    pub fn new() -> ButtonLook {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
        let mut screen = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        screen.add_renderable_widget(AbstractWidget::new(0, 0, 0, 0, String::new(), WidgetKind::Button, OnPress::Inert));
        ButtonLook { screen }
    }

    /// The button drawn over `widget`'s box, message, `active`, `visible`, alpha and focus (the hover is the pass's).
    pub fn draw(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        let b = self.screen.widget_mut(0);
        b.set_position(widget.get_x(), widget.get_y());
        b.width = widget.width;
        b.height = widget.height;
        if b.message != widget.message || b.message_text != widget.message_text {
            b.message = widget.message.clone();
            b.message_text = widget.message_text.clone();
        }
        b.message_style = widget.message_style.clone();
        b.active = widget.active;
        b.visible = widget.visible;
        b.alpha = widget.alpha;
        if b.is_focused() != widget.is_focused() {
            b.set_focused(widget.is_focused(), mcrust::net::minecraft::client::input_type::InputType::None);
        }
        context.extract_widgets(&mut self.screen, mouse_x, mouse_y);
    }

    /// `draw` with the message the mod object holds (a subclass that changes its own `message` - `setMessage(..)` -
    /// between the game's calls).
    pub fn draw_with_message(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
        context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        let b = self.screen.widget_mut(0);
        b.set_position(widget.get_x(), widget.get_y());
        b.width = widget.width;
        b.height = widget.height;
        if b.message_text.as_ref() != Some(message) {
            b.message = message.get_string();
            b.message_text = Some(message.clone());
        }
        b.active = widget.active;
        b.visible = widget.visible;
        b.alpha = widget.alpha;
        if b.is_focused() != widget.is_focused() {
            b.set_focused(widget.is_focused(), mcrust::net::minecraft::client::input_type::InputType::None);
        }
        context.extract_widgets(&mut self.screen, mouse_x, mouse_y);
    }

    /// `AbstractButton.extractDefaultSprite(graphics)` alone (a subclass that draws its own contents over the
    /// button's sprite): the button drawn with no label.
    pub fn draw_sprite(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        let b = self.screen.widget_mut(0);
        b.set_position(widget.get_x(), widget.get_y());
        b.width = widget.width;
        b.height = widget.height;
        if !b.message.is_empty() || b.message_text.is_some() {
            b.message = String::new();
            b.message_text = None;
        }
        b.active = widget.active;
        b.visible = widget.visible;
        b.alpha = widget.alpha;
        if b.is_focused() != widget.is_focused() {
            b.set_focused(widget.is_focused(), mcrust::net::minecraft::client::input_type::InputType::None);
        }
        context.extract_widgets(&mut self.screen, mouse_x, mouse_y);
    }
}

/// `new EditBox(font, x, y, width, height, message)`: the game's widget with an `EditBox` behind it (the port's
/// widgets are `AbstractWidget`s told apart by their kind; `message` is the box's narration message).
pub fn edit_box(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    message: impl IntoComponent,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    use mcrust::net::minecraft::network::chat::component::Component as _;
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
    let font = font();
    let edit = mcrust::net::minecraft::client::gui::components::edit_box::EditBox::new(&font.splitter(), x, y, width, height);
    let message = message.into_component();
    let mut widget = AbstractWidget::new(x, y, width, height, message.get_string(), WidgetKind::EditBox(edit), OnPress::Inert);
    widget.message_text = Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message));
    widget
}

/// The `EditBox` behind a widget made by `edit_box` (`None` for any other kind).
pub fn with_edit_box<R>(
    widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
    f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::components::edit_box::EditBox) -> R,
) -> Option<R> {
    match &mut widget.kind {
        mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(e) => Some(f(e)),
        _ => None,
    }
}

/// `editBox.getValue()`.
pub fn edit_box_value(widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> String {
    match &widget.kind {
        mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(e) => e.get_value(),
        _ => panic!("ClassCastException: the widget is not an EditBox"),
    }
}

/// `editBox.setValue(value)` (the port's takes the font's splitter).
pub fn set_edit_box_value(widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, value: &str) {
    let font = font();
    with_edit_box(widget, |e| e.set_value(&font.splitter(), value)).expect("ClassCastException: the widget is not an EditBox");
}

/// `ContainerEventHandler.keyPressed(event)` over widgets a mod object holds itself (a container row's
/// `children()`, a composite widget): `getFocused() != null && getFocused().keyPressed(event)`. An `EditBox` child
/// takes the key with the window's clipboard; a mod widget child its own `keyPressed`.
pub fn children_key_pressed(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    event: &mcrust::net::minecraft::client::input::key_event::KeyEvent,
) -> bool {
    let Some(focused) = children.get_focused() else {
        return false;
    };
    let widget = children.widget_mut(focused);
    if let Some(slot) = widget.mod_widget() {
        return slot.key_pressed(widget, event);
    }
    let (is_focused, active) = (widget.is_focused(), widget.active);
    let font = font();
    with_edit_box(widget, |e| {
        let mut memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
        match mcrust::net::minecraft::client::minecraft::GlfwClipboard::published() {
            Some(mut clipboard) => e.key_pressed(&font.splitter(), event.clone(), is_focused, active, &mut clipboard),
            None => e.key_pressed(&font.splitter(), event.clone(), is_focused, active, &mut memory),
        }
    })
    .unwrap_or(false)
}

/// `ContainerEventHandler.charTyped(event)` over widgets a mod object holds itself (see `children_key_pressed`).
pub fn children_char_typed(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent,
) -> bool {
    let Some(focused) = children.get_focused() else {
        return false;
    };
    let widget = children.widget_mut(focused);
    if let Some(slot) = widget.mod_widget() {
        return slot.char_typed(widget, event);
    }
    let (is_focused, active) = (widget.is_focused(), widget.active);
    let Some(c) = char::from_u32(event.codepoint() as u32) else {
        return false;
    };
    let font = font();
    with_edit_box(widget, |e| e.char_typed(&font.splitter(), c, is_focused, active)).unwrap_or(false)
}

/// `InputConstants.Key.getDisplayName()`: the key's system name (`glfwGetKeyName`, the table the frame loop fills),
/// else its translation, else `key.mouse` with the button number.
pub fn key_display_name(
    key: &mcrust::com::mojang::blaze3d::platform::input_constants::Key,
) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
    use mcrust::com::mojang::blaze3d::platform::input_constants::DisplayName;
    use mcrust::net::minecraft::network::chat::component;
    let p = MINECRAFT.load(std::sync::atomic::Ordering::Acquire);
    let key_name = |k: i32, _scancode: i32| -> Option<String> {
        if p.is_null() {
            return None;
        }
        minecraft().key_names.get(&k).cloned()
    };
    match key.get_display_name(&key_name, &|n| mcrust::net::minecraft::locale::language::has(n)) {
        DisplayName::Literal(text) => component::literal(&text),
        DisplayName::Translatable(k) => component::translatable(&k),
        DisplayName::MouseIndex(index) => component::translatable_args(
            "key.mouse",
            vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(index.to_string())],
        ),
    }
}

/// `ConfirmLinkScreen.confirmLink(parent, url)` - the `Button.OnPress` `button -> confirmLinkNow(parent, url)`, with
/// the screen that is up as the parent (every mod caller passes `this`). See `confirm_link_now`.
pub fn confirm_link(url: &str) -> mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress {
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{OnPress, PressLambda};
    let url = url.to_string();
    OnPress::Lambda(PressLambda::new(move |_screen, _index| confirm_link_now(&url)))
}

/// `ConfirmLinkScreen.confirmLinkNow(parent, url)` (trusted): `minecraft.gui.setScreen(new ConfirmLinkScreen(open ->
/// { if (open) Util.getPlatform().openUri(url); minecraft.gui.setScreen(parent); }, url, true))`. The game's own
/// screen, parked where `Screen.clickUrlAction` parks it (`Minecraft.pending_confirm_link`): the client loop pushes it
/// over the screen that is up, and its answer returns to that screen.
pub fn confirm_link_now(url: &str) {
    minecraft().pending_confirm_link = Some((url.to_string(), true));
}

/// `new ConfirmScreen(callback, title, message, yesButton, noButton)` opened by a mod: the game's `ConfirmScreen`
/// layout (`confirm_screen::init_components` - the title, the message, the YES / NO row) as a mod screen whose two
/// buttons and Escape answer the mod's `callback` (`BooleanConsumer`), as Java's do. The callback is held on the
/// render thread (the mod's objects are not `Send`).
pub struct ConfirmScreen {
    callback: RenderThread<std::rc::Rc<dyn Fn(bool)>>,
    title: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
    message: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
    yes_button: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
    no_button: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
}

impl ConfirmScreen {
    /// java: `new ConfirmScreen(callback, title, message, yesButton, noButton)`
    pub fn new(
        callback: impl Fn(bool) + 'static,
        title: impl IntoComponent,
        message: impl IntoComponent,
        yes_button: impl IntoComponent,
        no_button: impl IntoComponent,
    ) -> ConfirmScreen {
        use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
        ConfirmScreen {
            callback: RenderThread::new(std::rc::Rc::new(callback)),
            title: FormattedText::from(title.into_component()),
            message: FormattedText::from(message.into_component()),
            yes_button: FormattedText::from(yes_button.into_component()),
            no_button: FormattedText::from(no_button.into_component()),
        }
    }

    /// java: `new ConfirmScreen(callback, title, message)` - `CommonComponents.GUI_YES` / `GUI_NO`
    pub fn new_3(callback: impl Fn(bool) + 'static, title: impl IntoComponent, message: impl IntoComponent) -> ConfirmScreen {
        use mcrust::net::minecraft::network::chat::component;
        use mcrust::net::minecraft::client::gui::screens::confirm_screen::{GUI_NO_KEY, GUI_YES_KEY};
        ConfirmScreen::new(callback, title, message, component::translatable(GUI_YES_KEY), component::translatable(GUI_NO_KEY))
    }
}

// SAFETY: the callback is reached only through `RenderThread::get`, which checks the thread.
unsafe impl Send for ConfirmScreen {}

impl mcrust::mcrust::modding::screens::ModScreen for ConfirmScreen {
    fn get_title(&self) -> String {
        self.title.get_string()
    }

    /// java: ConfirmScreen.init:47-58 (+ repositionElements) - the vanilla dialog's widgets, its buttons answering
    /// the callback
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::{OnPress, PressLambda};
        let font = font();
        let built = mcrust::net::minecraft::client::gui::screens::confirm_screen::init_components(
            screen.width,
            screen.height,
            &self.title,
            &self.message,
            &self.yes_button,
            &self.no_button,
            &font.splitter(),
        );
        let renderables: Vec<usize> = built.screen.renderables().to_vec();
        for (i, widget) in built.screen.children().iter().enumerate() {
            let mut widget = widget.clone();
            if i == built.yes_index || i == built.no_index {
                let answer = i == built.yes_index;
                let callback = RenderThread::new(self.callback.get().clone());
                widget.on_press = OnPress::Lambda(PressLambda::new(move |_screen, _index| (callback.get())(answer)));
            }
            let index = screen.add_widget(widget);
            if renderables.contains(&i) {
                screen.mark_renderable(index);
            }
        }
    }

    /// java: ConfirmScreen.keyPressed - Escape answers `callback.accept(false)`
    fn key_pressed(&mut self, _screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if event.is_escape() {
            (self.callback.get())(false);
            return true;
        }
        false
    }

    /// java: ConfirmScreen.shouldCloseOnEsc:93-96 - `false`
    fn should_close_on_esc(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// A mod's widget class as ONE object (Java's `this`) that other mod objects also hold
// ---------------------------------------------------------------------------------------------------------------------

/// A mod class that `extends AbstractWidget` / `AbstractContainerWidget` whose object other mod objects reach between
/// the game's calls (Skyblocker's customization tabs set `widget.visible = ..` and call `widget.setCurrentItem(..)` on
/// widgets that sit in the screen's list). The object keeps Java's `AbstractWidget` fields itself (`object_box`: x, y,
/// size, `visible`, `active`, hover) and is the authority for `visible` / `active` - so a write made outside the
/// game's calls takes effect at the next frame, as Java's field write does. The game's widget keeps the box the
/// layouts move: what the game changed since the last call reaches the object through the class's own `setX` /
/// `setY` / `setWidth` / `setHeight` (Java's overrides run), and what a call changed goes back to the game's widget.
///
/// Every default body is `AbstractWidget.java`'s.
pub trait ObjectWidget: 'static {
    /// The object's `AbstractWidget` fields.
    fn object_box(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
    fn object_box_ref(&self) -> &mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;

    /// `setX(int)` - a subclass's override moves what it owns too.
    fn set_x(&mut self, x: i32) {
        self.object_box().set_x(x);
    }
    /// `setY(int)`.
    fn set_y(&mut self, y: i32) {
        self.object_box().set_y(y);
    }
    /// `setWidth(int)`.
    fn set_width(&mut self, width: i32) {
        self.object_box().set_width(width);
    }
    /// `setHeight(int)`.
    fn set_height(&mut self, height: i32) {
        self.object_box().set_height(height);
    }

    /// `extractWidgetRenderState` - ABSTRACT. Runs only while the object is `visible` (`extractRenderState:56-62`).
    fn extract_widget_render_state(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
        a: f32,
    );

    /// `updateWidgetNarration` (every class of this mod that uses this leaves it empty).
    fn update_widget_narration(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput,
    ) {
        let _ = (widget, output);
    }

    /// `onClick:100-101` - EMPTY.
    fn on_click(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        double_click: bool,
    ) {
        let _ = (widget, event, double_click);
    }

    /// `onRelease:103-104` - EMPTY.
    fn on_release(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
    ) {
        let _ = (widget, event);
    }

    /// `onDrag:106-107` - EMPTY.
    fn on_drag(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        dx: f64,
        dy: f64,
    ) {
        let _ = (widget, event, dx, dy);
    }

    /// `mouseClicked:109-125`.
    fn mouse_clicked(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        double_click: bool,
    ) -> bool {
        if !self.object_box_ref().is_active() {
            return false;
        }
        if self.is_valid_click_button(event.button_info()) && self.is_mouse_over(event.x(), event.y()) {
            self.on_click(widget, event, double_click);
            return true;
        }
        false
    }

    /// `mouseReleased:127-135`.
    fn mouse_released(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
    ) -> bool {
        if self.is_valid_click_button(event.button_info()) {
            self.on_release(widget, event);
            return true;
        }
        false
    }

    /// `mouseDragged:141-149`.
    fn mouse_dragged(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        dx: f64,
        dy: f64,
    ) -> bool {
        if self.is_valid_click_button(event.button_info()) {
            self.on_drag(widget, event, dx, dy);
            return true;
        }
        false
    }

    /// `GuiEventListener.mouseScrolled:30-32` - `false`.
    fn mouse_scrolled(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        x: f64,
        y: f64,
        scroll_x: f64,
        scroll_y: f64,
    ) -> bool {
        let _ = (widget, x, y, scroll_x, scroll_y);
        false
    }

    /// `GuiEventListener.keyPressed:34-36` - `false`.
    fn key_pressed(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::key_event::KeyEvent,
    ) -> bool {
        let _ = (widget, event);
        false
    }

    /// `GuiEventListener.charTyped:42-44` - `false`.
    fn char_typed(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent,
    ) -> bool {
        let _ = (widget, event);
        false
    }

    /// `isValidClickButton:137-139`.
    fn is_valid_click_button(&self, button_info: mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo) -> bool {
        button_info.button() == 0
    }

    /// `isMouseOver:161-164` - `isActive() && areCoordinatesInRectangle(..)`.
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.object_box_ref().is_mouse_over(mouse_x, mouse_y)
    }

    /// `setFocused(boolean):221-224`.
    fn set_focused(&mut self, widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, focused: bool) {
        widget.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None);
    }

    /// `playDownSound:166-168` - `UI_BUTTON_CLICK` (`None`: an override with an empty body).
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        Some(mcrust::net::minecraft::sounds::sound_events::ui_button_click())
    }

    /// `GuiEventListener.shouldTakeFocusAfterInteraction:63-65` - `true`.
    fn should_take_focus_after_interaction(&self) -> bool {
        true
    }

    /// `handleCursor` - what the class requests (`None`: a class that never calls it).
    fn handle_cursor(
        &self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes,
    ) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        let _ = (widget, cursors);
        None
    }
}

/// An `ObjectWidget` as the game's `ModWidget`: the object's handle (the same object the mod's code holds). Checked
/// to the render thread on every call.
pub struct ObjectWidgetSlot<C: ?Sized + 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
    /// the game's box (x, y, width, height) as of the end of the last call
    snap: std::cell::Cell<(i32, i32, i32, i32)>,
}

// SAFETY: every use goes through `check`, which pins the slot to the thread that made it.
unsafe impl<C: ?Sized + 'static> Send for ObjectWidgetSlot<C> {}

fn game_box(widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> (i32, i32, i32, i32) {
    (widget.get_x(), widget.get_y(), widget.get_width(), widget.get_height())
}

impl<C: ObjectWidget + ?Sized + 'static> ObjectWidgetSlot<C> {
    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod widget was reached off the render thread");
    }

    /// What the game changed on its widget since the last call (a layout moved or sized it) reaches the object
    /// through the class's own setters; the pass's hover is copied.
    fn sync_in(&self, object: &mut C, widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) {
        let (sx, sy, sw, sh) = self.snap.get();
        let (x, y, w, h) = game_box(widget);
        if x != sx {
            object.set_x(x);
        }
        if y != sy {
            object.set_y(y);
        }
        if w != sw {
            object.set_width(w);
        }
        if h != sh {
            object.set_height(h);
        }
        object.object_box().is_hovered = widget.is_hovered();
        self.snap.set((x, y, w, h));
    }

    /// The object's box written onto the game's widget (what the call, or the mod's code since, changed).
    fn sync_out(&self, object: &C, widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) {
        let b = object.object_box_ref();
        if b.get_x() != widget.get_x() {
            widget.set_x(b.get_x());
        }
        if b.get_y() != widget.get_y() {
            widget.set_y(b.get_y());
        }
        if b.get_width() != widget.get_width() || b.get_height() != widget.get_height() {
            widget.set_size(b.get_width(), b.get_height());
        }
        self.snap.set(game_box(widget));
    }

    fn call<R>(
        &self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        f: impl FnOnce(&mut C, &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> R,
    ) -> R {
        self.check();
        write(&self.object, |o| {
            self.sync_in(o, widget);
            let r = f(o, widget);
            self.sync_out(o, widget);
            r
        })
    }
}

/// `new MyWidget(..)` handed to the game (`addRenderableWidget`, a layout, a container's children): the game's widget
/// over the object's box.
pub fn object_widget<C: ObjectWidget + 'static>(
    object: Shared<C>,
    message: impl Into<String>,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    let (x, y, w, h) = read(&object, |o| {
        let b = o.object_box_ref();
        (b.get_x(), b.get_y(), b.get_width(), b.get_height())
    });
    let slot = ObjectWidgetSlot { object, owner: std::thread::current().id(), snap: std::cell::Cell::new((x, y, w, h)) };
    mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, y, w, h, message.into(), Box::new(slot))
}

/// The object's box written onto its game widget now (an owner that holds the screen, after a call that resized
/// the object - Java's `setWidth` override changing the height).
pub fn push_object_box<C: ObjectWidget + ?Sized>(
    object: &Shared<C>,
    widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
) {
    let (x, y, w, h) = read(object, |o| {
        let b = o.object_box_ref();
        (b.get_x(), b.get_y(), b.get_width(), b.get_height())
    });
    widget.set_position(x, y);
    widget.set_size(w, h);
}

impl<C: ObjectWidget + ?Sized + 'static> mcrust::mcrust::modding::widgets::ModWidget for ObjectWidgetSlot<C> {
    fn extract_widget_render_state(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
        a: f32,
    ) {
        self.check();
        write(&self.object, |o| {
            self.sync_in(o, widget);
            // AbstractWidget.extractRenderState:56-62 - `if (this.visible)`
            if o.object_box_ref().visible {
                o.extract_widget_render_state(widget, context, mouse_x, mouse_y, a);
            }
        })
    }

    fn update_widget_narration(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput,
    ) {
        self.check();
        write(&self.object, |o| o.update_widget_narration(widget, output))
    }

    fn mouse_clicked(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        double_click: bool,
    ) -> bool {
        self.call(widget, |o, w| o.mouse_clicked(w, event, double_click))
    }

    fn mouse_released(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
    ) -> bool {
        self.call(widget, |o, w| o.mouse_released(w, event))
    }

    fn mouse_dragged(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        dx: f64,
        dy: f64,
    ) -> bool {
        self.call(widget, |o, w| o.mouse_dragged(w, event, dx, dy))
    }

    fn mouse_scrolled(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        x: f64,
        y: f64,
        scroll_x: f64,
        scroll_y: f64,
    ) -> bool {
        self.call(widget, |o, w| o.mouse_scrolled(w, x, y, scroll_x, scroll_y))
    }

    fn key_pressed(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::key_event::KeyEvent,
    ) -> bool {
        self.call(widget, |o, w| o.key_pressed(w, event))
    }

    fn char_typed(
        &mut self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent,
    ) -> bool {
        self.call(widget, |o, w| o.char_typed(w, event))
    }

    fn is_valid_click_button(&self, button_info: mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo) -> bool {
        self.check();
        read(&self.object, |o| o.is_valid_click_button(button_info))
    }

    fn is_mouse_over(
        &self,
        _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        mouse_x: f64,
        mouse_y: f64,
    ) -> bool {
        self.check();
        read(&self.object, |o| o.is_mouse_over(mouse_x, mouse_y))
    }

    fn set_focused(&mut self, widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, focused: bool) {
        self.call(widget, |o, w| o.set_focused(w, focused))
    }

    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        self.check();
        read(&self.object, |o| o.play_down_sound())
    }

    fn should_take_focus_after_interaction(&self) -> bool {
        self.check();
        read(&self.object, |o| o.should_take_focus_after_interaction())
    }

    fn handle_cursor(
        &self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes,
    ) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        self.check();
        read(&self.object, |o| o.handle_cursor(widget, cursors))
    }

    fn class_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }
}

/// `ContainerEventHandler.mouseClicked:39-55` over widgets a mod object holds itself (an `AbstractContainerWidget`
/// subclass's `children()`): the child under the cursor takes the click, is focused and starts the drag. `true` when
/// a child took it.
pub fn children_mouse_clicked(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
) -> bool {
    let font = font();
    children.mouse_clicked_with(event.button(), event.x(), event.y(), Some(&font.splitter())).is_some()
}

/// `ContainerEventHandler.mouseReleased:57-66` over a mod object's own widgets.
pub fn children_mouse_released(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
) {
    children.mouse_released(event.button(), event.x(), event.y());
}

/// `ContainerEventHandler.mouseDragged:68-71` over a mod object's own widgets (the focused child, while dragging).
pub fn children_mouse_dragged(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
    dx: f64,
    dy: f64,
) {
    let font = font();
    children.mouse_dragged_with(event.x(), event.y(), dx, dy, Some(&font.splitter()));
}

/// `ContainerEventHandler.mouseScrolled:74-78` over a mod object's own widgets: the child under the cursor.
pub fn children_mouse_scrolled(
    children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    x: f64,
    y: f64,
    scroll_x: f64,
    scroll_y: f64,
) -> bool {
    children.mouse_scrolled_mod_widget(x, y, scroll_x, scroll_y)
}

/// `Checkbox.builder(message, font).pos(x, y).maxWidth(maxWidth).selected(selected).onValueChange(onValueChange)
/// .build()`: the game's checkbox widget. Its press is `Checkbox.onPress` - `selected = !selected;
/// onValueChange.onValueChange(this, selected)`.
pub fn checkbox(
    message: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
    x: i32,
    y: i32,
    max_width: Option<i32>,
    selected: bool,
    on_value_change: impl Fn(bool) + Send + Sync + 'static,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, PressLambda, WidgetKind};
    let font = font();
    let name = message.get_string();
    // java: Builder.build - `if (box.overflowsRowLimit(font)) box.setTooltip(..)`
    let checkbox = mcrust::net::minecraft::client::gui::components::checkbox::Checkbox::build(&font.splitter(), x, y, max_width, message, selected, true);
    let (width, height) = (checkbox.width, checkbox.height);
    let on_value_change = RenderThread::new(on_value_change);
    AbstractWidget::new(
        x,
        y,
        width,
        height,
        name,
        WidgetKind::Checkbox(checkbox),
        OnPress::Lambda(PressLambda::new(move |screen, index| {
            let checked = match &mut screen.widget_mut(index).kind {
                WidgetKind::Checkbox(c) => c.on_press(),
                _ => return,
            };
            (on_value_change.get())(checked);
        })),
    )
}

/// `checkbox.selected()`.
pub fn checkbox_selected(widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> bool {
    match &widget.kind {
        mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Checkbox(c) => c.selected(),
        _ => panic!("ClassCastException: the widget is not a Checkbox"),
    }
}

/// `((CheckboxAccessor) checkbox).setSelected(selected)` - the field, without the value-change callback.
pub fn set_checkbox_selected(widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, selected: bool) {
    match &mut widget.kind {
        mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Checkbox(c) => c.selected = selected,
        _ => panic!("ClassCastException: the widget is not a Checkbox"),
    }
}

/// `AbstractScrollArea.extractScrollbar(graphics, mouseX, mouseY)` for a mod's `AbstractContainerWidget` subclass
/// that scrolls (its `ScrollArea` and `contentHeight()`): the background and the scroller at `scrollBarX()`, only
/// while the content overflows (`defaultSettings` has no disabled-scroller sprite).
pub fn extract_scrollbar(
    context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
    area: &mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea,
    content_height: i32,
    scroll_bar_x: i32,
) {
    use mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_sprite_identifier;
    use mcrust::net::minecraft::resources::identifier::Identifier;
    if !area.scrollable(content_height) {
        return;
    }
    let pipeline = mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured();
    blit_sprite_identifier(
        context.gl,
        context.minecraft,
        pipeline.clone(),
        &Identifier::with_default_namespace(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLER_BACKGROUND_SPRITE),
        scroll_bar_x,
        area.y,
        area.scrollbar_width(),
        area.height,
    );
    blit_sprite_identifier(
        context.gl,
        context.minecraft,
        pipeline,
        &Identifier::with_default_namespace(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLER_SPRITE),
        scroll_bar_x,
        area.scroll_bar_y(content_height),
        area.scrollbar_width(),
        area.scroller_height(content_height),
    );
}

/// `AbstractSliderButton.extractWidgetRenderState` (the track, the handle and the message) for a mod's
/// `AbstractSliderButton` subclass that keeps its own value: the game draws a slider of `WidgetKind::PlainSlider`
/// with those bodies, so a one-widget `Screen` holding such a slider, brought up to the mod widget's box, value and
/// state before each draw, is drawn through the game's own slider pass (see `ButtonLook`).
pub struct SliderLook {
    screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl std::fmt::Debug for SliderLook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SliderLook")
    }
}

impl Default for SliderLook {
    fn default() -> SliderLook {
        SliderLook::new()
    }
}

impl SliderLook {
    pub fn new() -> SliderLook {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
        let mut screen = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        let state = mcrust::net::minecraft::client::gui::components::abstract_slider_button::SliderState::new(0.0);
        screen.add_renderable_widget(AbstractWidget::new(0, 0, 0, 0, String::new(), WidgetKind::PlainSlider(state), OnPress::Inert));
        SliderLook { screen }
    }

    /// The slider drawn over `widget`'s box, message, `active`, alpha and focus, at `state`'s value.
    pub fn draw(
        &mut self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
        state: &mcrust::net::minecraft::client::gui::components::abstract_slider_button::SliderState,
        context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
        let s = self.screen.widget_mut(0);
        s.set_position(widget.get_x(), widget.get_y());
        s.width = widget.width;
        s.height = widget.height;
        if s.message_text.as_ref() != Some(message) {
            s.message = message.get_string();
            s.message_text = Some(message.clone());
        }
        s.active = widget.active;
        s.visible = widget.visible;
        s.alpha = widget.alpha;
        if s.is_focused() != widget.is_focused() {
            s.set_focused(widget.is_focused(), mcrust::net::minecraft::client::input_type::InputType::None);
        }
        if let WidgetKind::PlainSlider(own) = &mut s.kind {
            if own != state {
                *own = state.clone();
            }
        }
        context.extract_widgets(&mut self.screen, mouse_x, mouse_y);
    }
}

/// `graphics.textRenderer(..).acceptScrollingWithDefaultCenter(message, left, right, top, bottom)`: the message
/// centred between `left` and `right` (on the 9-pixel line between `top` and `bottom`), scrolling back and forth
/// inside them when it does not fit - the placement `ActiveTextCollector.defaultScrollingHelper` answers, drawn the
/// way the game draws a widget's label.
pub fn accept_scrolling_with_default_center(
    context: &mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
    message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
) {
    let width = mcrust::net::minecraft::client::gui::font::width(context.minecraft.font.splitter().string_width_formatted(message));
    let placement = mcrust::net::minecraft::client::gui::active_text_collector::accept_scrolling_with_default_center(left, right, top, bottom, width, context.millis);
    mcrust::net::minecraft::client::gui::gui_graphics_extractor::draw_clipped_formatted_text(
        context.gl,
        context.minecraft,
        placement.x,
        placement.y,
        message,
        -1,
        &mcrust::net::minecraft::network::chat::style::Style::EMPTY,
        placement.scissor,
        None,
        context.scale,
        context.fb_height,
    );
}

// ---------------------------------------------------------------------------------------------------------------------
// Helpers other batches wrote for their files that belong here (moved by the cleanup round; the files that wrote them
// still hold their own copies until their owners switch to these).
// ---------------------------------------------------------------------------------------------------------------------
pub use moved_helpers::*;

#[allow(unused_imports)]
mod moved_helpers {
    use crate::compat;
    use crate::de::hysky::skyblocker::skyblock::events::event_toast::EventToast;
    use crate::de::hysky::skyblocker::utils::location::Location;
    use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group::WaypointGroup;
    use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
    use mcrust::net::minecraft::client::gui::components::toasts::toast_manager::ToastManager;
    use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
    use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
    use mcrust::net::minecraft::world::entity::entity::Entity;

    // ---- from de/hysky/skyblocker/skyblocker_screen.rs
    /// The port draws a `FormattedText` where Java draws a `FormattedCharSequence`: the sequence's styled
    /// characters, gathered back into runs of one style each.
    pub fn sequence_text(seq: &dyn FormattedCharSequence) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
        use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
        use mcrust::net::minecraft::network::chat::style::Style;
        let mut runs: Vec<(Style, String)> = Vec::new();
        seq.accept(&mut |_position: i32, style: &Style, codepoint: u32| {
            if let Some(c) = char::from_u32(codepoint) {
                match runs.last_mut() {
                    Some((s, text)) if s == style => text.push(c),
                    _ => runs.push((style.clone(), c.to_string())),
                }
            }
            true
        });
        FormattedText::composite(runs.into_iter().map(|(style, text)| FormattedText::of_styled(text, style)).collect())
    }

    // ---- from de/hysky/skyblocker/skyblock/chat/chat_rule.rs
    /// java: `Objects.equals(a, b)` over two nullable references, with the class's own `equals` for two non-null ones
    pub fn objects_equals<T: ?Sized>(a: &Option<compat::Shared<T>>, b: &Option<compat::Shared<T>>, equals: impl Fn(&compat::Shared<T>, &compat::Shared<T>) -> bool) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b) || equals(a, b),
            _ => false,
        }
    }

    // ---- from de/hysky/skyblocker/skyblock/chat/chat_rule.rs
    /// java: `Objects.hash(values...)` - `Arrays.hashCode`: `31 * h + hashCode(e)` from 1
    pub fn objects_hash(hashes: &[i32]) -> i32 {
        hashes.iter().fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(*e))
    }

    // ---- from de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs
    /// Java's `Character.isWhitespace` (what `String.stripLeading` strips): not the no-break spaces.
    pub fn is_java_whitespace(c: char) -> bool {
        match c {
            '\u{00A0}' | '\u{2007}' | '\u{202F}' => false,
            '\t' | '\n' | '\u{000B}' | '\u{000C}' | '\r' | '\u{001C}'..='\u{001F}' => true,
            _ => c.is_whitespace() && !matches!(c, '\u{0085}'),
        }
    }

    // ---- from de/hysky/skyblocker/skyblock/slayers/slayer_manager.rs
    /// `game::with_entity(id, f)`, or `f(held)` when `held` IS that entity: a caller that already holds an entity's
    /// lock (the packet handler that lent us the armor stand) reaches it through its own reference, as Java's single
    /// object would be reached; `with_entity` would answer `None` for it.
    pub fn with_entity_or_held<R>(held: Option<&dyn Entity>, id: i32, f: impl FnOnce(&dyn Entity) -> R) -> Option<R> {
        if let Some(held) = held {
            if held.base().get_id() == id {
                return Some(f(held));
            }
        }
        mcrust::mcrust::modding::game::with_entity(id, |e| f(&*e))
    }

    // ---- from de/hysky/skyblocker/skyblock/events/event_toast.rs
    /// `EventToast implements Toast`: the object as the game's `Toast`, for `ToastManager.addToast` - each of the
    /// interface's methods dispatches to the class's own (`compat::dispatch`, so a subclass's overrides run).
    pub struct EventToastAdapter(pub compat::Shared<dyn EventToast>);

    impl mcrust::net::minecraft::client::gui::components::toasts::toast::Toast for EventToastAdapter {
        fn get_wanted_visibility(&self) -> mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility {
            let __f = compat::dispatch(&self.0, |__t| __t.get_wanted_visibility());
            __f(&self.0)
        }

        fn update(&mut self, manager: &ToastManager, fully_visible_for_ms: i64) {
            let __f = compat::dispatch(&self.0, |__t| __t.update());
            __f(&self.0, manager, fully_visible_for_ms)
        }

        fn extract_render_state(
            &self,
            _graphics: &mut mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>,
            _font: &mcrust::net::minecraft::client::gui::font::font_set::FontSet,
            fully_visible_for_ms: i64,
        ) {
            // the class's body draws through the `gui_graphics_extractor` functions (the frame's own state), as the HUD
            // callbacks do; its `graphics` is the extractor those functions stand for
            let __f = compat::dispatch(&self.0, |__t| __t.extract_render_state());
            __f(&self.0, &GuiGraphicsExtractor::new(), &compat::font(), fully_visible_for_ms)
        }

        fn width(&self) -> i32 {
            let __f = compat::dispatch(&self.0, |__t| __t.width());
            __f(&self.0)
        }

        fn height(&self) -> i32 {
            let __f = compat::dispatch(&self.0, |__t| __t.height());
            __f(&self.0)
        }
    }

    // ---- from de/hysky/skyblocker/skyblock/waypoint/waypoints.rs
    /// `Multimap<Location, WaypointGroup>` as `MultimapBuilder.enumKeys(Location.class).arrayListValues().build()`
    /// makes it: the keys in `Location`'s declaration order (an `EnumMap`; `Location`'s derived `Ord` is that order),
    /// each key's values an `ArrayList` in insertion order. A key whose list is empty is not in the map (Guava drops
    /// it), which `equals` and `values` both see. Java holds the map by reference (`WaypointsScreen` and the
    /// `WaypointsShareScreen` it opens edit the same one), so every holder keeps a `compat::Shared` of it.
    /// rust: belongs with `compat` (Guava's `Multimap` is unported - `crate::unresolved::Multimap`); kept here, the one
    /// package that builds this map.
    #[derive(Default)]
    pub struct LocationMultimap {
        map: std::collections::BTreeMap<Location, Vec<compat::Shared<WaypointGroup>>>,
    }

    impl LocationMultimap {
        /// `MultimapBuilder.enumKeys(Location.class).arrayListValues().build()`
        pub fn new() -> LocationMultimap {
            LocationMultimap { map: std::collections::BTreeMap::new() }
        }

        /// `get(key)`, read: the groups of the key, in order (empty when there are none).
        pub fn get(&self, key: Location) -> Vec<compat::Shared<WaypointGroup>> {
            self.map.get(&key).cloned().unwrap_or_default()
        }

        /// `get(key)` as the live collection Guava hands out: a change to it is a change to the map.
        pub fn get_mut(&mut self, key: Location) -> &mut Vec<compat::Shared<WaypointGroup>> {
            self.map.entry(key).or_default()
        }

        /// `put(key, value)`: an `ArrayList` value collection always grows, so it answers `true`.
        pub fn put(&mut self, key: Location, value: compat::Shared<WaypointGroup>) -> bool {
            self.map.entry(key).or_default().push(value);
            true
        }

        /// `remove(key, value)`: the first group of the key that `equals` the value (`ArrayList.remove(Object)`).
        pub fn remove(&mut self, key: Location, value: &compat::Shared<WaypointGroup>) -> bool {
            let Some(list) = self.map.get_mut(&key) else { return false };
            let Some(index) = list.iter().position(|group| WaypointGroup::equals_shared(group, value)) else { return false };
            list.remove(index);
            true
        }

        /// `putAll(multimap)`: every entry of the other map, in its order; `true` when anything was added.
        pub fn put_all(&mut self, other: &LocationMultimap) -> bool {
            let mut changed = false;
            for (key, values) in other.map.iter() {
                for value in values.iter() {
                    changed |= self.put(*key, value.clone());
                }
            }
            changed
        }

        /// `clear()`
        pub fn clear(&mut self) {
            self.map.clear();
        }

        /// `values()`: every group, key by key.
        pub fn values(&self) -> Vec<compat::Shared<WaypointGroup>> {
            self.map.values().flat_map(|values| values.iter().cloned()).collect()
        }

        /// The keys that hold at least one group, in key order (`asMap().keySet()`).
        pub fn keys(&self) -> Vec<Location> {
            self.map.iter().filter(|(_, values)| !values.is_empty()).map(|(key, _)| *key).collect()
        }

        /// `equals(Object)`: `asMap().equals(other.asMap())` - the same keys, and for each key lists that are equal
        /// element by element (`WaypointGroup.equals`).
        pub fn equals(&self, other: &LocationMultimap) -> bool {
            let keys = self.keys();
            if keys != other.keys() {
                return false;
            }
            keys.iter().all(|key| {
                let (a, b) = (&self.map[key], &other.map[key]);
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| WaypointGroup::equals_shared(a, b))
            })
        }
    }

    // ---- from de/hysky/skyblocker/skyblock/dwarven/profittrackers/corpse/reward_list.rs
    /// `NumberFormat.getInstance()`: the default locale's NUMBER pattern `#,##0.###`. The port models no default locale
    /// (`java::text` has ROOT and US, whose CLDR symbols are the same); US is used.
    pub fn number_format_instance() -> mcrust::java::text::decimal_format::DecimalFormat {
        mcrust::java::text::decimal_format::DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us))
    }

    // ---- from de/hysky/skyblocker/skyblock/chest_value.rs
    /// java: `NumberFormat.getNumberInstance(Locale.US).parse(text).doubleValue()` - `DecimalFormat("#,##0.###")`'s
    /// `parse(String)`: the longest prefix of `text` that is a number - an optional `-`, digits with `,` grouping
    /// separators (a trailing `,` is not taken), at most one `.` (no `,` after it), and an `E` exponent; `NaN` and the
    /// infinity sign as the symbols. `None` is the `ParseException` (nothing at the start parsed).
    /// rust: port gap - `DecimalFormat.parse` / `NumberFormat.getNumberInstance` (java/text has `new` + `format`); the
    /// JDK body is reproduced here for this one caller (GAPS_batchI.md)
    pub fn number_instance_us_parse(text: &str) -> Option<f64> {
        let chars: Vec<char> = text.chars().collect();
        if text.starts_with("NaN") {
            return Some(f64::NAN);
        }
        let mut i: usize = 0;
        let negative = chars.first() == Some(&'-');
        if negative {
            i = 1;
        }
        if chars.get(i) == Some(&'∞') {
            return Some(if negative { f64::NEG_INFINITY } else { f64::INFINITY });
        }
        let mut number = String::new();
        let mut saw_digit = false;
        let mut saw_decimal = false;
        let mut exponent: i32 = 0;
        while i < chars.len() {
            let c = chars[i];
            if let Some(digit) = c.to_digit(10) {
                number.push(char::from_digit(digit, 10).unwrap_or('0'));
                saw_digit = true;
                i += 1;
            } else if c == '.' {
                if saw_decimal {
                    break;
                }
                saw_decimal = true;
                number.push('.');
                i += 1;
            } else if c == ',' {
                // a grouping separator: skipped (and not taken at all when no digit follows - nothing reads the position)
                if saw_decimal {
                    break;
                }
                i += 1;
            } else if c == 'E' && saw_digit {
                // `subparse` of the exponent: an optional `-`, then digits; without digits the `E` is not taken
                let mut j = i + 1;
                let exponent_negative = chars.get(j) == Some(&'-');
                if exponent_negative {
                    j += 1;
                }
                let start = j;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    j += 1;
                }
                if j == start {
                    break;
                }
                let digits: String = chars[start..j].iter().collect();
                exponent = digits.parse::<i32>().unwrap_or(i32::MAX);
                if exponent_negative {
                    exponent = -exponent;
                }
                break;
            } else {
                break;
            }
        }
        if !saw_digit {
            return None;
        }
        let mut parsed: f64 = number.parse::<f64>().ok()?;
        if exponent != 0 {
            parsed *= 10f64.powi(exponent);
        }
        Some(if negative { -parsed } else { parsed })
    }

    // ---- from de/hysky/skyblocker/skyblock/dungeon/secrets/secret_waypoint.rs
    /// java: `Component.hashCode()` (`MutableComponent.hashCode:112-117`: `31 * (31 * (31 * 1 + contents.hashCode()) +
    /// style.hashCode()) + siblings.hashCode()`) for the component a secret waypoint is named with,
    /// `Component.nullToEmpty(name)`: `LiteralContents(text)` (a record of one field: `text.hashCode()`), `Style.EMPTY`
    /// (`Objects.hash` of ten nulls: `31^10`) and no siblings (`List.hashCode()` of an empty list: `1`).
    /// rust: port gap - any other component (a style, siblings, other contents) has no Java `int` in the port; such a
    /// name hashes by the port's own value (consistent with `equals`, not the JVM's number)
    pub fn component_java_hash_code(name: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> i32 {
        use mcrust::net::minecraft::network::chat::component::Component as _;
        use mcrust::net::minecraft::network::chat::component_contents::ComponentContents;
        use mcrust::net::minecraft::network::chat::contents::plain_text_contents::PlainTextContents;
        if let ComponentContents::Text(PlainTextContents::Literal(text)) = &**name.get_contents() && name.get_style().is_empty() && name.get_siblings().is_empty() {
            let style_empty: i32 = (0..10).fold(1i32, |h, _| h.wrapping_mul(31));
            let siblings_empty: i32 = 1;
            let contents: i32 = compat::string_hash_code(text);
            return 1i32.wrapping_mul(31).wrapping_add(contents).wrapping_mul(31).wrapping_add(style_empty).wrapping_mul(31).wrapping_add(siblings_empty);
        }
        compat::hash_code(name)
    }


    // ---- from de/hysky/skyblocker/skyblock/dungeon/partyfinder/finder_settings_container.rs
    /// A mod class that `extends AbstractContainerWidget` (this container, `RangedValueWidget`, `ChatRuleConfigScreen
    /// .ContentContainer`, ..): its container half (`base`), which holds `super`'s fields.
    ///
    /// (A helper for the batch's container widgets; it belongs beside `compat::WidgetHalf` - see PATTERNS_batchF.md.)
    pub trait ContainerWidgetHalf {
        fn container_half(&mut self) -> &mut AbstractContainerWidget;
    }

    /// The fields of a container widget's box the game keeps on its own `AbstractWidget` and syncs into the part
    /// (`AbstractContainerWidget::sync_from_widget` / `sync_to_widget`).
    #[derive(Clone, Copy, PartialEq)]
    pub struct ContainerBox {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
        pub active: bool,
        pub visible: bool,
    }

    impl ContainerBox {
        pub fn of(w: &AbstractContainerWidget) -> ContainerBox {
            ContainerBox { x: w.get_x(), y: w.get_y(), width: w.width, height: w.height, active: w.active, visible: w.visible }
        }

        pub fn write(&self, w: &mut AbstractContainerWidget) {
            w.set_x(self.x);
            w.set_y(self.y);
            w.width = self.width;
            w.height = self.height;
            w.active = self.active;
            w.visible = self.visible;
        }
    }

    /// A mod object that `extends AbstractContainerWidget`, as the game's `ModWidget`: Java's ONE object is the mod's
    /// object (its fields, its `base`) plus the game's `AbstractWidget` in the screen's list. The game syncs that
    /// widget into `part` before each call (`ModWidget::container_widget_mut`); `lend` brings it into the object's
    /// `base` - keeping a box field the mod's own code changed since the last call (`setRectangle` from the screen),
    /// which then reaches the game's widget through `part` - and copies the object's box back into `part` after the
    /// call, which the game writes back onto its widget for a call that holds it (`sync_to_widget`).
    pub struct ContainerWidgetAdapter<C: 'static> {
        object: compat::RenderThread<compat::Shared<C>>,
        part: AbstractContainerWidget,
        /// the game's box as of the last call: a field of the object's `base` that differs from it was set by the mod
        synced: ContainerBox,
    }

    impl<C: ContainerWidgetHalf + 'static> ContainerWidgetAdapter<C> {
        /// `new MyContainer(..)` handed to the game (`addRenderableWidget`, a layout): the game's widget, built from
        /// the object's own box, with the object attached.
        pub fn widget(object: compat::Shared<C>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
        where
            ContainerWidgetAdapter<C>: mcrust::mcrust::modding::widgets::ModWidget,
        {
            let part: AbstractContainerWidget = compat::write(&object, |__t| __t.container_half().clone());
            let synced = ContainerBox::of(&part);
            let message: String = part.get_message().get_string();
            let mut widget = mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(synced.x, synced.y, synced.width, synced.height, message, Box::new(ContainerWidgetAdapter { object: compat::RenderThread::new(object), part, synced }));
            widget.active = synced.active;
            widget.visible = synced.visible;
            widget
        }

        /// The mod object.
        pub fn object(&self) -> &compat::Shared<C> {
            self.object.get()
        }

        /// The part the game syncs (`container_widget_mut`).
        pub fn part_mut(&mut self) -> &mut AbstractContainerWidget {
            &mut self.part
        }

        /// One of the object's methods, with its `base` brought up to the live widget first and its box handed back
        /// after.
        pub fn lend<R>(&mut self, f: impl FnOnce(&compat::Shared<C>) -> R) -> R {
            let object: compat::Shared<C> = self.object.get().clone();
            let live = ContainerBox::of(&self.part);
            let synced = self.synced;
            let hovered = self.part.is_hovered;
            compat::write(&object, |__t| {
                let half = __t.container_half();
                let mine = ContainerBox::of(half);
                let merged = ContainerBox {
                    x: if mine.x != synced.x { mine.x } else { live.x },
                    y: if mine.y != synced.y { mine.y } else { live.y },
                    width: if mine.width != synced.width { mine.width } else { live.width },
                    height: if mine.height != synced.height { mine.height } else { live.height },
                    active: if mine.active != synced.active { mine.active } else { live.active },
                    visible: if mine.visible != synced.visible { mine.visible } else { live.visible },
                };
                merged.write(half);
                half.is_hovered = hovered;
            });
            self.synced = live;
            let out = f(&object);
            let after = compat::write(&object, |__t| ContainerBox::of(__t.container_half()));
            after.write(&mut self.part);
            out
        }
    }
}

/// commons-lang3 `NumberUtils.isCreatable(String)`: whether `str` is a valid Java number literal (`createNumber`
/// accepts it) - an optional sign, hex (`0x`), octal (leading `0`), decimal with a point and an exponent, and a
/// trailing type qualifier (`d D f F l L`).
pub fn number_utils_is_creatable(str: &str) -> bool {
    if str.is_empty() {
        return false;
    }
    let chars: Vec<char> = str.chars().collect();
    let mut sz = chars.len();
    let mut has_exp = false;
    let mut has_dec_point = false;
    let mut allow_signs = false;
    let mut found_digit = false;
    let start = if chars[0] == '-' || chars[0] == '+' { 1 } else { 0 };
    if sz > start + 1 && chars[start] == '0' && !str.contains('.') {
        if chars[start + 1] == 'x' || chars[start + 1] == 'X' {
            let i = start + 2;
            if i == sz {
                return false;
            }
            return chars[i..].iter().all(|c| c.is_ascii_hexdigit());
        }
        if chars[start + 1].is_ascii_digit() {
            return chars[start + 1..].iter().all(|c| ('0'..='7').contains(c));
        }
    }
    sz -= 1;
    let mut i = start;
    while i < sz || (i < sz + 1 && allow_signs && !found_digit) {
        let c = chars[i];
        if c.is_ascii_digit() {
            found_digit = true;
            allow_signs = false;
        } else if c == '.' {
            if has_dec_point || has_exp {
                return false;
            }
            has_dec_point = true;
        } else if c == 'e' || c == 'E' {
            if has_exp || !found_digit {
                return false;
            }
            has_exp = true;
            allow_signs = true;
        } else if c == '+' || c == '-' {
            if !allow_signs {
                return false;
            }
            allow_signs = false;
            found_digit = false;
        } else {
            return false;
        }
        i += 1;
    }
    if i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            return true;
        }
        if c == 'e' || c == 'E' {
            return false;
        }
        if c == '.' {
            if has_dec_point || has_exp {
                return false;
            }
            return found_digit;
        }
        if !allow_signs && (c == 'd' || c == 'D' || c == 'f' || c == 'F') {
            return found_digit;
        }
        if c == 'l' || c == 'L' {
            return found_digit && !has_exp && !has_dec_point;
        }
        return false;
    }
    !allow_signs && found_digit
}

/// commons-lang3 `NumberUtils.toInt(String str, int defaultValue)` - `Integer.parseInt` (no trimming), the default on a
/// `NumberFormatException`. `toInt(str)` is `toInt(str, 0)`.
pub fn number_utils_to_int(str: &str, default_value: i32) -> i32 {
    str.parse::<i32>().unwrap_or(default_value)
}

/// `new URI(text).getPath()`: the decoded path of a hierarchical URI (what follows the scheme and the `//authority`, up
/// to `?` or `#`), `None` for an opaque one (`mailto:x`) as Java's `null`; `Err` is the constructor's
/// `URISyntaxException`. The port's `java.net.URI` keeps the text and the scheme only.
pub fn uri_get_path(text: &str) -> Result<Option<String>, String> {
    let uri = mcrust::java::net::uri::Uri::create(text).map_err(|e| e.to_string())?;
    let mut rest: &str = match uri.scheme() {
        Some(scheme) => &text[scheme.len() + 1..],
        None => text,
    };
    if uri.scheme().is_some() && !rest.starts_with('/') {
        return Ok(None);
    }
    if let Some(i) = rest.find('#') {
        rest = &rest[..i];
    }
    if let Some(i) = rest.find('?') {
        rest = &rest[..i];
    }
    if let Some(after) = rest.strip_prefix("//") {
        rest = match after.find('/') {
            Some(i) => &after[i..],
            None => "",
        };
    }
    // the path's `%XX` escapes, decoded as UTF-8 (`URI.decode`)
    let bytes = rest.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&rest[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    Ok(Some(String::from_utf8_lossy(&out).into_owned()))
}

/// commons-io `FilenameUtils.getName(fileName)`: what follows the last `/` or `\`.
pub fn filename_get_name(file_name: &str) -> String {
    match file_name.rfind(['/', '\\']) {
        Some(i) => file_name[i + 1..].to_string(),
        None => file_name.to_string(),
    }
}

/// commons-io `FilenameUtils.getExtension(fileName)`: what follows the last `.` of the name (`""` without one).
pub fn filename_get_extension(file_name: &str) -> String {
    let name = filename_get_name(file_name);
    match name.rfind('.') {
        Some(i) => name[i + 1..].to_string(),
        None => String::new(),
    }
}

/// commons-io `FilenameUtils.getBaseName(fileName)`: the name without its directories and its extension.
pub fn filename_get_base_name(file_name: &str) -> String {
    let name = filename_get_name(file_name);
    match name.rfind('.') {
        Some(i) => name[..i].to_string(),
        None => name,
    }
}

/// `ChatFormatting.name()` - the Java enum constant's name (`DARK_RED`); the port's enum has no `name()`.
pub fn chat_formatting_name(formatting: mcrust::net::minecraft::chat_formatting::ChatFormatting) -> &'static str {
    use mcrust::net::minecraft::chat_formatting::ChatFormatting;
    match formatting {
        ChatFormatting::Black => "BLACK",
        ChatFormatting::DarkBlue => "DARK_BLUE",
        ChatFormatting::DarkGreen => "DARK_GREEN",
        ChatFormatting::DarkAqua => "DARK_AQUA",
        ChatFormatting::DarkRed => "DARK_RED",
        ChatFormatting::DarkPurple => "DARK_PURPLE",
        ChatFormatting::Gold => "GOLD",
        ChatFormatting::Gray => "GRAY",
        ChatFormatting::DarkGray => "DARK_GRAY",
        ChatFormatting::Blue => "BLUE",
        ChatFormatting::Green => "GREEN",
        ChatFormatting::Aqua => "AQUA",
        ChatFormatting::Red => "RED",
        ChatFormatting::LightPurple => "LIGHT_PURPLE",
        ChatFormatting::Yellow => "YELLOW",
        ChatFormatting::White => "WHITE",
        ChatFormatting::Obfuscated => "OBFUSCATED",
        ChatFormatting::Bold => "BOLD",
        ChatFormatting::Strikethrough => "STRIKETHROUGH",
        ChatFormatting::Underline => "UNDERLINE",
        ChatFormatting::Italic => "ITALIC",
        ChatFormatting::Reset => "RESET",
    }
}

/// `minecraft.gameMode.handleContainerInput(containerId, slotId, buttonNum, containerInput, minecraft.player)`: the
/// port's `handleContainerInput:443-481` over the player (its `:445-447` check of the container id is made here - the
/// port's body is handed the open menu's own id), then `:480`'s send. (The same body four files keep as a private
/// copy: `leap_overlay`, `loadout_keybinds`, `radial_menu_screen`, `wardrobe_keybinds`.)
pub fn handle_container_input(
    container_id: i32,
    slot_id: i32,
    button_num: i32,
    container_input: mcrust::net::minecraft::world::inventory::container_input::ContainerInput,
) {
    use mcrust::net::minecraft::client::multiplayer::multi_player_game_mode;
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    let packet = mcrust::mcrust::modding::game::with_player(|player| {
        let open_container_id = player.player().container_menu_container_id();
        if container_id != open_container_id {
            crate::prelude::logger().warn(&format!("Ignoring click in mismatching container. Click in {}, player has {}.", container_id, open_container_id));
            return None;
        }
        multi_player_game_mode::handle_container_input_on_player(&mut minecraft().game_mode, player, slot_id, button_num, container_input)
    })
    .flatten();
    multi_player_game_mode::send_container_click(packet, container_input, slot_id, button_num);
}

// ===========================================================================
// batch N
// ===========================================================================

/// `player.connection.send(packet)` / `minecraft.getConnection().send(packet)` from the client thread: the play
/// connection's own hand-off (`client_advancements::send`, the handle's event loop runs the write). `false` when
/// there is no play connection (Java's `connection` would be null).
pub fn send_packet<P: mcrust::net::minecraft::network::protocol::packet::Packet + Send + 'static>(packet: P) -> bool {
    match mcrust::net::minecraft::client::multiplayer::client_packet_listener::published_play_sinks() {
        Some(sinks) => {
            mcrust::net::minecraft::client::multiplayer::client_advancements::send(&sinks.connection, packet);
            true
        }
        None => false,
    }
}

/// The `ContainerListener.slotChanged` calls a menu made, recorded for the screen that listens (`implements
/// ContainerListener`): the menu calls its listeners inside the packet handler's hold of the player, where the mod's
/// screen object cannot be reached, so the call is queued and the screen runs its `slotChanged` for each in its next
/// own call (`drain_slot_changes`). (Was a private copy in `leap_overlay` and `radial_menu_screen`.)
pub type SlotChangeQueue = std::sync::Arc<std::sync::Mutex<Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)>>>;

/// The listener the menu holds for a `SlotChangeQueue` (`name` is the listening class, for the menu's logs).
pub struct QueueSlotListener {
    pub pending: SlotChangeQueue,
    pub name: &'static str,
}

impl std::fmt::Debug for QueueSlotListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl mcrust::net::minecraft::world::inventory::container_listener::ContainerListener for QueueSlotListener {
    /// java: @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    fn slot_changed(&mut self, slot_index: i32, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        self.pending.lock().unwrap_or_else(|p| p.into_inner()).push((slot_index, item_stack.duplicate()));
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    fn data_changed(&mut self, id: i32, value: i32) {
        let _ = (id, value);
    }
}

/// `handler.addSlotListener(listener)` on the menu `fromPacket` built (not the player's yet): the port's takes the
/// `Containers` its `broadcastChanges` reads - the chest's own and the player's inventory. The menu half is moved out
/// of the `ChestMenu` for the call (the chest's containers borrow the rest of it) and put back.
pub fn add_slot_listener(
    handler: &mut mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu,
    listener: Box<dyn mcrust::net::minecraft::world::inventory::container_listener::ContainerListener>,
) {
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    let container_id = handler.menu.container_id;
    let mut menu = std::mem::replace(&mut handler.menu, mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu::new(container_id));
    mcrust::mcrust::modding::game::with_player(|player| {
        let (_, _, inventory) = player.player_mut().get_container_menu_and_inventory();
        let mut containers = handler.containers_for(inventory);
        menu.add_slot_listener(listener, &mut containers);
    });
    handler.menu = menu;
}

/// `handler.addSlotListener(this)` for a screen that queues its `slotChanged` calls: the queue the screen drains.
pub fn listen_slot_changes(handler: &mut mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu, name: &'static str) -> SlotChangeQueue {
    let pending: SlotChangeQueue = Default::default();
    add_slot_listener(handler, Box::new(QueueSlotListener { pending: pending.clone(), name }));
    pending
}

/// The queued `slotChanged` calls, in order (the queue is left empty).
pub fn take_slot_changes(queue: &SlotChangeQueue) -> Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> {
    std::mem::take(&mut *queue.lock().unwrap_or_else(|p| p.into_inner()))
}

/// `CompletableFuture.runAsync(runnable, executor).exceptionally(e -> { LOGGER.error(message, e); return null; })` for a
/// runnable over the mod's objects (`Arc<dyn Fn()>`, not `Send`): it runs deferred on the render thread
/// (`execute_local`), and a panic is the exceptional completion, logged. (Was `ItemRepository::run_async_logging`.)
pub fn run_async_logging(runnable: std::sync::Arc<dyn Fn()>, message: &'static str) {
    execute_local(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (runnable)()));
        if let Err(e) = result {
            crate::prelude::logger().error(&format!("{} {}", message, panic_message(&e)));
        }
    });
}

thread_local! {
    /// The render-thread futures `local_of` completes, by id: a worker's completion reaches the render thread as a
    /// `Send` task carrying the id and the value.
    static PENDING_LOCAL_FUTURES: std::cell::RefCell<std::collections::HashMap<u64, Box<dyn std::any::Any>>> = std::cell::RefCell::new(std::collections::HashMap::new());
}
static NEXT_LOCAL_FUTURE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// A worker's `CompletableFuture<T>` (`Send` values) continued on the render thread as a `LocalFuture<T>`: completed (or
/// failed) through `game::execute` when the worker's future completes - Java's `thenApplyAsync(.., minecraft)` hop.
/// (Was a private helper of `profileviewer2/profile_viewer.rs`.)
pub fn local_of<T: Clone + Send + 'static>(future: &mcrust::java::util::concurrent::completable_future::CompletableFuture<T>) -> LocalFuture<T> {
    let local = LocalFuture::<T>::new();
    let id = NEXT_LOCAL_FUTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    PENDING_LOCAL_FUTURES.with(|pending| pending.borrow_mut().insert(id, Box::new(local.clone())));
    future.when_complete(Box::new(move |result: Result<&T, &str>| {
        let result: Result<T, String> = result.map(|value| value.clone()).map_err(|e| e.to_string());
        mcrust::mcrust::modding::game::execute(move || {
            let Some(local) = PENDING_LOCAL_FUTURES.with(|pending| pending.borrow_mut().remove(&id)) else { return; };
            let Ok(local) = local.downcast::<LocalFuture<T>>() else { return; };
            match result {
                Ok(value) => {
                    local.complete(value);
                }
                Err(e) => {
                    local.complete_exceptionally(e);
                }
            }
        });
    }));
    local
}

/// `NEURepoFile.isFile()`.
///
/// rust: port gap - the NEU repo library (`io.github.moulberry.repo`) is a `crate::unresolved` placeholder with no file
/// behind it, and `NEURepoManager.tree(..)` answers no files (GAPS_L, GAPS_M #10); a placeholder file is no regular file.
/// (Was a private copy in `block_break_prediction.rs`.)
pub fn neu_repo_file_is_file(file: &crate::unresolved::NEURepoFile) -> bool {
    let _ = file;
    false
}

/// `new String(file.stream().readAllBytes())`.
///
/// rust: port gap - see `neu_repo_file_is_file` (GAPS_M #10): a placeholder file has no content. (Was a private copy in
/// `block_break_prediction.rs` and `attributes.rs`.)
pub fn neu_repo_file_read_all(file: &crate::unresolved::NEURepoFile) -> String {
    let _ = file;
    String::new()
}

/// `Class.forName(name)` - another mod's JVM class.
///
/// rust: port gap - SeerClient runs no JVM: there are no Java mod classes to load or reflect on (`Class.forName`,
/// `MethodHandles.Lookup.findStatic/findVirtual`, `Field.get`, `MethodHandle.invoke`), so the lookup fails as
/// `ClassNotFoundException` does in Java for an absent class (GAPS_batchF, GAPS_M #11). Every caller only gets here
/// after `FabricLoader.isModLoaded(..)` answered true for a SeerClient mod of that id. (Was a private copy in the
/// three `compatibility/*_compatibility.rs` files.)
pub fn jvm_class_for_name(name: &str) -> crate::unresolved::Class {
    panic!("ClassNotFoundException: {name}");
}

/// `MethodHandles.publicLookup()` - see `jvm_class_for_name` (the lookup exists; nothing can be found through it).
pub fn jvm_public_lookup() -> crate::unresolved::Lookup {
    crate::unresolved::Lookup
}

/// `lookup.findStatic(clazz, name, type)` / `findVirtual(..)` - see `jvm_class_for_name`.
pub fn jvm_find_method(_lookup: &crate::unresolved::Lookup, _clazz: &crate::unresolved::Class, name: &str) -> crate::unresolved::MethodHandle {
    panic!("NoSuchMethodException: {name}");
}

/// `methodHandle.invoke(..)` - see `jvm_class_for_name` (no handle can have been found).
pub fn jvm_invoke(_handle: &crate::unresolved::MethodHandle) -> Object {
    panic!("WrongMethodTypeException: no JVM method behind the handle");
}

/// `layout.defaultCellSetting().alignX()...` (Java's setters change the cell defaults in place): the port's
/// `LayoutSettings` setters answer a new value, so the chain is applied to the default and written back.
pub fn adjust_layout_settings(
    settings: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings,
    adjust: impl FnOnce(mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings) -> mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings,
) {
    *settings = adjust(*settings);
}
