//! Ported from de.hysky.skyblocker.utils.BlockPosSet by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;

/// java: protected static final long TOMBSTONE = Long.MAX_VALUE;
pub static TOMBSTONE: LazyLock<i64> = LazyLock::new(|| i64::MAX);
/// java: protected static final long EMPTY = TOMBSTONE - 1;
pub static EMPTY: LazyLock<i64> = LazyLock::new(|| (*TOMBSTONE).wrapping_sub(1 as i64));
/// java: protected static final float LOAD_FACTOR = 0.75f;
pub const LOAD_FACTOR: f32 = 0.75f32;
/// java: protected static final int LANES = 4;
pub const LANES: i32 = 4;

/// java: public class BlockPosSet extends AbstractSet<BlockPos> implements Cloneable
pub struct BlockPosSet {
    /// java: extends java.util.AbstractSet
    pub base: crate::unresolved::AbstractSet,
    /// java: protected int size = 0;
    pub size: i32,
    /// java: protected int tombstones = 0;
    pub tombstones: i32,
    /// java: protected int capacity = 8;
    pub capacity: i32,
    /// java: protected int mask = capacity - 1;
    pub mask: i32,
    /// java: protected int nextGrowThreshold = Math.min((int) (this.capacity * LOAD_FACTOR), this.capacity - 1);
    pub next_grow_threshold: i32,
    /// java: protected long[] entries = new long[this.capacity + LANES - 1];
    pub entries: Vec<i64>,
}

impl std::ops::Deref for BlockPosSet {
    type Target = crate::unresolved::AbstractSet;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BlockPosSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BlockPosSet {
    /// java: public BlockPosSet()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): `new AbstractSet(..)`: the port has no `AbstractSet::new`
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        __this.lock().borrow_mut().entries.fill(*EMPTY);
        __this
    }

    /// java: public static long hashPos(BlockPos pos)
    pub fn hash_pos(pos: &BlockPos) -> i64 {
        return BlockPosSet::hash_xyz(pos.get_x(), pos.get_y(), pos.get_z());
    }

    /// java: public static long hashXyz(int x, int y, int z)
    pub fn hash_xyz(mut x: i32, mut y: i32, mut z: i32) -> i64 {
        return BlockPosSet::hash_long(BlockPos::pack(x, y, z));
    }

