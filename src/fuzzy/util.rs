// The slice_utf8, find_longest_match are from https://github.com/logannc/fuzzywuzzy-rs/blob/master/src/utils.rs with some modifications
// I did not want to import the whole crate for just one function.

pub(crate) fn slice_utf8(string: &str, low: usize, high: usize) -> &str {
  let char_count = string.chars().count();
  debug_assert!(!(low > high));
  debug_assert!(!(high > char_count));
  if low == high {
    return "";
  }
  let mut indices = string
    .char_indices()
    .enumerate()
    .map(|(char_offset, (byte_offset, _))| (byte_offset, char_offset));
  let low_index = indices
    .find(|(_, co)| *co == low)
    .expect("Beginning of slice not found.")
    .0;
  let mut indices = indices.skip_while(|(_, co)| *co != high);
  #[allow(clippy::or_fun_call)]
  let high_index = indices.next().map(|(bo, _)| bo).unwrap_or(string.len());
  &string[low_index..high_index]
}

pub(crate) fn find_longest_match<'a>(
  shorter: &'a str,
  longer: &'a str,
  low1: usize,
  high1: usize,
  low2: usize,
  high2: usize,
) -> (usize, usize, usize) {
  let longsub = slice_utf8(longer, low2, high2);
  let mut byte_to_char_map = vec![0; longsub.len()];
  longsub
    .char_indices()
    .enumerate()
    .for_each(|(char_offset, (byte_offset, _))| {
      byte_to_char_map[byte_offset] = char_offset;
    });
  let slen = high1 - low1;
  for size in (1..slen + 1).rev() {
    for start in 0..slen - size + 1 {
      let substr = slice_utf8(&shorter, low1 + start, low1 + start + size);
      if let Some((startb, matchstr)) = longsub.match_indices(substr).next() {
        return (
          low1 + start,
          low2 + byte_to_char_map[startb],
          matchstr.chars().count(),
        );
      }
    }
  }
  (low1, low2, 0)
}