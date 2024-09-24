use crate::types::ReadStatValue;

/// Check if the value is missing, by checking if the value is tagged missing.
///
/// # Arguments
/// * `value` - A `ReadStatValue` struct.
///
/// # Returns
/// A boolean value. If the value is missing, return `true`, otherwise return `false`.
///
/// # Original C function
/// ```c
/// void sas_assign_tag(readstat_value_t *value, uint8_t tag) {
///    /* We accommodate two tag schemes. In the first, the tag is an ASCII code
///     * given by uint8_t tag above. System missing is represented by an ASCII
///     * period. In the second scheme, (tag-2) is an offset from 'A', except when
///     * tag == 0, in which case it represents an underscore, or tag == 1, in
///     * which case it represents system-missing.
///     */
///    if (tag == 0) {
///        tag = '_';
///    } else if (tag >= 2 && tag < 28) {
///        tag = 'A' + (tag - 2);
///    }
///    if (sas_validate_tag(tag) == READSTAT_OK) {
///        value->tag = tag;
///        value->is_tagged_missing = 1;
///    } else {
///        value->tag = 0;
///        value->is_system_missing = 1;
///    }
///}
/// ```
///
/// /* Values can be missing in one of three ways:
/// * 1. "System missing", delivered to value handlers as NaN. Occurs in all file
/// *    types. The most common kind of missing value.
/// * 2. Tagged missing, also delivered as NaN, but with a single character tag
/// *    accessible via readstat_value_tag(). The tag might be 'a', 'b', etc,
/// *    corresponding to Stata's .a, .b, values etc. Occurs only in Stata and
/// *    SAS files.
/// * 3. Defined missing. The value is a real number but is to be treated as
/// *    missing according to the variable's missingness rules (such as "value < 0 ||
/// *    value == 999"). Occurs only in SPSS files. access the rules via:
/// *
/// *       readstat_variable_get_missing_ranges_count()
/// *       readstat_variable_get_missing_range_lo()
/// *       readstat_variable_get_missing_range_hi()
/// *
/// * Note that "ranges" include individual values where lo == hi.
/// *
/// * readstat_value_is_missing() is equivalent to:
/// *
/// *    (readstat_value_is_system_missing()
/// *     || readstat_value_is_tagged_missing()
/// *     || readstat_value_is_defined_missing())
/// */
pub fn is_tagged_missing(value: &ReadStatValue) -> bool {
    let tags = &value.tags;
    if tags.is_empty() {
        return false;
    } else {
        let tag = tags[0].as_bytes()[0];
        if tag == 0 {
            return true;
        } else if tag >= 2 && tag < 28 {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReadStatValue;

    #[test]
    fn test_is_tagged_missing() {
        let value = ReadStatValue::builder().build().unwrap();
        assert_eq!(is_tagged_missing(&value), false);
    }

    #[test]
    fn test_is_tagged_missing_with_tag_0() {
        let value = ReadStatValue::builder()
            .tags(vec![String::from("_")])
            .build()
            .unwrap();
        assert_eq!(is_tagged_missing(&value), true);
    }

    #[test]
    fn test_is_tagged_missing_with_tag_2() {
        let value = ReadStatValue::builder()
            .tags(vec![String::from("A")])
            .build()
            .unwrap();
        assert_eq!(is_tagged_missing(&value), true);
    }

    #[test]
    fn test_is_tagged_missing_with_tag_28() {
        let value = ReadStatValue::builder()
            .tags(vec![String::from("Z")])
            .build()
            .unwrap();
        assert_eq!(is_tagged_missing(&value), false);
    }
}
