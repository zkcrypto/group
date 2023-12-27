use core::fmt;
use core::ops::{Mul, Neg};
use ff::PrimeField;

use crate::{Curve, Group, GroupEncoding, Identity};

/// This trait represents an element of a prime-order cryptographic group.
pub trait PrimeGroup: Group + GroupEncoding {}

/// Efficient representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurve: Curve<AffineRepr = <Self as PrimeCurve>::Affine> + PrimeGroup {
    type Affine: PrimeCurveAffine<Curve = Self, Scalar = Self::Scalar>
        + Mul<Self::Scalar, Output = Self>
        + for<'r> Mul<&'r Self::Scalar, Output = Self>;
}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurveAffine: GroupEncoding
    + Identity
    + Copy
    + Clone
    + Sized
    + Send
    + Sync
    + fmt::Debug
    + PartialEq
    + Eq
    + 'static
    + Neg<Output = Self>
    + Mul<<Self as PrimeCurveAffine>::Scalar, Output = <Self as PrimeCurveAffine>::Curve>
    + for<'r> Mul<&'r <Self as PrimeCurveAffine>::Scalar, Output = <Self as PrimeCurveAffine>::Curve>
{
    type Scalar: PrimeField;
    type Curve: PrimeCurve<Affine = Self, Scalar = Self::Scalar>;

    /// Returns a fixed generator of unknown exponent.
    fn generator() -> Self;

    /// Converts this element to its curve representation.
    fn to_curve(&self) -> Self::Curve;
}
