use crate::vectors::Vec2D;

pub struct Board<T>{
    board: Vec<Vec<T>>,
    bounds: Vec2D
}

impl <T> Board<T>{

    pub fn new(board: Vec<Vec<T>>) -> Self{
        let bounds = Self::calc_bounds(&board);
        Self { board, bounds }
    }

    pub fn get(&self, pos: Vec2D) -> &T {
        self.board
            .get::<usize>(pos.y.try_into().unwrap())
            .unwrap()
            .get::<usize>(pos.x.try_into().unwrap())
            .unwrap()
    }

    pub fn is_in_bounds(&self, pos: Vec2D) -> bool{
        pos.is_in_bounds(self.bounds)
    }

    pub fn get_bounds(&self) -> Vec2D{
        self.bounds
    }

    fn calc_bounds(board: &[Vec<T>]) -> Vec2D {
        let width = board.first().unwrap().len();
        let height = board.len();
    
        Vec2D::new(width.try_into().unwrap(), height.try_into().unwrap())
    }
}