pub struct Game {
    field: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>
}

impl Game {
    pub fn new(height: usize, width: usize) -> Game {
        Game {field: vec![vec![false; width]; height], next: vec![vec![false; width]; height]}
    }
    
    pub fn step(&mut self) -> &Vec<Vec<bool>> {
        let height = self.field.len();
        let width = self.field[0].len();
        for row in 0usize..height {
            for col in 0usize..width {
                let mut count = 0;
                let prev_row = row.checked_sub(1).unwrap_or(height -1);
                let next_row = if row +1 >= height {0} else {row +1};
                let prev_col = col.checked_sub(1).unwrap_or(width -1);
                let next_col = if col +1 >= width {0} else {col +1};
                if self.field[prev_row][prev_col] {count+=1};
                if self.field[prev_row][col] {count+=1};
                if self.field[prev_row][next_col] {count+=1};
                if self.field[row][prev_col] {count+=1};
                if self.field[row][next_col] {count+=1};
                if self.field[next_row][prev_col] {count+=1};
                if self.field[next_row][col] {count+=1};
                if self.field[next_row][next_col] {count+=1};

                self.next[row][col] =
                if self.field[row][col] {
                    if count == 2 || count == 3 {
                        true
                    } else {
                        false
                    }
                } else if count == 3 {
                    true
                }
                else {
                    false
                };
            }
        }
        std::mem::swap(&mut self.field, &mut self.next);
        &self.field
    }

    pub fn swap(&mut self, row: usize, col: usize) {
       self.field[row][col] = !self.field[row][col]; 
    }

    pub fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }
}
