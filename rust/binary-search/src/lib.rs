use std::cell::Cell;
use std::ops::Range;
pub fn find<A, T>(array: A, key: T) -> Option<usize>
where
    T: std::cmp::Ord + std::fmt::Debug,
    A: AsRef<[T]>,
{
    let arr = array.as_ref();
    let range = &Range {
        start: Cell::new(0),
        end: Cell::new(arr.len()),
    };

    if arr.len().eq(&0) {
        return None;
    }
    loop {
        let mut middle = (range.end.get() - range.start.get()) / 2 + range.start.get();
        if range.start.get().gt(&0) && range.start.get().eq(&(range.end.get() - 1)) {
            middle = range.end.get();
        }

        let Some(x) = arr.get(middle) else {
            break None;
        };
        if x.eq(&key) {
            break Some(middle);
        } else if x.lt(&key) {
            range.start.set(middle);
            continue;
        } else {
            if middle.eq(&0) {
                break None;
            }
            range.end.set(middle - 1);
            if range.start.get().gt(&0) && range.start.eq(&range.end) {
                break None;
            }
            continue;
        }
    }
}