    /// java: public static long hashLong(final long l)
    pub fn hash_long(mut l: i64) -> i64 {
        // TODO(fabric2seer): Long.rotateRight
        //   java: return Long.rotateRight(l * 0xCB45348A28CB43BDL, 32);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static long unhashLong(final long h)
    pub fn unhash_long(mut h: i64) -> i64 {
        // TODO(fabric2seer): Long.rotateLeft
        //   java: return Long.rotateLeft(h, 32) * 0xE3EBDD17C2778F95L;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean containsXyz(int x, int y, int z)
    pub fn contains_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        let mut hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut i: i32 = (((hash) as i32) & mask);
        loop {
            let mut n0: i64 = __this.lock().borrow().entries[(i.wrapping_add(0)) as usize];
            let mut n1: i64 = __this.lock().borrow().entries[(i.wrapping_add(1)) as usize];
            let mut n2: i64 = __this.lock().borrow().entries[(i.wrapping_add(2)) as usize];
            let mut n3: i64 = __this.lock().borrow().entries[(i.wrapping_add(3)) as usize];
            if n0 == hash || n1 == hash || n2 == hash || n3 == hash {
                return true;
            }
            if n0 == *EMPTY || n1 == *EMPTY || n2 == *EMPTY || n3 == *EMPTY {
                return false;
            }
            i = (i.wrapping_add(LANES) & mask);
            if !(true) { break; }
        }
    }

    /// java: public int indexOf(BlockPos pos)
    pub fn index_of(__this: &compat::Shared<Self>, pos: &BlockPos) -> i32 {
        let mut hash: i64 = BlockPosSet::hash_pos(pos);
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut i: i32 = (((hash) as i32) & mask);
        loop {
            if __this.lock().borrow().entries[(i) as usize] == hash {
                return i;
            }
            if __this.lock().borrow().entries[(i) as usize] == *EMPTY {
                return -1;
            }
            i = (i.wrapping_add(1) & mask);
            if !(true) { break; }
        }
    }

    /// java: public boolean addXYZ(int x, int y, int z)
    pub fn add_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        if compat::read(__this, |__t| __t.size) >= compat::read(__this, |__t| __t.next_grow_threshold) {
            BlockPosSet::resize(__this, (compat::read(__this, |__t| __t.capacity)).wrapping_mul(2));
        } else if compat::read(__this, |__t| __t.tombstones) >= (compat::read(__this, |__t| __t.capacity)).wrapping_sub(compat::read(__this, |__t| __t.next_grow_threshold)) {
            BlockPosSet::resize(__this, compat::read(__this, |__t| __t.capacity));
        }
        let mut hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut slot_to_insert: i32 = (((hash) as i32) & mask);
        let mut candidate_tombstone_slot: i32 = i32::MAX;
        loop {
            if __this.lock().borrow().entries[(slot_to_insert) as usize] == hash {
                return false;
            } else if __this.lock().borrow().entries[(slot_to_insert) as usize] == *TOMBSTONE && candidate_tombstone_slot == i32::MAX {
                candidate_tombstone_slot = slot_to_insert;
            } else if __this.lock().borrow().entries[(slot_to_insert) as usize] == *EMPTY {
                if candidate_tombstone_slot != i32::MAX {
                    slot_to_insert = candidate_tombstone_slot;
                    { let __v = compat::read(__this, |__t| __t.tombstones) - 1; compat::write(__this, |__t| __t.tombstones = __v); }
                }
                __this.lock().borrow_mut().entries[(slot_to_insert) as usize] = hash;
                { let __v = compat::read(__this, |__t| __t.size) + 1; compat::write(__this, |__t| __t.size = __v); }
                if slot_to_insert < LANES.wrapping_sub(1) {
                    let __r0 = compat::read(__this, |__t| __t.capacity);
                    __this.lock().borrow_mut().entries[(slot_to_insert.wrapping_add(__r0)) as usize] = hash;
                }
                return true;
            }
            slot_to_insert = (slot_to_insert.wrapping_add(1) & mask);
            if !(true) { break; }
        }
    }

