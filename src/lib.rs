pub fn no_op() {}

// Note: The rest isn't pub...

struct Wrapper<T>(T);

impl<'a, T> IntoIterator for &'a Wrapper<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}
