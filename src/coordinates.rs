//! Extension traits and structs that provide generic access to the coordinates of
//! elliptic curve points.
//!
//! Coordinates are meaningless without the context of the curve equation that constrains
//! them. To safely expose them in a generic context, we use extension traits to restrict
//! the scope of the generic curve parameter; this ensures that the code can only be used
//! with curve implementations that explicitly expose their use of a specific curve model.

use subtle::{Choice, ConditionallySelectable, CtOption};

use crate::CurveAffine;

//
// Twisted Edwards curve
//

/// An affine elliptic curve point on a twisted Edwards curve
/// $a \cdot x^2 + y^2 = 1 + d \cdot x^2 \cdot y^2$.
pub trait TwistedEdwardsPoint: CurveAffine + Default + ConditionallySelectable {
    /// Field element type used in the curve equation.
    type Base: Copy + ConditionallySelectable;

    /// The parameter $a$ in the twisted Edwards curve equation.
    ///
    /// When $a = 1$, this reduces to an ordinary Edwards curve.
    const A: Self::Base;

    /// The parameter $d$ in the twisted Edwards curve equation.
    const D: Self::Base;

    /// Obtains a point given $(x, y)$, failing if it is not on the curve.
    fn from_bare_coordinates(x: Self::Base, y: Self::Base) -> CtOption<Self>;

    /// Obtains a point given its coordinates.
    fn from_coordinates(coords: TwistedEdwardsCoordinates<Self>) -> Self;

    /// Returns the coordinates of this point.
    ///
    /// For twisted Edwards curves, the identity has valid coordinates on the curve, so
    /// this method is infallible.
    fn coordinates(&self) -> TwistedEdwardsCoordinates<Self>;
}

/// The affine coordinates for a [`TwistedEdwardsPoint`].
#[derive(Clone, Copy, Debug, Default)]
pub struct TwistedEdwardsCoordinates<P: TwistedEdwardsPoint> {
    x: P::Base,
    y: P::Base,
}

impl<P: TwistedEdwardsPoint> TwistedEdwardsCoordinates<P> {
    /// Obtains a `TwistedEdwardsCoordinates` value given $(x, y)$, failing if it is not
    /// on the curve.
    pub fn from_coordinates(x: P::Base, y: P::Base) -> CtOption<Self> {
        // We use `P::from_bare_coordinates` to validate the coordinates.
        P::from_bare_coordinates(x, y).map(|_| TwistedEdwardsCoordinates { x, y })
    }

    /// Returns the x-coordinate.
    pub fn x(&self) -> P::Base {
        self.x
    }

    /// Returns the y-coordinate.
    pub fn y(&self) -> P::Base {
        self.y
    }
}

impl<P: TwistedEdwardsPoint> ConditionallySelectable for TwistedEdwardsCoordinates<P> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        TwistedEdwardsCoordinates {
            x: P::Base::conditional_select(&a.x, &b.x, choice),
            y: P::Base::conditional_select(&a.y, &b.y, choice),
        }
    }
}

//
// Montgomery curve
//

/// An affine elliptic curve point on a Montgomery curve
/// $B \cdot v^2 = u^3 + A \cdot u^2 + u$.
///
/// For these curves, it is required that $B \cdot (A^2 - 4) ≠ 0$, which implies that
/// $A ≠ ±2$ and $B ≠ 0$.
pub trait MontgomeryPoint: CurveAffine + Default + ConditionallySelectable {
    /// Field element type used in the curve equation.
    type Base: Copy + ConditionallySelectable;

    /// The parameter $A$ in the Montgomery curve equation.
    const A: Self::Base;

    /// The parameter $B$ in the Montgomery curve equation.
    const B: Self::Base;

    /// Obtains a point given $(u, v)$, failing if it is not on the curve.
    fn from_bare_coordinates(u: Self::Base, v: Self::Base) -> CtOption<Self>;

