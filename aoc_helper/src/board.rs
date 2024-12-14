use crate::vectors::Vec2D;

pub struct Board<T> {
    board: Vec<Vec<T>>,
    bounds: Vec2D,
}

impl<'a, T> Board<T> {
    pub fn new(board: Vec<Vec<T>>) -> Self {
        let bounds = Self::calc_bounds(&board);
        Self { board, bounds }
    }

    /// Gets the element at given position
    /// # Examples
    /// ```
    /// use aoc_helper::board::Board;
    /// use aoc_helper::vectors::Vec2D;
    /// let board = Board::new(vec![vec![1, 2]]);
    /// assert_eq!(1, *board.get(Vec2D::new(0, 0)));
    /// assert_eq!(2, *board.get(Vec2D::new(1, 0)));
    /// ```
    pub fn get(&self, pos: Vec2D) -> &T {
        self.board
            .get::<usize>(pos.y.try_into().unwrap())
            .unwrap()
            .get::<usize>(pos.x.try_into().unwrap())
            .unwrap()
    }

    /// Checks whether a point is inside the board
    /// # Examples
    /// ```
    /// use aoc_helper::board::Board;
    /// use aoc_helper::vectors::Vec2D;
    /// let board = Board::new(vec![vec![1, 2]]);
    /// assert_eq!(true, board.is_in_bounds(Vec2D::new(1, 0)));
    /// assert_eq!(false, board.is_in_bounds(Vec2D::new(-1, 0)));
    /// assert_eq!(false, board.is_in_bounds(Vec2D::new(1, 2)));
    /// ```
    pub fn is_in_bounds(&self, pos: Vec2D) -> bool {
        pos.is_in_bounds(self.bounds)
    }

    /// Returns the bounds of the board
    /// # Examples
    /// ```
    /// use aoc_helper::board::Board;
    /// let board = Board::new(vec![vec![1, 2]]);
    /// let bounds = board.get_bounds();
    /// assert_eq!(1, bounds.y);
    /// assert_eq!(2, bounds.x);
    /// ```
    pub fn get_bounds(&self) -> Vec2D {
        self.bounds
    }

    pub fn iter_all_coordinates(&'a self) -> impl Iterator<Item = Vec2D> + use<'a, T> {
        let bounds = self.get_bounds();

        (0..bounds.y).flat_map(move |y| (0..bounds.x).map(move |x| Vec2D::new(x, y)))
    }

    fn calc_bounds(board: &[Vec<T>]) -> Vec2D {
        let width = board.first().unwrap().len();
        let height = board.len();

        Vec2D::new(width.try_into().unwrap(), height.try_into().unwrap())
    }
}
