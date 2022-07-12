
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, get_grid_tile: impl Fn(usize, usize) -> T) -> Self {
        let mut grid = Vec::new();

        for i in 0..height {
            grid.push(Vec::new());

            for j in 0..width {
                grid.last_mut().unwrap().push(get_grid_tile(i, j));
            }
        }

        Grid(grid)
    }

    pub fn empty() -> Self {
        Grid(Vec::new())
    }
    
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: isize, j: isize) -> Option<&T> {
        if i < 0 || j < 0 || i >= self.0.len() as isize || j >= self.0[0].len() as isize {
            None
        } else {
            Some(&self.0[i as usize][j as usize])
        }
    }
    
    pub fn get_mut(&mut self, i: isize, j: isize) -> Option<&mut T> {
        if i < 0 || j < 0 || i >= self.0.len() as isize || j >= self.0[0].len() as isize {
            None
        } else {
            Some(&mut self.0[i as usize][j as usize])
        }
    }
    
    pub fn reorder_indeces(&self) -> Vec<(usize, usize)> {
        let mut accum = Vec::new();
        let (height, width) = (self.0.len(), self.0[0].len());

        for i in 0..height {
            for j in 0..width {
                accum.push((i, j));
            }
        }

        accum
    }

    pub fn reorder(&self) -> Vec<(usize, usize, &T)> {
        let mut accum = Vec::new();
        let (height, width) = (self.0.len(), self.0[0].len());

        for i in 0..height {
            for j in 0..width {
                accum.push((i, j, &self.0[i][j]))
            }
        }

        accum
    }

    pub fn reorder_mut<'a>(&'a mut self) -> Vec<(usize, usize, &'a mut T)> {
        let mut accum = Vec::new();
        let mut_grid: Vec<Vec<&mut T>> = self.0.iter_mut().map(|row| row.iter_mut().collect()).collect();

        for (i, row) in mut_grid.into_iter().enumerate() {
            for (j, item) in row.into_iter().enumerate() {
                accum.push((i, j, item))
            }
        }

        accum
    }
}

impl<T> std::fmt::Display for Grid<T> where T: std::fmt::Display + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_str: String = format!("+{}+", " ".repeat(2 * self.0[0].len()));
        
        for (_, j, t) in self.reorder().into_iter() {
            if j == 0 { grid_str += "\n" }

            grid_str += format!("{}", t).as_str()
        }
    
        grid_str += format!("\n+{}+", " ".repeat(2 * self.0[0].len())).as_str();
    
        write!(f, "{}", grid_str)
    }
}

impl<S, T> From<Vec<Vec<S>>> for Grid<T> where T: From<S> {
    fn from(grid: Vec<Vec<S>>) -> Self {
        Grid(
            grid.into_iter().map(|row| {
                row.into_iter().map(|item| T::from(item)).collect()
            }).collect()
        )
    }   
}

impl<T> From<&str> for Grid<T> where T: From<char> + Default {
    fn from(string: &str) -> Self {
        let mut grid = vec![Vec::new()];
        let mut max_width = 0;

        for char in string.chars().into_iter() {
            if char == '|' { 
                let width = grid.last().unwrap().len();
                if width > max_width { max_width = width }

                grid.push(Vec::new()); 

                continue; 
            }

            grid.last_mut().unwrap().push(T::from(char));
        }

        let mut i = 0;

        while i < grid.len() {
            if grid[i].len() == 0 { grid.remove(i); continue; }

            while grid[i].len() < max_width {
                grid[i].push(T::default())
            }

            i += 1;
        }

        Grid(
            grid
        )
    }
}

impl<T> Clone for Grid<T> where T: Clone {
    fn clone(&self) -> Self {
        Grid(
            self.0.clone()
        )
    }
}