    /// Obtains a point given its coordinates.
    fn from_coordinates(coords: MontgomeryCoordinates<Self>) -> Self;

    /// Returns the coordinates of this point.
    ///
    /// Returns `None` if this is the identity.
    fn coordinates(&self) -> CtOption<MontgomeryCoordinates<Self>>;
}

/// The affine coordinates for a [`MontgomeryCoordinates`].
#[derive(Clone, Copy, Debug, Default)]
pub struct MontgomeryCoordinates<P: MontgomeryPoint> {
    u: P::Base,
    v: P::Base,
}

impl<P: MontgomeryPoint> MontgomeryCoordinates<P> {
    /// Obtains a `MontgomeryCoordinates` value given $(u, v)$, failing if it is not on
    /// the curve.
    pub fn from_coordinates(u: P::Base, v: P::Base) -> CtOption<Self> {
        // We use `P::from_bare_coordinates` to validate the coordinates.
        P::from_bare_coordinates(u, v).map(|_| MontgomeryCoordinates { u, v })
    }

    /// Returns the u-coordinate.
    pub fn u(&self) -> P::Base {
        self.u
    }

    /// Returns the v-coordinate.
    pub fn v(&self) -> P::Base {
        self.v
    }
}

impl<P: MontgomeryPoint> ConditionallySelectable for MontgomeryCoordinates<P> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        MontgomeryCoordinates {
            u: P::Base::conditional_select(&a.u, &b.u, choice),
            v: P::Base::conditional_select(&a.v, &b.v, choice),
        }
    }
}

//
// Short Weierstrass curve
//

/// An affine elliptic curve point on a short Weierstrass curve
/// $y^2 = x^3 + a \cdot x + b$.
pub trait ShortWeierstrassPoint: CurveAffine + Default + ConditionallySelectable {
    /// Field element type used in the curve equation.
    type Base: Copy + ConditionallySelectable;

    /// The parameter $a$ in the short Weierstrass curve equation.
    const A: Self::Base;

    /// The parameter $b$ in the short Weierstrass curve equation.
    const B: Self::Base;

    /// Obtains a point given $(x, y)$, failing if it is not on the curve.
    fn from_bare_coordinates(x: Self::Base, y: Self::Base) -> CtOption<Self>;

    /// Obtains a point given its coordinates.
    fn from_coordinates(coords: ShortWeierstrassCoordinates<Self>) -> Self;

    /// Returns the coordinates of this point.
    ///
    /// Returns `None` if this is the identity.
    fn coordinates(&self) -> CtOption<ShortWeierstrassCoordinates<Self>>;
}

/// The affine coordinates for a [`ShortWeierstrassCoordinates`].
#[derive(Clone, Copy, Debug, Default)]
pub struct ShortWeierstrassCoordinates<P: ShortWeierstrassPoint> {
    x: P::Base,
    y: P::Base,
}

impl<P: ShortWeierstrassPoint> ShortWeierstrassCoordinates<P> {
    /// Obtains a `ShortWeierstrassCoordinates` value given $(x, y)$, failing if it is not
    /// on the curve.
    pub fn from_coordinates(x: P::Base, y: P::Base) -> CtOption<Self> {
        // We use `P::from_bare_coordinates` to validate the coordinates.
        P::from_bare_coordinates(x, y).map(|_| ShortWeierstrassCoordinates { x, y })
    }

    /// Returns the x-coordinate.
    pub fn x(&self) -> P::Base {
        self.x
    }

    /// Returns the y-coordinate.
    pub fn y(&self) -> P::Base {
        self.y
    }
}

impl<P: ShortWeierstrassPoint> ConditionallySelectable for ShortWeierstrassCoordinates<P> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        ShortWeierstrassCoordinates {
            x: P::Base::conditional_select(&a.x, &b.x, choice),
            y: P::Base::conditional_select(&a.y, &b.y, choice),
        }
    }
}
