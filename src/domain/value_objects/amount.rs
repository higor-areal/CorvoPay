use serde::{Serialize, Deserializer, Deserialize};
use crate::domain::error::ValidationError;

#[derive(Serialize)]
#[serde(transparent)]
pub struct Amount(u64);

#[allow(dead_code)]
impl Amount {
    pub const MIN: u64 = 1;
    pub const MAX: u64 = 100_000_000;

    pub fn new(value: u64) -> Result<Self, ValidationError> {
        if value < Self::MIN {
            return Err(ValidationError::InvalidAmount);
        }

        if value > Self::MAX {
            return Err(ValidationError::InvalidAmount);
        }

        Ok(Self(value))
    }

    pub fn cents(&self) -> u64 {
        self.0
    }

    pub fn currency(&self) -> String {
        let reais = self.0 / 100;
        let centavos = self.0 % 100;

        format!("{}.{:02}", reais, centavos)
    }
}

impl<'de> Deserialize<'de> for Amount{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error> 
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;

        Amount::new(value)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod domain {
        use super::*;

        #[test]
        fn should_create_amount_with_minimum_valid_value() {
            let result = Amount::new(Amount::MIN);
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_amount_with_maximum_valid_value() {
            let result = Amount::new(Amount::MAX);
            assert!(result.is_ok());
        }

        #[test]
        fn should_create_amount_with_value_between_min_and_max() {
            let result = Amount::new(5_000);
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_when_value_is_zero() {
            let result = Amount::new(0);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_value_exceeds_maximum() {
            let result = Amount::new(Amount::MAX + 1);
            assert!(result.is_err());
        }

        #[test]
        fn should_return_correct_cents() {
            let amount = Amount::new(1050).unwrap();
            assert_eq!(amount.cents(), 1050);
        }

        #[test]
        fn should_format_currency_with_reais_and_centavos() {
            let amount = Amount::new(1050).unwrap();
            assert_eq!(amount.currency(), "10.50");
        }

        #[test]
        fn should_format_currency_when_centavos_is_zero() {
            let amount = Amount::new(1000).unwrap();
            assert_eq!(amount.currency(), "10.00");
        }

        #[test]
        fn should_format_currency_when_value_is_minimum() {
            let amount = Amount::new(Amount::MIN).unwrap();
            assert_eq!(amount.currency(), "0.01");
        }

        #[test]
        fn should_format_currency_when_value_is_maximum() {
            let amount = Amount::new(Amount::MAX).unwrap();
            assert_eq!(amount.currency(), "1000000.00");
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn should_serialize_amount_as_transparent_u64() {
            let amount = Amount::new(1050).unwrap();
            let json = serde_json::to_string(&amount).unwrap();
            assert_eq!(json, "1050");
        }

        #[test]
        fn should_serialize_minimum_amount() {
            let amount = Amount::new(Amount::MIN).unwrap();
            let json = serde_json::to_string(&amount).unwrap();
            assert_eq!(json, "1");
        }

        #[test]
        fn should_serialize_maximum_amount() {
            let amount = Amount::new(Amount::MAX).unwrap();
            let json = serde_json::to_string(&amount).unwrap();
            assert_eq!(json, "100000000");
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn should_deserialize_valid_u64_into_amount() {
            let result: Result<Amount, _> = serde_json::from_str("1050");
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_minimum_valid_value() {
            let result: Result<Amount, _> = serde_json::from_str("1");
            assert!(result.is_ok());
        }

        #[test]
        fn should_deserialize_maximum_valid_value() {
            let result: Result<Amount, _> = serde_json::from_str("100000000");
            assert!(result.is_ok());
        }

        #[test]
        fn should_fail_to_deserialize_zero() {
            let result: Result<Amount, _> = serde_json::from_str("0");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_value_above_maximum() {
            let result: Result<Amount, _> = serde_json::from_str("100000001");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_negative_number() {
            let result: Result<Amount, _> = serde_json::from_str::<Amount>("-1");
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_to_deserialize_string_instead_of_number() {
            let result: Result<Amount, _> = serde_json::from_str::<Amount>("\"1050\"");
            assert!(result.is_err());
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn should_fail_when_value_is_below_minimum_by_one() {
            let result = Amount::new(Amount::MIN - 1);
            assert!(result.is_err());
        }

        #[test]
        fn should_fail_when_value_exceeds_maximum_by_one() {
            let result = Amount::new(Amount::MAX + 1);
            assert!(result.is_err());
        }

        #[test]
        fn should_preserve_cents_value_after_roundtrip_serialization() {
            let original = Amount::new(9999).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Amount = serde_json::from_str(&json).unwrap();
            assert_eq!(original.cents(), restored.cents());
        }

        #[test]
        fn should_format_single_centavo_correctly() {
            let amount = Amount::new(1).unwrap();
            assert_eq!(amount.currency(), "0.01");
        }

        #[test]
        fn should_format_nine_centavos_with_leading_zero() {
            let amount = Amount::new(9).unwrap();
            assert_eq!(amount.currency(), "0.09");
        }
    }
}