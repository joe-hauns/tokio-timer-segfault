use std::time::Instant;

/// Returns the instant in time that corresponds to the next timeout
/// scheduled in this wheel.
pub fn next_timeout(wheel: &Vec<Option<Instant>>) -> Option<Instant> {
    // TODO: can this be optimized to not look at the whole array?

    let mut min = None;
    for a in wheel.iter().filter_map(|s| s.as_ref()) {
        if let Some(b) = min {
            if b < a {
                // FIXME: optimization bug here
//                    print!("");
                continue
            }
        }
        min = Some(a);
    }

    min.map(|t| *t)
}
