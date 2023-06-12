use mongodb::bson::Decimal128 as Decimal128Mongo;
use serde::{Serialize, Deserialize};
use async_graphql::{
    Value,
    Scalar,
    ScalarType,
    InputValueError,
    InputValueResult,
};


pub struct Decimal128(pub Decimal128Mongo);


#[Scalar]
impl ScalarType for Decimal128 {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            // Parse the Decimal128Mongo value
            let decimal = value.parse::<Decimal128Mongo>().map_err(InputValueError::custom)?;
            Ok(Decimal128(decimal))
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl Serialize for Decimal128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Decimal128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let decimal = Decimal128Mongo::deserialize(deserializer)?;
        Ok(Decimal128(decimal))
    }
}

impl Clone for Decimal128 {
    fn clone(&self) -> Self {
        Decimal128(self.0.clone())
    }
}
