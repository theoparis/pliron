//! Infrastructure for casting from `dyn Any` to `dyn Trait`,
//! for traits that the type contained by the [Any] object implements.
//!
//! A user must specify [type_to_trait](crate::type_to_trait) for a type that implements
//! a trait and needs to be casted to it, and then use [any_to_trait]
//! to do the actual cast. See their documentation for details and examples.

use std::{
    any::{Any, TypeId},
    sync::LazyLock,
};

use downcast_rs::Downcast;
use dyn_clone::DynClone;
use linkme::distributed_slice;
use rustc_hash::FxHashMap;

/// Cast a [dyn Any](Any) object to a `dyn Trait` object for any
/// trait that the contained (in [Any]) type implements, and for which
/// [type_to_trait](crate::type_to_trait) has been specified.
///
/// To cast from `dyn Trait1` to `dyn Trait2` (when the underlying type implements both),
/// the user may use [downcast_rs] to easily upcast from `dyn Trait1` to [Any],
/// and then use [any_to_trait] to cast to `dyn Trait2`.
/// Example:
/// ```
/// # use pliron::{type_to_trait, utils::trait_cast::any_to_trait};
/// # use std::any::Any;
/// # use downcast_rs::Downcast;
///
/// trait Trait1: Downcast {}
/// trait Trait2 {}
///
/// struct S;
/// impl Trait1 for S {}
/// impl Trait2 for S {}
///
/// type_to_trait!(S, Trait2);
///
/// let s1: &dyn Trait1 = &S;
/// any_to_trait::<dyn Trait2>(s1.as_any()).expect("Expected S to implement Trait2");
///
/// ```
pub fn any_to_trait<T: ?Sized + 'static>(r: &dyn Any) -> Option<&T> {
    TRAIT_CASTERS_MAP
        .get(&(r.type_id(), TypeId::of::<T>()))
        .and_then(|caster| {
            if let Some(caster) = (**caster)
                .as_any()
                .downcast_ref::<for<'a> fn(&'a (dyn Any + 'static)) -> Option<&'a T>>()
            {
                return caster(r);
            }
            None
        })
}

pub trait ClonableAny: Any + DynClone + Downcast {}
dyn_clone::clone_trait_object!(ClonableAny);
impl<T: Any + DynClone + Downcast> ClonableAny for T {}

#[doc(hidden)]
#[distributed_slice]
/// A distributed slice of (type_id of the object, type_id of the trait to cast to, cast function)
pub static TRAIT_CASTERS: [LazyLock<((TypeId, TypeId), Box<dyn ClonableAny + Sync + Send>)>];

#[doc(hidden)]
/// A map of all the trait casters, indexed by the type_id of the object
/// and the type_id of the trait to cast to. The map's values are
/// the cast function pointers. This is used to avoid having to search
/// through the distributed slice every time we want to cast an object.
static TRAIT_CASTERS_MAP: LazyLock<
    FxHashMap<(TypeId, TypeId), Box<dyn ClonableAny + Sync + Send>>,
> = LazyLock::new(|| {
    TRAIT_CASTERS
        .iter()
        .map(|lazy_tuple| (**lazy_tuple).clone())
        .collect()
});

/// Specify that a type may be casted to a `dyn Trait` object. Use [any_to_trait] for the actual cast.
/// Example:
/// ```
/// # use pliron::{type_to_trait, utils::trait_cast::any_to_trait};
/// # use std::any::Any;
/// trait Trait {}
/// struct S1;
/// impl Trait for S1 {}
/// type_to_trait!(S1, Trait);
///
/// let s1: &dyn Any = &S1;
/// any_to_trait::<dyn Trait>(s1).expect("Expected S1 to implement Trait");
///
/// struct S2;
/// let s2: &dyn Any = &S2;
/// assert!(
///     any_to_trait::<dyn Trait>(s2).is_none(),
///     "S2 does not implement Trait"
/// );
/// ```
#[macro_export]
macro_rules! type_to_trait {
    ($ty_name:ty, $to_trait_name:path) => {
        // The rust way to do an anonymous module.
        const _: () = {
            #[linkme::distributed_slice($crate::utils::trait_cast::TRAIT_CASTERS)]
            static CAST_TO_TRAIT: std::sync::LazyLock<(
                (std::any::TypeId, std::any::TypeId),
                Box<dyn $crate::utils::trait_cast::ClonableAny + Sync + Send>,
            )> = std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<$ty_name>(),
                        std::any::TypeId::of::<dyn $to_trait_name>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn $to_trait_name + 'static)>,
                    ),
                )
            });
            fn cast_to_trait<'a>(
                r: &'a (dyn std::any::Any + 'static),
            ) -> Option<&'a (dyn $to_trait_name + 'static)> {
                r.downcast_ref::<$ty_name>()
                    .map(|s| s as &dyn $to_trait_name)
            }
        };
    };
}
