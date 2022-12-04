/**
 * The leaderboard module.
 *
 * This module contains the leaderboard implementation.
 */
pub struct Leaderboard<T> {
    arr: Vec<T>,
    len: usize,
}

impl <T> Leaderboard<T> where T: Ord + Clone {
    /**
     * Create a new leaderboard.
     *
     * # Arguments
     *
     * * `size` - The size of the leaderboard.
     */
    pub fn new(size: usize) -> Leaderboard<T> {
        Leaderboard {
            arr: Vec::with_capacity(size),
            len: 0,
        }
    }

    /** Add a new score to the leaderboard.
     *
     * # Arguments
     *
     * * `score` - The score to add.
     */
    pub fn append(&mut self, item: T) {
        self.arr.push(item);
        self.arr.sort();
        self.arr.reverse();

        if self.len < self.arr.len() {
            self.arr.pop();
        }
    }

    /**
     * Get the current leaderboard.
     */
    pub fn get_leaderboard(&self) -> Vec<T> {
        self.arr.clone()
    }

    /**
     * Get the current leaderboard size.
     */
    pub fn get_size(&self) -> usize {
        self.len 
    }
}
