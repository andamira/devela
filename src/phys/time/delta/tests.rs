// devela::phys::time::delta::tests
//

#[cfg(feature = "std")]
mod std {
    use crate::{Duration, SystemInstant, TimeDelta};

    #[test]
    fn test_after() {
        let now = SystemInstant::now();
        let delta = TimeDelta::new(10, 500_000_000); // 10.5 seconds

        let future = delta.after(now);
        assert_eq!(future, now + Duration::new(10, 500_000_000));

        let past_delta = TimeDelta::new(-10, -500_000_000); // -10.5 seconds
        let past = past_delta.after(now);
        assert_eq!(past, now - Duration::new(10, 500_000_000));
    }

    #[test]
    fn test_add_time_delta_to_system_instant() {
        let now = SystemInstant::now();
        let delta = TimeDelta::new(5, 250_000_000); // 5.25 seconds

        let result = now + delta;
        assert_eq!(result, now + Duration::new(5, 250_000_000));

        let negative_delta = TimeDelta::new(-5, -250_000_000); // -5.25 seconds
        let result = now + negative_delta;
        assert_eq!(result, now - Duration::new(5, 250_000_000));
    }

    #[test]
    fn test_sub_time_delta_from_system_instant() {
        let now = SystemInstant::now();
        let delta = TimeDelta::new(3, 750_000_000); // 3.75 seconds

        let result = now - delta;
        assert_eq!(result, now - Duration::new(3, 750_000_000));

        let negative_delta = TimeDelta::new(-3, -750_000_000); // -3.75 seconds
        let result = now - negative_delta;
        assert_eq!(result, now + Duration::new(3, 750_000_000));
    }

    #[test]
    fn test_sub_system_instant_from_time_delta() {
        let now = SystemInstant::now();
        let delta = TimeDelta::new(7, 500_000_000); // 7.5 seconds

        let result = delta - now;
        assert_eq!(result, now - Duration::new(7, 500_000_000));

        let negative_delta = TimeDelta::new(-7, -500_000_000); // -7.5 seconds
        let result = negative_delta - now;
        assert_eq!(result, now + Duration::new(7, 500_000_000));
    }

    #[test]
    fn test_edge_cases() {
        let now = SystemInstant::now();
        let zero_delta = TimeDelta::new(0, 0);

        // Adding/subtracting zero should not change the instant
        assert_eq!(zero_delta.after(now), now);
        assert_eq!(now + zero_delta, now);
        assert_eq!(now - zero_delta, now);
        assert_eq!(zero_delta - now, now);

        // Maximum and minimum values
        let max_delta = TimeDelta::new(i64::MAX, 999_999_999);
        let min_delta = TimeDelta::new(i64::MIN, -999_999_999);

        assert![max_delta.checked_after(now).is_none()];
        assert![min_delta.checked_after(now).is_none()];
    }
}
