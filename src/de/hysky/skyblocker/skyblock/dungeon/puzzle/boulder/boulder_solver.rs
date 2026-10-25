//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.boulder.BoulderSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class BoulderSolver
#[derive(Clone)]
pub struct BoulderSolver {}

impl BoulderSolver {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static @Nullable List<int[]> aStarSolve(List<GameState> initialStates)
    pub fn a_star_solve(initial_states: &Vec<GameState>) -> Option<Vec<Vec<i32>>> {
        let mut visited: std::collections::HashSet<GameState> = std::collections::HashSet::new();
        // PriorityQueue<Pair<GameState, List<int[]>>> queue = new PriorityQueue<>(new AStarComparator());
        let mut queue: PriorityQueue = PriorityQueue::new(AStarComparator::new());
        for initial_state in initial_states.iter() {
            queue.add((initial_state.clone(), Vec::new()));
        }
        let mut max_iterations: i32 = 10000;
        let mut iterations: i32 = 0;
        while !queue.is_empty() && iterations < max_iterations {
            let mut pair: (GameState, Vec<Vec<i32>>) = queue.poll().unwrap();
            let state = &pair.0;
            let path = &mut pair.1;
            if visited.contains(state) {
                continue;
            }
            visited.insert(state.clone());
            path.push(vec![state.player_x, state.player_y]);
            if state.is_solved() {
                return Some(path.clone());
            }
            for direction in vec![vec![-1, 0], vec![0, -1], vec![0, 1], vec![1, 0]] {
                let mut new_state: GameState = GameState::new(state.grid.clone(), state.player_x, state.player_y, state.moved_boxes);
                if new_state.move_player(direction[(0) as usize], direction[(1) as usize]) {
                    queue.add((new_state, path.clone()));
                }
            }
            iterations += 1;
        }
        return None;
    }

}

/// `java.util.PriorityQueue<Pair<GameState, List<int[]>>>` ordered by an `AStarComparator`: Java's binary heap
/// (`siftUp` / `siftDown` with the comparator, `offer` / `poll`), so elements of equal cost come out in the
/// order Java's heap gives them.
pub struct PriorityQueue {
    queue: Vec<(GameState, Vec<Vec<i32>>)>,
    comparator: AStarComparator,
}

impl PriorityQueue {
    pub fn new(comparator: AStarComparator) -> PriorityQueue {
        PriorityQueue { queue: Vec::new(), comparator }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// `add` = `offer`: append, then `siftUpUsingComparator`.
    pub fn add(&mut self, e: (GameState, Vec<Vec<i32>>)) {
        let mut k = self.queue.len();
        self.queue.push(e);
        while k > 0 {
            let parent = (k - 1) >> 1;
            if self.comparator.compare(&self.queue[k], &self.queue[parent]) >= 0 {
                break;
            }
            self.queue.swap(k, parent);
            k = parent;
        }
    }

    /// `poll`: the head; the last element is moved to the root and `siftDownUsingComparator`.
    pub fn poll(&mut self) -> Option<(GameState, Vec<Vec<i32>>)> {
        if self.queue.is_empty() {
            return None;
        }
        let last = self.queue.len() - 1;
        self.queue.swap(0, last);
        let result = self.queue.pop();
        let n = self.queue.len();
        let mut k = 0;
        let half = n >> 1;
        while k < half {
            let mut child = (k << 1) + 1;
            let right = child + 1;
            if right < n && self.comparator.compare(&self.queue[child], &self.queue[right]) > 0 {
                child = right;
            }
            if self.comparator.compare(&self.queue[k], &self.queue[child]) <= 0 {
                break;
            }
            self.queue.swap(k, child);
            k = child;
        }
        result
    }
}

/// java: private static class AStarComparator implements Comparator<Pair<GameState, List<int[]>>>
#[derive(Clone)]
pub struct AStarComparator {}

impl AStarComparator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public int compare(Pair<GameState, List<int[]>> a, Pair<GameState, List<int[]>> b)
    pub fn compare(&self, a: &(GameState, Vec<Vec<i32>>), b: &(GameState, Vec<Vec<i32>>)) -> i32 {
        let mut cost_a: i32 = ((a.1.len() as i32).wrapping_add(a.0.moved_boxes.wrapping_mul(2))).wrapping_add(a.0.heuristic());
        let mut cost_b: i32 = ((b.1.len() as i32).wrapping_add(b.0.moved_boxes.wrapping_mul(2))).wrapping_add(b.0.heuristic());
        return compat::ord_int(cost_a.cmp(&cost_b));
    }

}

