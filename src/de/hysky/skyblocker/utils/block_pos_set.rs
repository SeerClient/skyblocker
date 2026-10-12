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
        // the field initialisers: capacity = 8, mask = capacity - 1,
        // nextGrowThreshold = Math.min((int) (this.capacity * LOAD_FACTOR), this.capacity - 1),
        // entries = new long[this.capacity + LANES - 1]
        let capacity: i32 = 8;
        let mut this = Self {
            base: crate::unresolved::AbstractSet,
            size: 0,
            tombstones: 0,
            capacity,
            mask: capacity.wrapping_sub(1),
            next_grow_threshold: (((capacity as f32) * LOAD_FACTOR) as i32).min(capacity.wrapping_sub(1)),
            entries: vec![0; (capacity.wrapping_add(LANES).wrapping_sub(1)) as usize],
        };
        this.entries.fill(*EMPTY);
        compat::shared(this)
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
        // java: return Long.rotateRight(l * 0xCB45348A28CB43BDL, 32); // 64 bit mcg
        return l.wrapping_mul(0xCB45348A28CB43BDu64 as i64).rotate_right(32);
    }

    /// java: public static long unhashLong(final long h)
    pub fn unhash_long(mut h: i64) -> i64 {
        // java: return Long.rotateLeft(h, 32) * 0xE3EBDD17C2778F95L;
        return h.rotate_left(32).wrapping_mul(0xE3EBDD17C2778F95u64 as i64);
    }

    /// java: public boolean containsXyz(int x, int y, int z)
    pub fn contains_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        compat::read(__this, |this| this.contains_xyz_body(x, y, z))
    }

    fn contains_xyz_body(&self, x: i32, y: i32, z: i32) -> bool {
        let hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mask: i32 = self.mask;
        let mut i: i32 = ((hash) as i32) & mask;
        loop {
            let n0: i64 = self.entries[(i.wrapping_add(0)) as usize];
            let n1: i64 = self.entries[(i.wrapping_add(1)) as usize];
            let n2: i64 = self.entries[(i.wrapping_add(2)) as usize];
            let n3: i64 = self.entries[(i.wrapping_add(3)) as usize];
            if n0 == hash || n1 == hash || n2 == hash || n3 == hash {
                return true;
            }
            if n0 == *EMPTY || n1 == *EMPTY || n2 == *EMPTY || n3 == *EMPTY {
                return false;
            }
            i = i.wrapping_add(LANES) & mask;
        }
    }

    /// java: public int indexOf(BlockPos pos)
    pub fn index_of(__this: &compat::Shared<Self>, pos: &BlockPos) -> i32 {
        compat::read(__this, |this| {
            let hash: i64 = BlockPosSet::hash_pos(pos);
            let mask: i32 = this.mask;
            let mut i: i32 = ((hash) as i32) & mask;
            loop {
                if this.entries[(i) as usize] == hash {
                    return i;
                }
                if this.entries[(i) as usize] == *EMPTY {
                    return -1;
                }
                i = i.wrapping_add(1) & mask;
            }
        })
    }

    /// java: public boolean addXYZ(int x, int y, int z)
    pub fn add_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        compat::write(__this, |this| this.add_xyz_body(x, y, z))
    }

    fn add_xyz_body(&mut self, x: i32, y: i32, z: i32) -> bool {
        if self.size >= self.next_grow_threshold {
            self.resize_body(self.capacity.wrapping_mul(2));
        } else if self.tombstones >= self.capacity.wrapping_sub(self.next_grow_threshold) {
            self.resize_body(self.capacity);
        }
        let hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mask: i32 = self.mask;
        let mut slot_to_insert: i32 = ((hash) as i32) & mask;
        let mut candidate_tombstone_slot: i32 = i32::MAX;
        // scan slots for empty spot
        loop {
            // already exists
            if self.entries[(slot_to_insert) as usize] == hash {
                return false;
            }
            // we can't insert immediately if we find a tombstone slot, because the entry could show up later
            else if self.entries[(slot_to_insert) as usize] == *TOMBSTONE && candidate_tombstone_slot == i32::MAX {
                candidate_tombstone_slot = slot_to_insert;
            }
            // found an absent slot
            else if self.entries[(slot_to_insert) as usize] == *EMPTY {
                // check if we found a tombstone to replace earlier
                if candidate_tombstone_slot != i32::MAX {
                    slot_to_insert = candidate_tombstone_slot;
                    // if we replaced a tombstone, reduce tombstone count
                    self.tombstones -= 1;
                }
                self.entries[(slot_to_insert) as usize] = hash;
                self.size += 1;
                // duplicate entries near the start to the extra padding at the end
                if slot_to_insert < LANES.wrapping_sub(1) {
                    let capacity = self.capacity;
                    self.entries[(slot_to_insert.wrapping_add(capacity)) as usize] = hash;
                }
                return true;
            }
            // wrap to beginning
            slot_to_insert = slot_to_insert.wrapping_add(1) & mask;
        }
    }

    /// java: public boolean removeXyz(int x, int y, int z)
    pub fn remove_xyz(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut z: i32) -> bool {
        compat::write(__this, |this| this.remove_xyz_body(x, y, z))
    }

    fn remove_xyz_body(&mut self, x: i32, y: i32, z: i32) -> bool {
        if self.size == 0 {
            return false;
        }
        let hash: i64 = BlockPosSet::hash_xyz(x, y, z);
        let mask: i32 = self.mask;
        let capacity: i32 = self.capacity;
        let mut i: i32 = ((hash) as i32) & mask;
        loop {
            if self.entries[(i) as usize] == hash {
                // replace with tombstone
                self.entries[(i) as usize] = *TOMBSTONE;
                if i < LANES.wrapping_sub(1) {
                    self.entries[(i.wrapping_add(capacity)) as usize] = *TOMBSTONE;
                }
                self.size -= 1;
                self.tombstones += 1;
                // travel backwards and attempt to remove tombstones if the next slot is empty
                if self.entries[((i.wrapping_add(1) & mask)) as usize] != *EMPTY {
                    return true;
                }
                loop {
                    self.entries[(i) as usize] = *EMPTY;
                    if i < LANES.wrapping_sub(1) {
                        self.entries[(i.wrapping_add(capacity)) as usize] = *EMPTY;
                    }
                    self.tombstones -= 1;
                    i = i.wrapping_sub(1) & mask;
                    if !(self.entries[(i) as usize] == *TOMBSTONE) {
                        break;
                    }
                }
                return true;
            } else if self.entries[(i) as usize] == *EMPTY {
                return false;
            }
            i = i.wrapping_add(1) & mask;
        }
    }

    /// java: public void resize(int newCapacity)
    pub fn resize(__this: &compat::Shared<Self>, mut new_capacity: i32) {
        compat::write(__this, |this| this.resize_body(new_capacity))
    }

    fn resize_body(&mut self, new_capacity: i32) {
        debug_assert!(new_capacity >= self.size, "capacity must be greater than size! (got: {}, size: {})", new_capacity, self.size);
        debug_assert!((new_capacity & new_capacity.wrapping_sub(1)) == 0, "capacity must be a power of two! (got: {})", new_capacity);
        let mask: i32 = new_capacity.wrapping_sub(1);
        let mut new_entries: Vec<i64> = vec![0; ((new_capacity.wrapping_add(LANES)).wrapping_sub(1)) as usize];
        new_entries.fill(*EMPTY);
        // iterate through the current entries array
        for current_index in 0..self.capacity {
            // skip tombstones and absents
            let hash: i64 = self.entries[(current_index) as usize];
            if hash >= *EMPTY {
                continue;
            }
            // add to new entries array
            let mut new_index: i32 = ((hash) as i32) & mask;
            loop {
                if new_entries[(new_index) as usize] == *EMPTY {
                    new_entries[(new_index) as usize] = hash;
                    if new_index < LANES.wrapping_sub(1) {
                        new_entries[(new_index.wrapping_add(new_capacity)) as usize] = hash;
                    }
                    break;
                }
                // wrap to beginning
                new_index = new_index.wrapping_add(1) & mask;
            }
        }
        // no more tombstones
        self.tombstones = 0;
        self.capacity = new_capacity;
        self.mask = new_capacity.wrapping_sub(1);
        self.next_grow_threshold = (((self.capacity as f32) * LOAD_FACTOR) as i32).min(self.capacity.wrapping_sub(1));
        self.entries = new_entries;
    }

    /// java: public void clearRetainingCapacity()
    pub fn clear_retaining_capacity(__this: &compat::Shared<Self>) {
        compat::write(__this, |this| {
            this.size = 0;
            this.entries.fill(*EMPTY);
        });
    }

    /// java: @Override public int size()
    pub fn size(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.size);
    }

    /// java: @Override public boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.size) == 0;
    }

    /// `o instanceof BlockPos pos` (a `MutableBlockPos` is one too): the position.
    fn object_pos(o: &(dyn std::any::Any + Send + Sync)) -> Option<BlockPos> {
        if let Some(pos) = o.downcast_ref::<BlockPos>() {
            return Some(*pos);
        }
        o.downcast_ref::<MutableBlockPos>().map(|pos| pos.immutable())
    }

    /// java: @Override public boolean contains(Object o)
    pub fn contains(__this: &compat::Shared<Self>, o: &compat::Object) -> bool {
        // java: return o instanceof BlockPos pos && this.contains(pos);
        return match BlockPosSet::object_pos(&**o) {
            Some(pos) => BlockPosSet::contains_block_pos(__this, &pos),
            None => false,
        };
    }

    /// java: public boolean contains(BlockPos pos)
    pub fn contains_block_pos(__this: &compat::Shared<Self>, pos: &BlockPos) -> bool {
        return BlockPosSet::contains_xyz(__this, pos.get_x(), pos.get_y(), pos.get_z());
    }

    /// java: @Override public Object[] toArray()
    pub fn to_array(__this: &compat::Shared<Self>) -> Vec<compat::Object> {
        let size: i32 = compat::read(__this, |__t| __t.size);
        let capacity: i32 = compat::read(__this, |__t| __t.capacity);
        let entries: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        let mut array: Vec<compat::Object> = Vec::with_capacity(size as usize);
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while i < capacity && j < size {
                let h: i64 = entries[(i) as usize];
                if h < *EMPTY {
                    array.push(std::sync::Arc::new(BlockPos::of(BlockPosSet::unhash_long(h))) as compat::Object);
                    j += 1;
                }
                i += 1;
            }
        }
        return array;
    }

    /// java: @Override public <T> T[] toArray(T[] a)
    pub fn to_array_1<T: From<BlockPos>>(__this: &compat::Shared<Self>, mut a: Vec<Option<T>>) -> Vec<Option<T>> {
        let size: i32 = compat::read(__this, |__t| __t.size);
        let capacity: i32 = compat::read(__this, |__t| __t.capacity);
        let entries: Vec<i64> = compat::read(__this, |__t| __t.entries.clone());
        if (a.len() as i32) < size {
            // java: a = (T[]) Array.newInstance(a.getClass().getComponentType(), size);
            a = (0..size).map(|_| None).collect();
        } else if (a.len() as i32) > size {
            a[(size) as usize] = None;
        }
        {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            while i < capacity && j < size {
                let h: i64 = entries[(i) as usize];
                if h < *EMPTY {
                    a[(j) as usize] = Some(T::from(BlockPos::of(BlockPosSet::unhash_long(h))));
                    j += 1;
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
        // java: return o instanceof BlockPos pos && this.remove(pos);
        return match BlockPosSet::object_pos(&**o) {
            Some(pos) => BlockPosSet::remove_block_pos(__this, &pos),
            None => false,
        };
    }

    /// java: public boolean remove(BlockPos pos)
    pub fn remove_block_pos(__this: &compat::Shared<Self>, pos: &BlockPos) -> bool {
        return BlockPosSet::remove_xyz(__this, pos.get_x(), pos.get_y(), pos.get_z());
    }

    /// `c.contains(pos)`: `BlockPos.equals` (Vec3i's: the same x, y, z).
    fn collection_contains(c: &Vec<compat::Object>, pos: &MutableBlockPos) -> bool {
        let pos = pos.immutable();
        c.iter().any(|e| BlockPosSet::object_pos(&**e) == Some(pos))
    }

    /// java: @Override public boolean retainAll(Collection<?> c)
    pub fn retain_all(__this: &compat::Shared<Self>, c: &Vec<compat::Object>) -> bool {
        compat::write(__this, |this| this.remove_where(|pos| !BlockPosSet::collection_contains(c, pos)))
    }

    /// The loop `retainAll` and `removeAll` both run in Java (written out twice there; `longs`/`entries` IS
    /// `this.entries`): every entry `remove_it` picks is removed in place.
    fn remove_where(&mut self, mut remove_it: impl FnMut(&MutableBlockPos) -> bool) -> bool {
        let mask: i32 = self.mask;
        let capacity: i32 = self.capacity;
        let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
        let mut removed_any: bool = false;
        {
            let mut i: i32 = capacity.wrapping_sub(1);
            while i >= 0 {
                'continue_1: {
                    let h: i64 = self.entries[(i) as usize];
                    if h >= *EMPTY {
                        break 'continue_1;
                    }
                    pos.set_long(BlockPosSet::unhash_long(h));
                    if !remove_it(&pos) {
                        break 'continue_1;
                    }
                    if self.entries[((i.wrapping_add(1) & mask)) as usize] != *EMPTY {
                        self.entries[(i) as usize] = *TOMBSTONE;
                        if i < LANES.wrapping_sub(1) {
                            self.entries[(i.wrapping_add(capacity)) as usize] = *TOMBSTONE;
                        }
                        self.size -= 1;
                        self.tombstones += 1;
                    } else {
                        // no need to use tombstones if the next slot is empty
                        let mut j: i32 = i;
                        self.size -= 1;
                        self.tombstones += 1;
                        loop {
                            self.entries[(j) as usize] = *EMPTY;
                            if j < LANES.wrapping_sub(1) {
                                self.entries[(j.wrapping_add(capacity)) as usize] = *EMPTY;
                            }
                            self.tombstones -= 1;
                            j = j.wrapping_sub(1) & mask;
                            if !(self.entries[(j) as usize] == *TOMBSTONE) {
                                break;
                            }
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
                removed_any |= BlockPosSet::remove(__this, e);
            }
            return removed_any;
        }
        compat::write(__this, |this| this.remove_where(|pos| BlockPosSet::collection_contains(c, pos)))
    }

    /// java: @Override public void clear()
    pub fn clear(__this: &compat::Shared<Self>) {
        BlockPosSet::clear_retaining_capacity(__this);
    }

    /// java: @Override public Iterator<BlockPos> iterator()
    pub fn iterator(__this: &compat::Shared<Self>) -> Iter {
        return Iter::new(__this.clone(), compat::read(__this, |__t| __t.capacity));
    }

    /// java: public Iterable<BlockPos.MutableBlockPos> iterateMut()
    pub fn iterate_mut(__this: &compat::Shared<Self>) -> IterMut {
        return IterMut::new(__this.clone(), compat::read(__this, |__t| __t.capacity));
    }

    /// java: public Iterable<BlockPos> destroyAndIterate()
    pub fn destroy_and_iterate(__this: &compat::Shared<Self>) -> OwningIter {
        return OwningIter::new(compat::read(__this, |__t| __t.size), __this.clone());
    }

    /// java: public Iterable<BlockPos.MutableBlockPos> destroyAndIterateMut()
    pub fn destroy_and_iterate_mut(__this: &compat::Shared<Self>) -> OwningIterMut {
        return OwningIterMut::new(compat::read(__this, |__t| __t.size), __this.clone());
    }

    /// java: @Override public BlockPosSet clone()
    pub fn clone(__this: &compat::Shared<Self>) -> compat::Shared<BlockPosSet> {
        // java: var cloned = (BlockPosSet) super.clone(); cloned.entries = cloned.entries.clone(); return cloned;
        // (Object.clone: a new object with the same field values; then its own copy of the array)
        compat::read(__this, |this| {
            compat::shared(BlockPosSet {
                base: crate::unresolved::AbstractSet,
                size: this.size,
                tombstones: this.tombstones,
                capacity: this.capacity,
                mask: this.mask,
                next_grow_threshold: this.next_grow_threshold,
                entries: this.entries.clone(),
            })
        })
    }

}

// NOTE(fabric2seer): add, clear, clone, contains, isEmpty, iterator, remove, removeAll, retainAll, size, toArray override(s) of AbstractSet - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// One slot of the set's array as an iterator holds it: Java hands the iterator the array ITSELF (`this.entries`),
/// so the iterator's writes (`remove`, the owning iterators' compaction) are writes to the set's array.
fn entry_at(set: &compat::Shared<BlockPosSet>, i: i32) -> i64 {
    compat::read(set, |t| t.entries[(i) as usize])
}

fn set_entry(set: &compat::Shared<BlockPosSet>, i: i32, value: i64) {
    compat::write(set, |t| t.entries[(i) as usize] = value);
}

/// java: private class Iter implements Iterator<BlockPos>
pub struct Iter {
    /// java: private int index;
    pub index: i32,
    /// java: private int nextIndex;
    pub next_index: i32,
    /// java: private final int capacity;
    pub capacity: i32,
    /// java: private final long[] entries; - the set's own array (and the inner class's `BlockPosSet.this`)
    pub set: compat::Shared<BlockPosSet>,
}

impl Iter {
    /// java: Iter(int capacity, long[] entries)
    pub fn new(set: compat::Shared<BlockPosSet>, mut capacity: i32) -> Self {
        let mut this = Self {
            index: i32::MIN,
            next_index: i32::MIN,
            capacity: 0,
            set,
        };
        for i in 0..capacity {
            if entry_at(&this.set, i) < *EMPTY {
                this.next_index = i;
                break;
            }
        }
        this.capacity = capacity;
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return self.next_index != i32::MIN;
    }

    /// java: @Override public void remove()
    pub fn remove(&mut self) {
        if self.index == i32::MIN {
            panic!("IllegalStateException");
        }
        set_entry(&self.set, self.index, *TOMBSTONE);
        if self.index < LANES.wrapping_sub(1) {
            set_entry(&self.set, self.index.wrapping_add(self.capacity), *TOMBSTONE);
        }
        // prevent this element from being removed again
        self.index = i32::MIN;
        compat::write(&self.set, |t| {
            t.size -= 1;
            t.tombstones += 1;
        });
    }
}

impl Iterator for Iter {
    type Item = BlockPos;

    /// java: @Override public BlockPos next() (`None` where Java's `hasNext()` is false and `next()` throws)
    fn next(&mut self) -> Option<BlockPos> {
        if !self.has_next() {
            return None;
        }
        self.index = self.next_index;
        loop {
            self.next_index += 1;
            if self.next_index >= self.capacity {
                self.next_index = i32::MIN;
                break;
            }
            if !(entry_at(&self.set, self.next_index) >= *EMPTY) {
                break;
            }
        }
        return Some(BlockPos::of(BlockPosSet::unhash_long(entry_at(&self.set, self.index))));
    }
}

/// java: private class IterMut implements Iterator<BlockPos.MutableBlockPos>, Iterable<BlockPos.MutableBlockPos>
pub struct IterMut {
    /// java: private int index;
    pub index: i32,
    /// java: private int nextIndex;
    pub next_index: i32,
    /// java: private final int capacity;
    pub capacity: i32,
    /// java: private final long[] entries; - the set's own array (and the inner class's `BlockPosSet.this`)
    pub set: compat::Shared<BlockPosSet>,
    /// java: private final BlockPos.MutableBlockPos pos;
    pub pos: MutableBlockPos,
}

impl IterMut {
    /// java: IterMut(int capacity, long[] entries)
    pub fn new(set: compat::Shared<BlockPosSet>, mut capacity: i32) -> Self {
        let mut this = Self {
            index: i32::MIN,
            next_index: i32::MIN,
            capacity: 0,
            set,
            pos: MutableBlockPos::new_origin(),
        };
        for i in 0..capacity {
            if entry_at(&this.set, i) < *EMPTY {
                this.next_index = i;
                break;
            }
        }
        this.capacity = capacity;
        this
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return self.next_index != i32::MIN;
    }

    /// java: @Override public void remove()
    pub fn remove(&mut self) {
        if self.index == i32::MIN {
            panic!("IllegalStateException");
        }
        set_entry(&self.set, self.index, *TOMBSTONE);
        if self.index < LANES.wrapping_sub(1) {
            set_entry(&self.set, self.index.wrapping_add(self.capacity), *TOMBSTONE);
        }
        // prevent this element from being removed again
        self.index = i32::MIN;
        compat::write(&self.set, |t| {
            t.size -= 1;
            t.tombstones += 1;
        });
    }

    /// java: @Override public Iterator<BlockPos.MutableBlockPos> iterator() - `return this;`
    pub fn iterator(self) -> Self {
        return self;
    }
}

impl Iterator for IterMut {
    type Item = MutableBlockPos;

    /// java: @Override public BlockPos.MutableBlockPos next() - Java answers the SAME mutable object each time; a
    /// `MutableBlockPos` here is a value, so each call answers that object's state after its `set`
    fn next(&mut self) -> Option<MutableBlockPos> {
        if !self.has_next() {
            return None;
        }
        self.index = self.next_index;
        loop {
            self.next_index += 1;
            if self.next_index >= self.capacity {
                self.next_index = i32::MIN;
                break;
            }
            if !(entry_at(&self.set, self.next_index) >= *EMPTY) {
                break;
            }
        }
        self.pos.set_long(BlockPosSet::unhash_long(entry_at(&self.set, self.index)));
        return Some(self.pos);
    }
}

/// `OwningIter` / `OwningIterMut`'s constructor: compacts the set's array in place.
fn compact(set: &compat::Shared<BlockPosSet>, size: i32) {
    compat::write(set, |t| {
        let entries = &mut t.entries;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        while j < size {
            if entries[(i) as usize] < *EMPTY {
                entries[(j) as usize] = entries[(i) as usize];
                j += 1;
            }
            i += 1;
        }
    });
}

/// java: private static class OwningIter implements Iterable<BlockPos>, Iterator<BlockPos>
pub struct OwningIter {
    /// java: private int i = 0;
    pub i: i32,
    /// java: private final int size;
    pub size: i32,
    /// java: private final long[] entries; - the set's own array
    pub set: compat::Shared<BlockPosSet>,
}

impl OwningIter {
    /// java: OwningIter(int size, long[] entries)
    pub fn new(mut size: i32, set: compat::Shared<BlockPosSet>) -> Self {
        compact(&set, size);
        Self { i: 0, size, set }
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return self.i < self.size;
    }

    /// java: public Iterator<BlockPos> iterator() - `return this;`
    pub fn iterator(self) -> Self {
        return self;
    }
}

impl Iterator for OwningIter {
    type Item = BlockPos;

    /// java: @Override public BlockPos next()
    fn next(&mut self) -> Option<BlockPos> {
        if self.i >= self.size {
            return None;
        }
        let l: i64 = BlockPosSet::unhash_long(entry_at(&self.set, self.i));
        self.i += 1;
        return Some(BlockPos::of(l));
    }
}

/// java: private static class OwningIterMut implements Iterable<BlockPos.MutableBlockPos>, Iterator<BlockPos.MutableBlockPos>
pub struct OwningIterMut {
    /// java: private int i = 0;
    pub i: i32,
    /// java: private final int size;
    pub size: i32,
    /// java: private final long[] entries; - the set's own array
    pub set: compat::Shared<BlockPosSet>,
    /// java: private final BlockPos.MutableBlockPos pos;
    pub pos: MutableBlockPos,
}

impl OwningIterMut {
    /// java: OwningIterMut(int size, long[] entries)
    pub fn new(mut size: i32, set: compat::Shared<BlockPosSet>) -> Self {
        compact(&set, size);
        Self { i: 0, size, set, pos: MutableBlockPos::new_origin() }
    }

    /// java: @Override public boolean hasNext()
    pub fn has_next(&self) -> bool {
        return self.i < self.size;
    }

    /// java: public Iterator<BlockPos.MutableBlockPos> iterator() - `return this;`
    pub fn iterator(self) -> Self {
        return self;
    }
}

impl Iterator for OwningIterMut {
    type Item = MutableBlockPos;

    /// java: @Override public BlockPos.MutableBlockPos next()
    fn next(&mut self) -> Option<MutableBlockPos> {
        if self.i >= self.size {
            return None;
        }
        self.pos.set_long(BlockPosSet::unhash_long(entry_at(&self.set, self.i)));
        self.i += 1;
        return Some(self.pos);
    }
}
