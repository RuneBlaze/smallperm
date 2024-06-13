#[cfg(test)]
mod tests {
    use crate::feistel::integer_log2;

    #[test]
    fn test_integer_log() {
        assert_eq!(None, integer_log2(0), "failed for {}", 0);
        assert_eq!(Some(1), integer_log2(1), "failed for {}", 1);
        assert_eq!(Some(2), integer_log2(2), "failed for {}", 2);
        assert_eq!(Some(2), integer_log2(3), "failed for {}", 3);
        assert_eq!(Some(3), integer_log2(4), "failed for {}", 4);
        assert_eq!(Some(3), integer_log2(5), "failed for {}", 5);
        assert_eq!(Some(3), integer_log2(6), "failed for {}", 6);
        assert_eq!(Some(3), integer_log2(7), "failed for {}", 7);
        assert_eq!(Some(4), integer_log2(8), "failed for {}", 8);
        assert_eq!(Some(4), integer_log2(9), "failed for {}", 9);
        assert_eq!(Some(4), integer_log2(10), "failed for {}", 10);
    }
}
