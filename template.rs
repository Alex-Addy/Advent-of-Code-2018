use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let lines: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    println!("Part 1: {}", do_work(&lines));
}

fn do_work<T: Deref<Target = str>>(lines: &[T]) -> usize {
}

#[cfg(test)]
mod test {
    use super::*;
}