    /// java: public boolean removeXyz(int x, int y, int z)
    pub fn remove_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        if compat::read(__this, |__t| __t.size) == 0 {
            return false;
        }
        let mut hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut i: i32 = (((hash) as i32) & mask);
        loop {
            if __this.lock().borrow().entries[(i) as usize] == hash {
                __this.lock().borrow_mut().entries[(i) as usize] = *TOMBSTONE;
                if i < LANES.wrapping_sub(1) {
                    let __r0 = compat::read(__this, |__t| __t.capacity);
                    __this.lock().borrow_mut().entries[(i.wrapping_add(__r0)) as usize] = *TOMBSTONE;
                }
                { let __v = compat::read(__this, |__t| __t.size) - 1; compat::write(__this, |__t| __t.size = __v); }
                { let __v = compat::read(__this, |__t| __t.tombstones) + 1; compat::write(__this, |__t| __t.tombstones = __v); }
                if __this.lock().borrow().entries[((i.wrapping_add(1) & mask)) as usize] != *EMPTY {
                    return true;
                }
                loop {
                    __this.lock().borrow_mut().entries[(i) as usize] = *EMPTY;
                    if i < LANES.wrapping_sub(1) {
                        let __r0 = compat::read(__this, |__t| __t.capacity);
                        __this.lock().borrow_mut().entries[(i.wrapping_add(__r0)) as usize] = *EMPTY;
                    }
                    { let __v = compat::read(__this, |__t| __t.tombstones) - 1; compat::write(__this, |__t| __t.tombstones = __v); }
                    i = (i.wrapping_sub(1) & mask);
                    if !(__this.lock().borrow().entries[(i) as usize] == *TOMBSTONE) { break; }
                }
                return true;
            } else if __this.lock().borrow().entries[(i) as usize] == *EMPTY {
                return false;
            }
            i = (i.wrapping_add(1) & mask);
            if !(true) { break; }
        }
    }

    /// java: public void resize(int newCapacity)
    pub fn resize(__this: &compat::Shared<Self>, mut new_capacity: i32) {
        debug_assert!(new_capacity >= compat::read(__this, |__t| __t.size), "{}", format!("capacity must be greater than size! (got: {}, size: {})", new_capacity, compat::read(__this, |__t| __t.size)).to_string());
        debug_assert!((new_capacity & new_capacity.wrapping_sub(1)) == 0, "{}", format!("capacity must be a power of two! (got: {})", new_capacity).to_string());
        let mut mask: i32 = new_capacity.wrapping_sub(1);
        let mut new_entries: Vec<i64> = vec![0; ((new_capacity.wrapping_add(LANES)).wrapping_sub(1)) as usize];
        new_entries.fill(*EMPTY);
        for current_index in 0..compat::read(__this, |__t| __t.capacity) {
            let mut hash: i64 = __this.lock().borrow().entries[(current_index) as usize];
            if hash >= *EMPTY {
                continue;
            }
            let mut new_index: i32 = (((hash) as i32) & mask);
            loop {
                if new_entries[(new_index) as usize] == *EMPTY {
                    new_entries[(new_index) as usize] = hash;
                    if new_index < LANES.wrapping_sub(1) {
                        new_entries[(new_index.wrapping_add(new_capacity)) as usize] = hash;
                    }
                    break;
                }
                new_index = (new_index.wrapping_add(1) & mask);
            }
        }
        { let __v = 0; compat::write(__this, |__t| __t.tombstones = __v); }
        { let __v = new_capacity; compat::write(__this, |__t| __t.capacity = __v); }
        { let __v = new_capacity.wrapping_sub(1); compat::write(__this, |__t| __t.mask = __v); }
        { let __v = ((((compat::read(__this, |__t| __t.capacity) as f32) * LOAD_FACTOR)) as i32).min((compat::read(__this, |__t| __t.capacity)).wrapping_sub(1)); compat::write(__this, |__t| __t.next_grow_threshold = __v); }
        { let __v = new_entries.clone(); compat::write(__this, |__t| __t.entries = __v); }
    }

    /// java: public void clearRetainingCapacity()
    pub fn clear_retaining_capacity(__this: &compat::Shared<Self>) {
        { let __v = 0; compat::write(__this, |__t| __t.size = __v); }
        __this.lock().borrow_mut().entries.fill(*EMPTY);
    }

    /// java: @Override public int size()
    pub fn size(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.size);
    }

    /// java: @Override public boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(__this, |__t| __t.size) == 0);
    }

    /// java: @Override public boolean contains(Object o)
    pub fn contains(__this: &compat::Shared<Self>, o: &compat::Object) -> bool {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return o instanceof BlockPos pos && this.contains(pos);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean contains(BlockPos pos)
    pub fn contains_block_pos(__this: &compat::Shared<Self>, pos: &BlockPos) -> bool {
        return BlockPosSet::contains_xyz(__this, pos.get_x(), pos.get_y(), pos.get_z());
    }

    /// java: @Override public Object[] toArray()
    pub fn to_array(__this: &compat::Shared<Self>) -> Vec<compat::Object> {
        let mut size: i32 = compat::read(__this, |__t| __t.size);
        let mut capacity: i32 = compat::read(__this, |__t| __t.capacity);
        let mut entries: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        let mut array: Vec<compat::Object> = (0..(size) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while i < capacity && j < size {
                'continue_1: {
                    let mut h: i64 = entries[(i) as usize];
                    if h >= *EMPTY {
                        break 'continue_1;
                    }
                    array[({ let __v = j; j += 1; __v }) as usize] = (std::sync::Arc::new(BlockPos::of(BlockPosSet::unhash_long(h))) as compat::Object);
                }
                i += 1;
            }
        }
        return array;
    }

    /// java: @SuppressWarnings(
    pub fn to_array_1<T: 'static>(__this: &compat::Shared<Self>, mut a: Vec<T>) -> Vec<T> {
        let mut size: i32 = compat::read(__this, |__t| __t.size);
        let mut capacity: i32 = compat::read(__this, |__t| __t.capacity);
        let mut entries: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        if (a.len() as i32) < size {
            // TODO(fabric2seer): static call Array.newInstance(..)
            //   java: a = (T[]) Array.newInstance(a.getClass().getComponentType(), size);
        } else if (a.len() as i32) > size {
            a[(size) as usize] = None;
        }
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while i < capacity && j < size {
                'continue_1: {
                    let mut h: i64 = entries[(i) as usize];
                    if h >= *EMPTY {
                        break 'continue_1;
                    }
                    a[({ let __v = j; j += 1; __v }) as usize] = BlockPos::of(BlockPosSet::unhash_long(h));
                }
                i += 1;
            }
        }
        return a;
    }

    /// java: @Override public boolean add(BlockPos blockPos)
    pub fn add(__this: &compat::Shared<Self>, block_pos: &BlockPos) -> bool {
        return BlockPosSet::add_xyz(__this, block_pos.get_x(), block_pos.get_y(), block_pos.get_z());
    }

    /// java: @Override public boolean remove(Object o)
    pub fn remove(__this: &compat::Shared<Self>, o: &compat::Object) -> bool {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return o instanceof BlockPos pos && this.remove(pos);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean remove(BlockPos pos)
    pub fn remove_block_pos(__this: &compat::Shared<Self>, pos: &BlockPos) -> bool {
        return BlockPosSet::remove_xyz(__this, pos.get_x(), pos.get_y(), pos.get_z());
    }

    /// java: @Override public boolean retainAll(Collection<?> c)
    pub fn retain_all(__this: &compat::Shared<Self>, c: &Vec<compat::Object>) -> bool {
        let mut longs: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
        let mut removed_any: bool = false;
        {
            let mut i: i32 = (compat::read(__this, |__t| __t.capacity)).wrapping_sub(1);
            while i >= 0 {
                'continue_1: {
                    let mut h: i64 = longs[(i) as usize];
                    if h >= *EMPTY {
                        break 'continue_1;
                    }
                    pos.set_long(BlockPosSet::unhash_long(h));
                    if c.iter().any(|e| *e == pos.clone()) {
                        break 'continue_1;
                    }
                    if longs[((i.wrapping_add(1) & mask)) as usize] != *EMPTY {
                        longs[(i) as usize] = *TOMBSTONE;
                        if i < LANES.wrapping_sub(1) {
                            longs[(i.wrapping_add(compat::read(__this, |__t| __t.capacity))) as usize] = *TOMBSTONE;
                        }
                        { let __v = compat::read(__this, |__t| __t.size) - 1; compat::write(__this, |__t| __t.size = __v); }
                        { let __v = compat::read(__this, |__t| __t.tombstones) + 1; compat::write(__this, |__t| __t.tombstones = __v); }
                    } else {
                        let mut j: i32 = i;
                        { let __v = compat::read(__this, |__t| __t.size) - 1; compat::write(__this, |__t| __t.size = __v); }
                        { let __v = compat::read(__this, |__t| __t.tombstones) + 1; compat::write(__this, |__t| __t.tombstones = __v); }
                        loop {
                            longs[(j) as usize] = *EMPTY;
                            if j < LANES.wrapping_sub(1) {
                                longs[(j.wrapping_add(compat::read(__this, |__t| __t.capacity))) as usize] = *EMPTY;
                            }
                            { let __v = compat::read(__this, |__t| __t.tombstones) - 1; compat::write(__this, |__t| __t.tombstones = __v); }
                            j = (j.wrapping_sub(1) & mask);
                            if !(longs[(j) as usize] == *TOMBSTONE) { break; }
                        }
                    }
                    removed_any = true;
                }
                i -= 1;
            }
        }
        return removed_any;
    }

    /// java: @Override public boolean removeAll(Collection<?> c)
    pub fn remove_all(__this: &compat::Shared<Self>, c: &Vec<compat::Object>) -> bool {
        let mut removed_any: bool = false;
        if compat::read(__this, |__t| __t.size) > (c.len() as i32) {
            for e in c.iter() {
                removed_any = ((removed_any | BlockPosSet::remove(__this, e))) as bool;
            }
            return removed_any;
        }
        let mut entries: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        let mut mask: i32 = compat::read(__this, |__t| __t.mask);
        let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
        {
            let mut i: i32 = (compat::read(__this, |__t| __t.capacity)).wrapping_sub(1);
            while i >= 0 {
                'continue_2: {
                    let mut h: i64 = entries[(i) as usize];
                    if h >= *EMPTY {
                        break 'continue_2;
                    }
                    pos.set_long(BlockPosSet::unhash_long(h));
                    if !(c.iter().any(|e| *e == pos.clone())) {
                        break 'continue_2;
                    }
                    if entries[((i.wrapping_add(1) & mask)) as usize] != *EMPTY {
                        entries[(i) as usize] = *TOMBSTONE;
                        if i < LANES.wrapping_sub(1) {
                            entries[(i.wrapping_add(compat::read(__this, |__t| __t.capacity))) as usize] = *TOMBSTONE;
                        }
                        { let __v = compat::read(__this, |__t| __t.size) - 1; compat::write(__this, |__t| __t.size = __v); }
                        { let __v = compat::read(__this, |__t| __t.tombstones) + 1; compat::write(__this, |__t| __t.tombstones = __v); }
                    } else {
                        let mut j: i32 = i;
                        { let __v = compat::read(__this, |__t| __t.size) - 1; compat::write(__this, |__t| __t.size = __v); }
                        { let __v = compat::read(__this, |__t| __t.tombstones) + 1; compat::write(__this, |__t| __t.tombstones = __v); }
                        loop {
                            entries[(j) as usize] = *EMPTY;
                            if j < LANES.wrapping_sub(1) {
                                entries[(j.wrapping_add(compat::read(__this, |__t| __t.capacity))) as usize] = *EMPTY;
                            }
                            { let __v = compat::read(__this, |__t| __t.tombstones) - 1; compat::write(__this, |__t| __t.tombstones = __v); }
                            j = (j.wrapping_sub(1) & mask);
                            if !(entries[(j) as usize] == *TOMBSTONE) { break; }
                        }
                    }
                    removed_any = true;
                }
                i -= 1;
            }
        }
        return removed_any;
    }

    /// java: @Override public void clear()
    pub fn clear(__this: &compat::Shared<Self>) {
        BlockPosSet::clear_retaining_capacity(__this);
    }

    /// java: @Override public Iterator<BlockPos> iterator()
    pub fn iterator(__this: &compat::Shared<Self>) -> Box<dyn Iterator<Item = BlockPos> + '_> {
        return Box::new((Iter::new(compat::read(__this, |__t| __t.capacity), compat::read(__this, |__t| __t.entries.clone()))).iter().cloned());
    }

    /// java: public Iterable<BlockPos.MutableBlockPos> iterateMut()
    pub fn iterate_mut(__this: &compat::Shared<Self>) -> Vec<MutableBlockPos> {
        return IterMut::new(compat::read(__this, |__t| __t.capacity), compat::read(__this, |__t| __t.entries.clone()));
    }

    /// java: public Iterable<BlockPos> destroyAndIterate()
    pub fn destroy_and_iterate(__this: &compat::Shared<Self>) -> Vec<BlockPos> {
        return OwningIter::new(compat::read(__this, |__t| __t.size), compat::read(__this, |__t| __t.entries.clone()));
    }

    /// java: public Iterable<BlockPos.MutableBlockPos> destroyAndIterateMut()
    pub fn destroy_and_iterate_mut(__this: &compat::Shared<Self>) -> Vec<MutableBlockPos> {
        return OwningIterMut::new(compat::read(__this, |__t| __t.size), compat::read(__this, |__t| __t.entries.clone()));
    }

    /// java: @Override public BlockPosSet clone()
    pub fn clone(__this: &compat::Shared<Self>) -> compat::Shared<BlockPosSet> {
        // java: try {..} catch (CloneNotSupportedException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `AbstractSet.clone` has no Rust method in the port (looked for `clone` in AbstractSet and JavaSrc)
            // rust: let mut cloned = __this.lock().borrow_mut().base.clone();
            // TODO(fabric2seer): field `entries` of a value of unknown type
            //   java: cloned.entries = cloned.entries.clone();
            return cloned;
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                panic!("RuntimeException");
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// NOTE(fabric2seer): add, clear, clone, contains, isEmpty, iterator, remove, removeAll, retainAll, size, toArray override(s) of AbstractSet - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class Iter implements Iterator<BlockPos>
#[derive(Clone)]
pub struct Iter {
    /// java: private int index;
    pub index: i32,
    /// java: private int nextIndex;
    pub next_index: i32,
    /// java: private final int capacity;
    pub capacity: i32,
    /// java: private final long[] entries;
    pub entries: Vec<i64>,
}

impl Iter {
    /// java: Iter(int capacity, long[] entries)
    pub fn new(mut capacity: i32, mut entries: Vec<i64>) -> Self {
        let __init_index = i32::MIN;
        let __init_next_index = i32::MIN;
        let mut this = Self {
            index: __init_index,
            next_index: __init_next_index,
            capacity: 0,
            entries: Default::default(),
        };
        for i in 0..capacity {
            if entries[(i) as usize] < *EMPTY {
                this.next_index = i;
                break;
            }
        }
        this.capacity = capacity;
        this.entries = entries.clone();
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return (self.next_index != i32::MIN);
    }

    /// java: @Override public BlockPos next()
    pub fn next(&mut self) -> BlockPos {
        if !self.has_next() {
            panic!("NoSuchElementException");
        }
        self.index = self.next_index;
        loop {
            self.next_index += 1;
            if self.next_index >= self.capacity {
                self.next_index = i32::MIN;
                break;
            }
            if !(self.entries[(self.next_index) as usize] >= *EMPTY) { break; }
        }
        return BlockPos::of(BlockPosSet::unhash_long(self.entries[(self.index) as usize]));
    }

    /// java: @Override public void remove()
    pub fn remove(&mut self) {
        if self.index == i32::MIN {
            panic!("IllegalStateException");
        }
        self.entries[(self.index) as usize] = *TOMBSTONE;
        if self.index < LANES.wrapping_sub(1) {
            self.entries[(self.index.wrapping_add(self.capacity)) as usize] = *TOMBSTONE;
        }
        self.index = i32::MIN;
        // TODO(fabric2seer): `BlockPosSet.this` (the enclosing instance of an inner class)
        //   java: BlockPosSet.this.size--;
        // TODO(fabric2seer): `BlockPosSet.this` (the enclosing instance of an inner class)
        //   java: BlockPosSet.this.tombstones++;
    }

}

/// java: private class IterMut implements Iterator<BlockPos.MutableBlockPos>, Iterable<BlockPos.MutableBlockPos>
#[derive(Clone)]
pub struct IterMut {
    /// java: private int index;
    pub index: i32,
    /// java: private int nextIndex;
    pub next_index: i32,
    /// java: private final int capacity;
    pub capacity: i32,
    /// java: private final long[] entries;
    pub entries: Vec<i64>,
    /// java: private final BlockPos.MutableBlockPos pos;
    pub pos: Option<MutableBlockPos>,
}

impl IterMut {
    /// java: IterMut(int capacity, long[] entries)
    pub fn new(mut capacity: i32, mut entries: Vec<i64>) -> Self {
        let __init_index = i32::MIN;
        let __init_next_index = i32::MIN;
        let mut this = Self {
            index: __init_index,
            next_index: __init_next_index,
            capacity: 0,
            entries: Default::default(),
            pos: None,
        };
        for i in 0..capacity {
            if entries[(i) as usize] < *EMPTY {
                this.next_index = i;
                break;
            }
        }
        this.capacity = capacity;
        this.entries = entries.clone();
        this.pos = Some(MutableBlockPos::new_origin());
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return (self.next_index != i32::MIN);
    }

    /// java: @Override public BlockPos.MutableBlockPos next()
    pub fn next(&mut self) -> MutableBlockPos {
        if !self.has_next() {
            panic!("NoSuchElementException");
        }
        self.index = self.next_index;
        loop {
            self.next_index += 1;
            if self.next_index >= self.capacity {
                self.next_index = i32::MIN;
                break;
            }
            if !(self.entries[(self.next_index) as usize] >= *EMPTY) { break; }
        }
        { let __s0 = BlockPosSet::unhash_long(self.entries[(self.index) as usize]); self.pos.as_mut().unwrap().set_long(__s0) };
        return self.pos.clone().unwrap();
    }

    /// java: @Override public void remove()
    pub fn remove(&mut self) {
        if self.index == i32::MIN {
            panic!("IllegalStateException");
        }
        self.entries[(self.index) as usize] = *TOMBSTONE;
        if self.index < LANES.wrapping_sub(1) {
            self.entries[(self.index.wrapping_add(self.capacity)) as usize] = *TOMBSTONE;
        }
        self.index = i32::MIN;
        // TODO(fabric2seer): `BlockPosSet.this` (the enclosing instance of an inner class)
        //   java: BlockPosSet.this.size--;
        // TODO(fabric2seer): `BlockPosSet.this` (the enclosing instance of an inner class)
        //   java: BlockPosSet.this.tombstones++;
    }

    /// java: @Override public Iterator<BlockPos.MutableBlockPos> iterator()
    pub fn iterator(&self) -> Box<dyn Iterator<Item = MutableBlockPos> + '_> {
        return Box::new(self.iter().cloned());
    }

}

