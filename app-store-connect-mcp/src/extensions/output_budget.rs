//! Count encoded JSON bytes without allocating the serialized output.
use serde::Serialize;
use std::io::{self, Write};

#[derive(Default)]
struct Counter(usize);
impl Write for Counter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self.0.saturating_add(bytes.len());
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Serialization failures are treated as oversized, so callers fail closed.
pub(super) fn json_bytes<T: Serialize + ?Sized>(value: &T) -> usize {
    let mut counter = Counter::default();
    if serde_json::to_writer(&mut counter, value).is_err() {
        usize::MAX
    } else {
        counter.0
    }
}

/// Exact JSON array size, including brackets and commas. Rejected candidates
/// leave the budget unchanged, allowing a later smaller value to fit.
pub(super) struct ArrayBudget {
    limit: usize,
    bytes: usize,
    count: usize,
}
impl ArrayBudget {
    pub(super) fn new(limit: usize) -> Self {
        Self {
            limit,
            bytes: 2,
            count: 0,
        }
    }
    /// Accept values whose resulting array is at most the inclusive limit.
    pub(super) fn push<T: Serialize + ?Sized>(&mut self, value: &T) -> bool {
        let Some(bytes) = self
            .bytes
            .checked_add(usize::from(self.count != 0))
            .and_then(|bytes| bytes.checked_add(json_bytes(value)))
        else {
            return false;
        };
        if bytes > self.limit {
            return false;
        }
        self.bytes = bytes;
        self.count += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn counting_matches_utf8_and_json_escaping() {
        let value = json!({"é🙂":"quotes\" slash\\ newline\n tab\t control\u{0001}","rows":[null,true,3.75]});
        assert_eq!(
            json_bytes(&value),
            serde_json::to_vec(&value).unwrap().len()
        );
        assert_eq!(json_bytes("é🙂"), 8); // 6 UTF-8 bytes plus quotes
        assert_eq!(json_bytes("\n"), 4); // escaped newline plus quotes
    }
    #[test]
    fn array_budget_counts_empty_arrays_commas_and_exact_limits() {
        let mut too_small = ArrayBudget::new(2);
        assert_eq!(too_small.bytes, json_bytes(&Vec::<u8>::new()));
        assert!(!too_small.push(&0));
        assert_eq!(too_small.bytes, 2);
        let mut budget = ArrayBudget::new(5);
        assert!(budget.push(&0));
        assert_eq!(budget.bytes, 3);
        assert!(budget.push(&1));
        assert_eq!(budget.bytes, json_bytes(&[0, 1]));
        assert_eq!(budget.bytes, 5);
        assert!(!budget.push(&2));
        assert_eq!(budget.bytes, 5);
    }
    #[test]
    fn serialization_errors_fail_closed() {
        struct Bad;
        impl Serialize for Bad {
            fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("unsupported value"))
            }
        }
        assert_eq!(json_bytes(&Bad), usize::MAX);
        let mut budget = ArrayBudget::new(100);
        assert!(!budget.push(&Bad));
        assert_eq!(budget.bytes, 2);
        assert!(budget.push(&0));
    }
    #[test]
    fn oversized_rows_do_not_consume_capacity_for_smaller_rows() {
        let row = json!({"A":"x"});
        let exact = 2 + json_bytes(&row);
        let mut budget = ArrayBudget::new(exact);
        assert!(!budget.push(&json!({"A":"too long"})));
        assert_eq!(budget.bytes, 2);
        assert!(budget.push(&row));
        assert_eq!(budget.bytes, exact);
        assert!(!budget.push(&row));
    }
}
