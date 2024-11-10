pub struct Table<T = String, H = String> {
    header: Vec<H>,
    rows: Vec<Vec<T>>,
}

impl<T, H> Table<T, H> {
    pub fn new(header: impl IntoIterator<Item = impl Into<H>>) -> Self {
        Self {
            header: header.into_iter().map(|col_name| col_name.into()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn with_rows(
        self,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<T>>>,
    ) -> Self {
        let ans = Self {
            header: self.header,
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(|value| value.into()).collect())
                .collect(),
        };
        for row in &ans.rows {
            assert_eq!(row.len(), ans.width());
        }
        ans
    }

    pub fn new_with_rows(
        header: impl IntoIterator<Item = impl Into<H>>,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<T>>>,
    ) -> Self {
        Self::new(header).with_rows(rows)
    }

    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }

    pub fn header(&self) -> &Vec<H> {
        &self.header
    }

    pub fn get(&self, row_num: usize, col_num: usize) -> &T {
        &self.rows()[row_num][col_num]
    }

    pub fn width(&self) -> usize {
        self.header.len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}
