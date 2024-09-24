use chrono::DateTime;

/// The number of seconds between the Unix epoch and the SAS epoch.
fn get_sas_epoch_offset() -> i128 {
    let sas_epoch = DateTime::parse_from_rfc3339("1960-01-01T00:00:00Z").unwrap();
    let unix_epoch = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap();
    (sas_epoch - unix_epoch + chrono::Duration::days(1)).num_seconds() as i128
}

/// Max and min values for a SAS timestamp.
fn get_max_long() -> i128 {
    2_i128.pow(63) - 1
}

fn get_min_long() -> i128 {
    -(2_i128.pow(63)) - 1
}

/// Convert a SAS timestamp to a Unix timestamp.
pub fn sas_to_unix(sas_timestamp: i128) -> i128 {
    let epoch_offset = get_sas_epoch_offset();
    let unix_timestamp = sas_timestamp - epoch_offset;

    let max_long = get_max_long();
    let min_long = get_min_long();

    println!("max long: {} | min_long: {}", max_long, min_long);
    println!("unix_timestamp: {}", unix_timestamp);

    if unix_timestamp > max_long {
        return max_long;
    } else if unix_timestamp < min_long {
        return min_long;
    } else {
        return unix_timestamp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_offset() {
        assert_eq!(get_sas_epoch_offset(), -((10 * 365 + 2) * 24 * 60 * 60)); // Add 2 days for leap
                                                                              // years
    }

    #[test]
    fn test_sas_to_unix() {
        let sas_timestamp = 0;
        assert_eq!(sas_to_unix(sas_timestamp), ((10 * 365 + 2) * 24 * 60 * 60));
    }

    #[test]
    fn test_sas_to_unix_max() {
        let sas_timestamp = get_max_long();
        assert_eq!(sas_to_unix(sas_timestamp), get_max_long());
    }

    #[test]
    fn test_sas_to_unix_min() {
        let sas_timestamp = get_min_long();
        let timestamp_from_min_long =
            get_min_long() + Duration::days(10 * 365 + 2).num_seconds() as i128;
        println!(
            "n days between sas and getminlong: {}",
            Duration::seconds(
                (sas_to_unix(sas_timestamp) as i128 - timestamp_from_min_long) as i64
            )
            .num_days()
        );
        assert_eq!(sas_to_unix(sas_timestamp), timestamp_from_min_long);
    }
}