/// java: private static class OwningIter implements Iterable<BlockPos>, Iterator<BlockPos>
#[derive(Clone)]
pub struct OwningIter {
    /// java: private int i = 0;
    pub i: i32,
    /// java: private final int size;
    pub size: i32,
    /// java: private final long[] entries;
    pub entries: Vec<i64>,
}

impl OwningIter {
    /// java: OwningIter(int size, long[] entries)
    pub fn new(mut size: i32, mut entries: Vec<i64>) -> Self {
        let mut this = Self {
            i: 0,
            size: 0,
            entries: Default::default(),
        };
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while j < size {
                'continue_1: {
                    if entries[(i) as usize] >= *EMPTY {
                        break 'continue_1;
                    }
                    entries[(j) as usize] = entries[(i) as usize];
                    j += 1;
                }
                i += 1;
            }
        }
        this.entries = entries.clone();
        this.size = size;
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return (self.i < self.size);
    }

    /// java: @Override public BlockPos next()
    pub fn next(&mut self) -> BlockPos {
        if self.i >= self.size {
            panic!("NoSuchElementException");
        }
        let mut l: i64 = BlockPosSet::unhash_long(self.entries[(self.i) as usize]);
        self.i += 1;
        return BlockPos::of(l);
    }

    /// java: public Iterator<BlockPos> iterator()
    pub fn iterator(&self) -> Box<dyn Iterator<Item = BlockPos> + '_> {
        return Box::new(self.iter().cloned());
    }

}

