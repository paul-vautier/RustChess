pub fn add_usize(value: usize, offset: i32) -> usize {
    let val = if offset.is_negative() {
        match value.checked_sub(-offset as usize) {
            Some(new) => new,
            None => panic!("Index out of bounds {} {}", value, offset),
        }
    } else {
        match value.checked_add(offset as usize) {
            Some(new) => new,
            None => panic!("Index out of bounds {} + {}", value, offset),
        }
    };
    val
}
