pub trait SplitArray<'a> {
    fn split_array_exact<const N: usize>(self, sep: impl AsRef<str>) -> Option<[&'a str; N]>;
    fn split_array<const N: usize>(self, sep: impl AsRef<str>) -> Option<[&'a str; N]>;
}

impl<'a> SplitArray<'a> for &'a str {
    fn split_array_exact<const N: usize>(self, sep: impl AsRef<str>) -> Option<[&'a str; N]> {
        let mut iter = self.splitn(N + 1, sep.as_ref());
        let mut out = [""; N];
        for slot in out.iter_mut() {
            *slot = iter.next()?;
        }
        if iter.next().is_some() {
            return None;
        }
        Some(out)
    }
    fn split_array<const N: usize>(self, sep: impl AsRef<str>) -> Option<[&'a str; N]> {
        let mut iter = self.splitn(N, sep.as_ref());
        let mut out = [""; N];
        for slot in out.iter_mut() {
            *slot = iter.next()?;
        }
        Some(out)
    }
}
