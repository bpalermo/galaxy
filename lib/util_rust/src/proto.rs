use galaxy_api::r#type::Money;
use num_traits::pow::Pow;
use prost_types::Timestamp;
use rust_decimal::prelude::*;
use std::ops::Mul;
use time::PrimitiveDateTime;

pub fn decimal_to_google_money(currency: String, balance: Decimal) -> Option<Money> {
    let units: Option<i64> = balance.trunc().to_i64();
    if units.is_none() {
        return None;
    }

    let nanos: Option<i32> = balance
        .fract()
        .mul(Decimal::TEN.pow(balance.scale() as i64))
        .trunc()
        .to_i32();
    if nanos.is_none() {
        return None;
    }

    Some(Money {
        currency_code: currency,
        units: units.unwrap(),
        nanos: nanos.unwrap(),
    })
}

pub fn primitive_date_time_to_google_timestamp(
    date_time: Option<PrimitiveDateTime>,
) -> Option<Timestamp> {
    if date_time.is_none() {
        return None;
    }
    return Some(Timestamp {
        seconds: date_time?.assume_utc().unix_timestamp(),
        nanos: date_time?.nanosecond() as i32,
    });
}

#[cfg(test)]
mod tests {

    use super::*;
    use galaxy_api::r#type::Money;
    use rust_decimal::Decimal;

    #[test]
    fn test_balance_to_money_zero() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: 0,
            nanos: 0,
        };

        let m: Option<Money> = decimal_to_google_money("BRL".to_string(), Decimal::ZERO);
        assert_ne!(None, m);

        assert_eq!(expected, m.unwrap());
    }

    #[test]
    fn test_balance_to_money_one() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: 1,
            nanos: 0,
        };

        let m: Option<Money> = decimal_to_google_money("BRL".to_string(), Decimal::ONE);
        assert_ne!(None, m);

        assert_eq!(expected, m.unwrap());
    }

    #[test]
    fn test_balance_to_money_zero_unit_negative_nanos() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: 0,
            nanos: -1,
        };

        let m: Option<Money> =
            decimal_to_google_money("BRL".to_string(), Decimal::from_str("-0.1").unwrap());
        assert_ne!(None, m);
        assert_eq!(expected, m.unwrap());
    }

    #[test]
    fn test_balance_to_money_test_balance_to_money_zero_unit_positive_nanos() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: 0,
            nanos: 1,
        };

        let m: Option<Money> =
            decimal_to_google_money("BRL".to_string(), Decimal::from_str("0.1").unwrap());
        assert_ne!(None, m);
        assert_eq!(expected, m.unwrap());
    }

    #[test]
    fn test_balance_to_money_min() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: -1,
            nanos: -999_999_999,
        };

        let m: Option<Money> = decimal_to_google_money(
            "BRL".to_string(),
            Decimal::from_str("-1.999999999").unwrap(),
        );
        assert_ne!(None, m);
        assert_eq!(expected, m.unwrap());
    }

    #[test]
    fn test_balance_to_money_max() {
        let expected = Money {
            currency_code: "BRL".to_string(),
            units: 10,
            nanos: 999_999_999,
        };

        let m: Option<Money> = decimal_to_google_money(
            "BRL".to_string(),
            Decimal::from_str("10.999999999").unwrap(),
        );
        assert_ne!(None, m);
        assert_eq!(expected, m.unwrap());
    }
}
