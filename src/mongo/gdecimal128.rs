use mongodb::bson::Decimal128;
use async_graphql::{
    Value,
    Scalar,
    ScalarType,
    InputValueError,
    InputValueResult,
};


pub struct GDecimal128(pub Decimal128);


#[Scalar]
impl ScalarType for GDecimal128 {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            // Parse the Decimal128 value
            let decimal = value.parse::<Decimal128>().map_err(InputValueError::custom)?;
            Ok(GDecimal128(decimal))
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl Serialize for GDecimal128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GDecimal128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let decimal = Decimal128::deserialize(deserializer)?;
        Ok(GDecimal128(decimal))
    }
}

impl Clone for GDecimal128 {
    fn clone(&self) -> Self {
        GDecimal128(self.0.clone())
    }
}
