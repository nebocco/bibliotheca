/// 区間を配列サイズに収まるように丸める。
///
/// 与えられた区間 `r` と `0..len` の共通部分を、有界な半開区間として返す。
///
/// # Examples
/// ```
/// use bibliotheca::utils::bounds::bounds_within;
///
/// assert_eq!(bounds_within(.., 7), 0..7);
/// assert_eq!(bounds_within(..=4, 7), 0..5);
/// ```
pub fn bounds_within<R>(r: R, len: usize) -> std::ops::Range<usize>
where
    R: std::ops::RangeBounds<usize>,
{
    use std::ops::Bound;
    let end = match r.end_bound() {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => len,
    }
    .min(len);
    let start = match r.start_bound() {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
        Bound::Unbounded => 0,
    }
    .min(end);
    start..end
}
