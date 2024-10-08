use super::Scalar;
use crate::datatypes::ArrowDataType;

/// A single entry of a [`crate::array::StructArray`].
#[derive(Debug, Clone)]
pub struct StructScalar {
    values: Vec<Box<dyn Scalar>>,
    is_valid: bool,
    dtype: ArrowDataType,
}

impl PartialEq for StructScalar {
    fn eq(&self, other: &Self) -> bool {
        (self.dtype == other.dtype)
            && (self.is_valid == other.is_valid)
            && ((!self.is_valid) | (self.values == other.values))
    }
}

impl StructScalar {
    /// Returns a new [`StructScalar`]
    #[inline]
    pub fn new(dtype: ArrowDataType, values: Option<Vec<Box<dyn Scalar>>>) -> Self {
        let is_valid = values.is_some();
        Self {
            values: values.unwrap_or_default(),
            is_valid,
            dtype,
        }
    }

    /// Returns the values irrespectively of the validity.
    #[inline]
    pub fn values(&self) -> &[Box<dyn Scalar>] {
        &self.values
    }
}

impl Scalar for StructScalar {
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    #[inline]
    fn dtype(&self) -> &ArrowDataType {
        &self.dtype
    }
}
