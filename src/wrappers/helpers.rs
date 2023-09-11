/// An iterator that can either be std::iter::none or `I`
///
/// This is mostly just used in `GraphState` to ensure we can consistently have
/// `ActionsIter` as an actual Iterator, rather than an `IntoIterator`, giving
/// us space (and maybe performance?) savings.
pub enum OptionIter<I> {
  Some(I),
  None,
}

impl<I: Iterator> Iterator for OptionIter<I> {
  type Item = I::Item;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      OptionIter::Some(iter) => iter.next(),
      OptionIter::None => None,
    }
  }
}
