use std::iter;
use std::ops;

pub struct Matrix {
    pub shape: Shape,
    pub unrolled: Vec<f32>,
}

impl <'a> ops::Index<&'a Index> for Matrix {
    type Output = f32;
    fn index(&self, index: &'a Index) -> &f32 {
        &self.unrolled[index.in_unrolled(&self.shape)]
    }
}

impl <'a> ops::IndexMut<&'a Index> for Matrix {
    fn index_mut(&mut self, index: &Index) -> &mut f32 {
        &mut self.unrolled[index.in_unrolled(&self.shape)]
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// Dimensions and their lengths
pub struct Shape(pub Vec<usize>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Index(pub Vec<usize>);

impl Index {
    fn in_unrolled(&self, shape: &Shape) -> usize {
            self.0.iter().zip(iter::once(&1).chain(shape.0.iter())).map(|(ix, prev_dim_length)| ix * prev_dim_length).sum()
    }
}

impl Shape {
    pub fn unrolled_length(&self) -> usize {
        self.0.iter().product()
    }

    pub fn indices(&self) -> ShapeIndices {
        ShapeIndices { shape: self, index: None }
    }

    pub fn accomodates(&self, ixs: &Index) -> bool {
        (ixs.0.len() <= self.0.len()) && ixs.0.iter().zip(self.0.iter()).all(|(ix, s)| ix < s)
    }
}

pub struct ShapeIndices<'a> { shape: &'a Shape, index: Option<Index> }

impl<'a> Iterator for ShapeIndices<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Index> {
        match self.index {
            None => if self.shape.0.is_empty() {
                None
            } else {
                self.index = Some(Index(self.shape.0.iter().map(|_| 0).collect()));
                self.index.clone()
            },
            Some(ref mut index) => {
                for (s_ix, &s) in self.shape.0.iter().enumerate() {
                    if index.0[s_ix] == (s - 1) {
                        index.0[s_ix] = 0;
                    } else {
                        index.0[s_ix] += 1;
                        return Some(index.clone());
                    }
                }
                None
            }
        }
    }
}

pub fn zeros(shape: &Shape) -> Matrix {
    Matrix {
        shape: shape.clone(),
        unrolled: iter::repeat(0.0).take(shape.unrolled_length()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use mat::*;

    #[test]
    fn enumerates_indices() {
        assert_eq!(
            Shape(vec![3, 3, 3]).indices().collect::<Vec<Index>>(),
            vec![
                Index(vec![0, 0, 0]),
                Index(vec![1, 0, 0]),
                Index(vec![2, 0, 0]),
                Index(vec![0, 1, 0]),
                Index(vec![1, 1, 0]),
                Index(vec![2, 1, 0]),
                Index(vec![0, 2, 0]),
                Index(vec![1, 2, 0]),
                Index(vec![2, 2, 0]),
                Index(vec![0, 0, 1]),
                Index(vec![1, 0, 1]),
                Index(vec![2, 0, 1]),
                Index(vec![0, 1, 1]),
                Index(vec![1, 1, 1]),
                Index(vec![2, 1, 1]),
                Index(vec![0, 2, 1]),
                Index(vec![1, 2, 1]),
                Index(vec![2, 2, 1]),
                Index(vec![0, 0, 2]),
                Index(vec![1, 0, 2]),
                Index(vec![2, 0, 2]),
                Index(vec![0, 1, 2]),
                Index(vec![1, 1, 2]),
                Index(vec![2, 1, 2]),
                Index(vec![0, 2, 2]),
                Index(vec![1, 2, 2]),
                Index(vec![2, 2, 2]),
            ]
        )
    }

    #[test]
    fn unrolled_index() {
//        assert_eq!(Index(vec![9]).in_unrolled(&Shape(vec![3])), None);
        assert_eq!(Index(vec![5]).in_unrolled(&Shape(vec![9])), 5);
        assert_eq!(Index(vec![5, 4]).in_unrolled(&Shape(vec![9, 6])), 5 + 4 * 9);
    }
}