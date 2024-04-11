use approx::{AbsDiffEq, RelativeEq};
use serde::{Deserialize, Serialize};

use super::coordnum::CoordNum;
use super::line_string::LineString;
use super::no_value::NoValue;
use super::triangle::Triangle;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Hash)]
pub struct Polygon<T: CoordNum = f64, Z: CoordNum = NoValue> {
    exterior: LineString<T, Z>,
    interiors: Vec<LineString<T, Z>>,
}

pub type Polygon2D<T> = Polygon<T>;
pub type Polygon3D<T> = Polygon<T, T>;

impl<T: CoordNum, Z: CoordNum> Polygon<T, Z> {
    pub fn new(mut exterior: LineString<T, Z>, mut interiors: Vec<LineString<T, Z>>) -> Self {
        exterior.close();
        for interior in &mut interiors {
            interior.close();
        }
        Self {
            exterior,
            interiors,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn into_inner(self) -> (LineString<T, Z>, Vec<LineString<T, Z>>) {
        (self.exterior, self.interiors)
    }

    pub fn exterior(&self) -> &LineString<T, Z> {
        &self.exterior
    }

    pub fn exterior_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut LineString<T, Z>),
    {
        f(&mut self.exterior);
        self.exterior.close();
    }

    pub fn interiors(&self) -> &[LineString<T, Z>] {
        &self.interiors
    }

    pub fn interiors_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [LineString<T, Z>]),
    {
        f(&mut self.interiors);
        for interior in &mut self.interiors {
            interior.close();
        }
    }

    pub fn interiors_push(&mut self, new_interior: impl Into<LineString<T, Z>>) {
        let mut new_interior = new_interior.into();
        new_interior.close();
        self.interiors.push(new_interior);
    }
}

// impl<T: CoordNum> From<Rect<T>> for Polygon<T> {
//     fn from(r: Rect<T>) -> Self {
//         Polygon::new(
//             vec![
//                 (r.min().x, r.min().y),
//                 (r.max().x, r.min().y),
//                 (r.max().x, r.max().y),
//                 (r.min().x, r.max().y),
//                 (r.min().x, r.min().y),
//             ]
//             .into(),
//             Vec::new(),
//         )
//     }
// }

impl<T: CoordNum, Z: CoordNum> From<Triangle<T, Z>> for Polygon<T, Z> {
    fn from(t: Triangle<T, Z>) -> Self {
        Self::new(vec![t.0, t.1, t.2, t.0].into(), Vec::new())
    }
}

impl<T> RelativeEq for Polygon<T, T>
where
    T: AbsDiffEq<Epsilon = T> + CoordNum + RelativeEq,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        if !self
            .exterior
            .relative_eq(&other.exterior, epsilon, max_relative)
        {
            return false;
        }

        if self.interiors.len() != other.interiors.len() {
            return false;
        }
        let mut zipper = self.interiors.iter().zip(other.interiors.iter());
        zipper.all(|(lhs, rhs)| lhs.relative_eq(rhs, epsilon, max_relative))
    }
}

impl<T: AbsDiffEq<Epsilon = T> + CoordNum> AbsDiffEq for Polygon<T, T> {
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if !self.exterior.abs_diff_eq(&other.exterior, epsilon) {
            return false;
        }

        if self.interiors.len() != other.interiors.len() {
            return false;
        }
        let mut zipper = self.interiors.iter().zip(other.interiors.iter());
        zipper.all(|(lhs, rhs)| lhs.abs_diff_eq(rhs, epsilon))
    }
}
