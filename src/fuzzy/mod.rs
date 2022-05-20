use std::cmp::max;

use self::util::find_longest_match;

mod util;

// This function is from https://github.com/logannc/fuzzywuzzy-rs/blob/master/src/utils.rs with some modifications
fn _get_matching_blocks<'a>(a: &'a str, b: &'a str) -> Vec<(usize, usize, usize)> {
  let flipped;
  let (shorter, len1, longer, len2) = {
    let a_len = a.chars().count();
    let b_len = b.chars().count();
    if a_len <= b_len {
      flipped = false;
      (a, a_len, b, b_len)
    } else {
      flipped = true;
      (b, b_len, a, a_len)
    }
  };

  let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, len1, 0, len2)];
  let mut matching_blocks = Vec::new();
  while let Some((low1, high1, low2, high2)) = queue.pop() {
    let (i, j, k) = find_longest_match(shorter, longer, low1, high1, low2, high2);
    if k != 0 {
      matching_blocks.push((i, j, k));
      if low1 < i && low2 < j {
        queue.push((low1, i, low2, j));
      }
      if i + k < high1 && j + k < high2 {
        queue.push((i + k, high1, j + k, high2));
      }
    }
  }
  matching_blocks.sort_unstable(); // Is this necessary?
  let (mut i1, mut j1, mut k1) = (0, 0, 0);
  let mut non_adjacent = Vec::new();
  for (i2, j2, k2) in matching_blocks {
    if i1 + k1 == i2 && j1 + k1 == j2 {
      k1 += k2;
    } else {
      if k1 != 0 {
        non_adjacent.push((i1, j1, k1));
      }
      i1 = i2;
      j1 = j2;
      k1 = k2;
    }
  }
  if k1 != 0 {
    non_adjacent.push((i1, j1, k1));
  }
  non_adjacent.push((len1, len2, 0));
  non_adjacent
    .into_iter()
    .map(|(i, j, k)| if flipped { (j, i, k) } else { (i, j, k) })
    .collect()
}

pub type MatchingBlocks = (Vec<(usize, String)>, usize);
pub fn get_matching_blocks(a: &str, b: &str) -> MatchingBlocks {
  let mut blocks = _get_matching_blocks(&a.to_lowercase(), &b.to_lowercase());
  blocks.pop();

  let mut output = Vec::new();
  let mut total_len = 0;
  for (_, text_idx, len) in blocks {
    output.push((text_idx, b[text_idx..text_idx + len].to_string()));
    total_len += len;
  }

  (output, total_len)
}

pub fn get_score(a: &str, b: &str) -> usize {
  let a_len = a.chars().count();
  let b_len = b.chars().count();
  let max_len = max(a_len, b_len);
  let (blocks, matching_cars) = get_matching_blocks(a, b);

  let mut base_similarity = (matching_cars as f64) / (a_len as f64);

  for (index, _) in blocks {
    let is_word_boundary = index == 0 || b[index - 1..index] == *" ";
    if !is_word_boundary {
      base_similarity -= 0.5 / a_len as f64;
    }
  }

  let score = 100.0 * base_similarity * a_len as f64 / (a_len as f64 + (max_len - a_len) as f64 * 0.001);

  score.round() as usize
}