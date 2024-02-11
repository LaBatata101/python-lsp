use smallvec::SmallVec;

use crate::call_path::CallPath;

pub fn collect_import_from_member<'a>(
    level: Option<u32>,
    module: Option<&'a str>,
    member: &'a str,
) -> CallPath<'a> {
    let mut call_path: CallPath = SmallVec::with_capacity(
        level.unwrap_or_default() as usize
            + module
                .map(|module| module.split('.').count())
                .unwrap_or_default()
            + 1,
    );

    // Include the dots as standalone segments.
    if let Some(level) = level {
        if level > 0 {
            for _ in 0..level {
                call_path.push(".");
            }
        }
    }

    // Add the remaining segments.
    if let Some(module) = module {
        call_path.extend(module.split('.'));
    }

    // Add the member.
    call_path.push(member);

    call_path
}

/// Format the call path for a relative import, or `None` if the relative import extends beyond
/// the root module.
pub fn from_relative_import<'a>(
    // The path from which the import is relative.
    module: &'a [String],
    // The path of the import itself (e.g., given `from ..foo import bar`, `[".", ".", "foo", "bar]`).
    import: &[&'a str],
    // The remaining segments to the call path (e.g., given `bar.baz`, `["baz"]`).
    tail: &[&'a str],
) -> Option<CallPath<'a>> {
    let mut call_path: CallPath = SmallVec::with_capacity(module.len() + import.len() + tail.len());

    // Start with the module path.
    call_path.extend(module.iter().map(String::as_str));

    // Remove segments based on the number of dots.
    for segment in import {
        if *segment == "." {
            if call_path.is_empty() {
                return None;
            }
            call_path.pop();
        } else {
            call_path.push(segment);
        }
    }

    // Add the remaining segments.
    call_path.extend_from_slice(tail);

    Some(call_path)
}
