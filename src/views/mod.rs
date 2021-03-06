//! Contains views which are building blocks for `fields`.
mod autocomplete;
mod multiselect;

pub use self::autocomplete::Autocomplete;
pub use self::multiselect::Multiselect;

use cursive::views::SelectView;

/// Checks if `select` includes `to_check`.
fn is_value_from_select(select: &SelectView, to_check: &str) -> bool {
    let mut idx = 0;
    while let Some((_, v)) = select.get_item(idx) {
        idx += 1;
        if to_check == *v {
            return true;
        }
    }
    return false;
}