impl PartialEq for GameState {
    /// `equals`: `Arrays.deepEquals(grid, ..)` and the three ints.
    fn eq(&self, other: &GameState) -> bool {
        self.grid == other.grid && self.player_x == other.player_x && self.player_y == other.player_y && self.moved_boxes == other.moved_boxes
    }
}

impl Eq for GameState {}

impl std::hash::Hash for GameState {
    /// `hashCode`: `Objects.hash(Arrays.deepHashCode(grid), playerX, playerY, movedBoxes)` - the same fields as
    /// `equals`.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
        self.player_x.hash(state);
        self.player_y.hash(state);
        self.moved_boxes.hash(state);
    }
}

/// java: public static class GameState
#[derive(Clone)]
pub struct GameState {
    /// java: private final char[][] grid;
    pub grid: Vec<Vec<char>>,
    /// java: private int playerX;
    pub player_x: i32,
    /// java: private int playerY;
    pub player_y: i32,
    /// java: private int movedBoxes;
    pub moved_boxes: i32,
}

impl GameState {
    /// java: public GameState(char[][] grid, int playerX, int playerY, int movedBoxes)
    pub fn new(mut grid: Vec<Vec<char>>, mut player_x: i32, mut player_y: i32, mut moved_boxes: i32) -> Self {
        let mut this = Self {
            grid: Default::default(),
            player_x: 0,
            player_y: 0,
            moved_boxes: 0,
        };
        this.grid = this.copy_grid(&grid);
        this.player_x = player_x;
        this.player_y = player_y;
        this.moved_boxes = moved_boxes;
        this
    }

    /// java: @Override public boolean equals(@Nullable Object obj)
    pub fn equals(&self, mut obj: Option<compat::Object>) -> bool {
        obj.as_ref().and_then(|o| o.downcast_ref::<GameState>()).is_some_and(|game_state| self.grid == game_state.grid && self.player_x == game_state.player_x && self.player_y == game_state.player_y && self.moved_boxes == game_state.moved_boxes)
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        // Arrays.hashCode / deepHashCode / Objects.hash: `31 * h + e` from 1
        let fold = |values: &mut dyn Iterator<Item = i32>| values.fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(e));
        let deep_hash_code: i32 = fold(&mut self.grid.iter().map(|row| fold(&mut row.iter().map(|c| *c as i32))));
        fold(&mut [deep_hash_code, self.player_x, self.player_y, self.moved_boxes].into_iter())
    }

    /// java: public boolean movePlayer(int dx, int dy)
    pub fn move_player(&mut self, mut dx: i32, mut dy: i32) -> bool {
        let mut new_x: i32 = self.player_x.wrapping_add(dx);
        let mut new_y: i32 = self.player_y.wrapping_add(dy);
        if self.is_valid_position(new_x, new_y) {
            if (self.grid[(new_x) as usize])[(new_y) as usize] == 'B' {
                let mut next_to_box_x: i32 = new_x.wrapping_add(dx);
                let mut next_to_box_y: i32 = new_y.wrapping_add(dy);
                if self.is_valid_position(next_to_box_x, next_to_box_y) && (self.grid[(next_to_box_x) as usize])[(next_to_box_y) as usize] == '.' {
                    (self.grid[(new_x) as usize])[(new_y) as usize] = '.';
                    (self.grid[(next_to_box_x) as usize])[(next_to_box_y) as usize] = 'B';
                    self.player_x = new_x;
                    self.player_y = new_y;
                    self.moved_boxes += 1;
                    return true;
                }
            } else {
                self.player_x = new_x;
                self.player_y = new_y;
                return true;
            }
        }
        return false;
    }

    /// java: private boolean isValidPosition(int x, int y)
    pub fn is_valid_position(&self, mut x: i32, mut y: i32) -> bool {
        return (x >= 0 && y >= 0 && x < (self.grid.len() as i32) && y < ((self.grid[(0) as usize]).len() as i32));
    }

    /// java: public boolean isSolved()
    pub fn is_solved(&self) -> bool {
        return (self.player_x == 0);
    }

    /// java: public int heuristic()
    pub fn heuristic(&self) -> i32 {
        return self.player_x.wrapping_abs().wrapping_add((self.player_y.wrapping_sub(((self.grid[(0) as usize]).len() as i32).wrapping_div(2))).wrapping_abs());
    }

    /// java: private char[][] copyGrid(char[][] original)
    pub fn copy_grid(&self, original: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut copy: Vec<Vec<char>> = vec![Vec::new(); ((original.len() as i32)) as usize];
        for i in 0..(original.len() as i32) {
            copy[(i) as usize] = (original[(i) as usize]).clone();
        }
        return copy;
    }

}
