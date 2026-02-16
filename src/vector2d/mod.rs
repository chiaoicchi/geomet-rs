/// A 2-dimensional vector over `T`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2D<T>(T, T);

/// Shorthand for constructing a `Vector2D`.
///
/// - `v2!(x, y)` creates `Vector2D::new(x.into(), y.into())`.
/// - `v2!(x, y; T)` creates `Vector2D::<T>::new(x.into(), y.into())`.
#[macro_export]
macro_rules! v2 {
    ($x:expr, $y:expr) => {
        $crate::Vector2D::new($x.into(), $y.into())
    };
    ($x:expr, $y:expr; $t:ty) => {
        $crate::Vector2D::<$t>::new($x.into(), $y.into())
    };
}

impl<T: Copy> Vector2D<T> {
    /// Creates a new 2-dimensional vector.
    ///
    /// # Time complexity
    ///
    /// O(1)
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Returns the x element of the vector.
    ///
    /// # Time complexity
    ///
    /// O(1)
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0
    }

    /// Returns the y element of the vector.
    ///
    /// # Time complexity
    ///
    /// O(1)
    #[inline(always)]
    pub fn y(&self) -> T {
        self.1
    }
}

impl<T: Copy + Default> Vector2D<T> {
    /// Returns zero vector.
    ///
    /// # Time complexity
    ///
    /// O(1)
    #[inline(always)]
    pub fn zero() -> Self {
        Vector2D(T::default(), T::default())
    }
}

impl<T: Ord + std::ops::Mul<Output = T> + Copy + Default> Vector2D<T> {
    /// Compares `self` and `other` by their argument (polar angle).
    ///
    /// The argument is measured counter-clockwise from the positive x-axis,
    /// ranging over [0, 2π). The positive x-axis has argument 0.
    ///
    /// # Panics
    ///
    /// Panics if either vector is the origin in debug builds.
    pub fn arg_cmp(&self, other: &Self) -> std::cmp::Ordering {
        debug_assert!(*self != Self::zero(), "self must not be zero",);
        debug_assert!(*other != Self::zero(), "other must not be zero",);
        ((self.y(), self.x()) < (T::default(), T::default()))
            .cmp(&((other.y(), other.x()) < (T::default(), T::default())))
            .then_with(|| (other.x() * self.y()).cmp(&(self.x() * other.y())))
    }
}
