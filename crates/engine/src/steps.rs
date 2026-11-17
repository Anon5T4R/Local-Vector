//! References between the steps of a batch (`localvector-cli run`, MCP `command_batch`).
//!
//! The implementation lives in [`craft_mcp::script`], shared with every Craft app: a parameter
//! string that is exactly `$N` or `$N.path` takes that value from the result of step `N` (0-based,
//! in the order the steps ran), and `$last.` from the step before. See that module for the path
//! syntax (`$1.id`, `$2.ids.0`, `$last.ids[0]`), the `$$` escape and the bounds on what one step
//! may copy in.
//!
//! Kept as a re-export so the engine's callers (the MCP dispatcher, the CLI) and their tests name
//! the engine they already use.

pub use craft_mcp::script::resolve;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn references_take_values_from_earlier_results() {
        let results = [json!({"index": 0}), json!({"id": 2}), json!({"ids": [5, 6]})];
        let p = json!({"id": "$1.id", "ids": ["$2.ids.1", "$last.ids[0]"], "nested": {"all": "$2"}, "n": 3, "flag": true});
        assert_eq!(resolve(&p, &results).unwrap(), json!({"id": 2, "ids": [6, 5], "nested": {"all": {"ids": [5, 6]}}, "n": 3, "flag": true}));
    }

    #[test]
    fn plain_strings_and_escapes_are_left_alone() {
        let results = [json!({"id": 2})];
        let p = json!({"text": "costs $1", "price": "$$1.99", "name": "a$1.id", "dollar": "$", "word": "$lastname"});
        assert_eq!(
            resolve(&p, &results).unwrap(),
            json!({"text": "costs $1", "price": "$1.99", "name": "a$1.id", "dollar": "$", "word": "$lastname"})
        );
    }

    #[test]
    fn a_reference_to_nothing_is_an_error() {
        let results = [json!({"id": 2})];
        for bad in ["$1", "$1.id", "$0.ids", "$0.id.x", "$last.nope", "$7[0]"] {
            let e = resolve(&json!({"id": bad}), &results).unwrap_err();
            assert!(e.contains(bad), "{bad}: {e}");
        }
        assert!(resolve(&json!({"id": "$last"}), &[]).is_err(), "no step before the first");
    }
}
