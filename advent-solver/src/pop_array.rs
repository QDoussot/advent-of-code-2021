pub enum PopArrayError {
    EmptyArrayNotAllowed,
    Missing(usize),
}

pub trait PopArray<T> {
    fn pop_array<const N: usize>(&mut self) -> Result<[T; N], PopArrayError>;
}

impl<T: Copy, I: Iterator<Item = T>> PopArray<T> for I {
    fn pop_array<const N: usize>(&mut self) -> Result<[T; N], PopArrayError> {
        if N == 0 {
            return Err(PopArrayError::EmptyArrayNotAllowed);
        }
        if let Some(item) = self.next() {
            let mut array = [item; N];

            let mut i = 1;
            while let Some(item) = (i < N).then(|| self.next()).flatten() {
                array[i] = item;
                i += 1;
            }
            if i == N {
                Ok(array)
            } else {
                Err(PopArrayError::Missing(N - i))
            }
        } else {
            Err(PopArrayError::Missing(N))
        }
    }
}
