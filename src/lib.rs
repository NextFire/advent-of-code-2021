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
