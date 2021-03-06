extern crate test;

pub fn nth<T: Clone>(v: Vec<T>, idx: isize) -> Option<T> {
  match idx {
    d if d < 0 => {
      let uidx = idx.wrapping_abs() as usize;
      v.get(v.len() - uidx).cloned()
    }
    _ => v.get(idx.wrapping_abs() as usize).cloned(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn it_works() {
    // test with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(nth(v.clone(), 0), Some(1));
    assert_eq!(nth(v.clone(), -1), Some(10));
  }

  #[bench]
  fn bench_nth(b: &mut Bencher) {
    // benchmark with 10 items
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    b.iter(|| nth(v.clone(), 1));
  }
}
