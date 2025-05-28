#[macro_export]
macro_rules! match_accessors {
    ($accessors:expr, []) => {
        false
    };
    ($accessors:expr, [$($pattern:tt),* $(,)?]) => {
        {
            let mut iter = $accessors.iter();
            $crate::match_accessors_inner!(iter, $($pattern),*)
        }
    };
}

#[macro_export]
macro_rules! match_accessors_inner {
    ($iter:expr) => {
        $iter.next().is_none()
    };
    ($iter:expr, $key:literal) => {
        match $iter.next() {
            Some($crate::Accessor::Key(k)) if k == $key => $iter.next().is_none(),
            _ => false,
        }
    };
    ($iter:expr, _) => {
        match $iter.next() {
            Some(_) => $iter.next().is_none(),
            None => false,
        }
    };
    ($iter:expr, $key:literal, $($rest:tt),* $(,)?) => {
        match $iter.next() {
            Some($crate::Accessor::Key(k)) if k == $key => $crate::match_accessors_inner!($iter, $($rest),*),
            _ => false,
        }
    };
    ($iter:expr, _, $($rest:tt),* $(,)?) => {
        match $iter.next() {
            Some(_) => $crate::match_accessors_inner!($iter, $($rest),*),
            None => false,
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::accessor::Accessor;

    #[test]
    fn test_match_accessors() {
        let accessors = vec![
            Accessor::Key("tool".to_string()),
            Accessor::Key("uv".to_string()),
            Accessor::Key("sources".to_string()),
            Accessor::Key("local".to_string()),
            Accessor::Key("workspace".to_string()),
        ];

        // Exact match
        assert!(match_accessors!(
            &accessors,
            ["tool", "uv", "sources", "local", "workspace"]
        ));

        // Wildcard
        assert!(match_accessors!(
            &accessors,
            ["tool", "uv", "sources", _, "workspace"]
        ));

        // Partial match should fail
        assert!(!match_accessors!(&accessors, ["tool", "uv", "sources"]));

        // Mismatch
        assert!(!match_accessors!(&accessors, ["tool", "uv", "invalid"]));
        assert!(!match_accessors!(
            &accessors,
            ["tool", "uv", "sources", "local", "invalid"]
        ));

        // Different array length
        assert!(!match_accessors!(
            &accessors,
            ["tool", "uv", "sources", "local", "workspace", "extra"]
        ));

        // Empty accessors
        let empty_accessors: Vec<Accessor> = vec![];
        assert!(!match_accessors!(&empty_accessors, ["tool"]));
    }

    #[test]
    fn test_match_accessors_with_index() {
        let accessors = vec![
            Accessor::Key("tool".to_string()),
            Accessor::Key("uv".to_string()),
            Accessor::Index(0),
            Accessor::Key("workspace".to_string()),
        ];

        // Pattern with index
        assert!(match_accessors!(&accessors, ["tool", "uv", _, "workspace"]));

        // Pattern with specified index (should not match)
        assert!(!match_accessors!(
            &accessors,
            ["tool", "uv", "sources", "workspace"]
        ));
    }
}