/// java: private static class OwningIterMut implements Iterable<BlockPos.MutableBlockPos>, Iterator<BlockPos.MutableBlockPos>
#[derive(Clone)]
pub struct OwningIterMut {
    /// java: private int i = 0;
    pub i: i32,
    /// java: private final int size;
    pub size: i32,
    /// java: private final long[] entries;
    pub entries: Vec<i64>,
    /// java: private final BlockPos.MutableBlockPos pos;
    pub pos: Option<MutableBlockPos>,
}

impl OwningIterMut {
    /// java: OwningIterMut(int size, long[] entries)
    pub fn new(mut size: i32, mut entries: Vec<i64>) -> Self {
        let mut this = Self {
            i: 0,
            size: 0,
            entries: Default::default(),
            pos: None,
        };
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while j < size {
                'continue_1: {
                    if entries[(i) as usize] >= *EMPTY {
                        break 'continue_1;
                    }
                    entries[(j) as usize] = entries[(i) as usize];
                    j += 1;
                }
                i += 1;
            }
        }
        this.entries = entries.clone();
        this.size = size;
        this.pos = Some(MutableBlockPos::new_origin());
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return (self.i < self.size);
    }

    /// java: @Override public BlockPos.MutableBlockPos next()
    pub fn next(&mut self) -> MutableBlockPos {
        if self.i >= self.size {
            panic!("NoSuchElementException");
        }
        { let __s0 = BlockPosSet::unhash_long(self.entries[(self.i) as usize]); self.pos.as_mut().unwrap().set_long(__s0) };
        self.i += 1;
        return self.pos.clone().unwrap();
    }

    /// java: public Iterator<BlockPos.MutableBlockPos> iterator()
    pub fn iterator(&self) -> Box<dyn Iterator<Item = MutableBlockPos> + '_> {
        return Box::new(self.iter().cloned());
    }

}
