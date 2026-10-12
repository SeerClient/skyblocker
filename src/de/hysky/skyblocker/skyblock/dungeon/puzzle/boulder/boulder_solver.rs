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
        // TODO(fabric2seer): `new PriorityQueue(..)`
        //   java: PriorityQueue<Pair<GameState, List<int[]>>> queue = new PriorityQueue<>(new AStarComparator());
        let mut queue: crate::unresolved::PriorityQueue = todo!("fabric2seer: the initialiser of `queue` (TODO above)");
        for initial_state in initial_states.iter() {
            // TODO(fabric2seer): `.add(..)` on a PriorityQueue
            //   java: queue.add(Pair.of(initialState, new ArrayList<>()));
        }
        let mut max_iterations: i32 = 10000;
        let mut iterations: i32 = 0;
        // TODO(fabric2seer): the condition: `.isEmpty(..)` on a PriorityQueue
        //   java: !queue.isEmpty() && iterations < maxIterations
        while todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.poll(..)` on a PriorityQueue
            //   java: Pair<GameState, List<int[]>> pair = queue.poll();
            let mut pair: (GameState, Vec<Vec<i32>>) = todo!("fabric2seer: the initialiser of `pair` (TODO above)");
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
                    // TODO(fabric2seer): `.add(..)` on a PriorityQueue
                    //   java: queue.add(Pair.of(newState, new ArrayList<>(path)));
                }
            }
            iterations += 1;
        }
        return None;
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
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return obj instanceof GameState gameState && Arrays.deepEquals(grid, gameState.grid) && playerX == gameState.playerX && playerY == gameState.playerY && movedBoxes == gameState.movedBoxes;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        // TODO(fabric2seer): Arrays.deepHashCode
        //   java: return Objects.hash(Arrays.deepHashCode(grid), playerX, playerY, movedBoxes);
        todo!("fabric2seer: the value of this function is in a TODO above")
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
        let mut copy: Vec<Vec<char>> = vec!['\0'; ((original.len() as i32)) as usize];
        for i in 0..(original.len() as i32) {
            copy[(i) as usize] = (original[(i) as usize]).clone();
        }
        return copy;
    }

}
