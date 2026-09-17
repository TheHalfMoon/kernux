pub const fn baseline_marker() -> &'static str {
    "kernux"
}

#[cfg(test)]
mod tests {
    use super::baseline_marker;

    #[test]
    fn baseline_marker_is_stable() {
        assert_eq!(baseline_marker(), "kernux");
    }
}
