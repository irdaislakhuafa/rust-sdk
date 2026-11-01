/// Returns a reference allocated on the heap, or helper for pointers.
pub fn ptr_to<T>(v: T) -> Box<T> {
    Box::new(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_to() {
        let p = ptr_to(42);
        assert_eq!(*p, 42);
    }
}
