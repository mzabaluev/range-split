use range_split::assert_str_range;

#[test]
fn assert_str_range_borrows() {
    let s = String::new();
    let r = 0..0;
    assert_str_range!(s, r);
    let _ = (s, r);
}
