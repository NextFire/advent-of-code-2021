#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(rows: Vec<Vec<T>>) -> Self {
        Matrix { rows }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.rows.get(i).and_then(|row| row.get(j))
    }

    pub fn set(&mut self, i: usize, j: usize, v: T) {
        self.rows[i][j] = v;
    }

    pub fn size(&self) -> (usize, usize) {
        let nb_row = self.rows.len();
        let nb_col: usize = self
            .rows
            .get(0)
            .and_then(|col| Some(col.len()))
            .unwrap_or(0);
        (nb_row, nb_col)
    }
}

impl<T: Eq> Matrix<T> {
    pub fn find(&self, x: T) -> Option<(usize, usize)> {
        match self
            .rows
            .iter()
            .map(|l| l.iter().position(|c| *c == x))
            .enumerate()
            .find(|(_, m)| m.is_some())
        {
            Some((i, Some(j))) => Some((i, j)),
            _ => None,
        }
    }
}
