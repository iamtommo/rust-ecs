use std::any::TypeId;

/// component.
/// trait `Sized` enforces fixed size
pub trait Component : Sized {

}

/// shared component.
/// trait `Sized` enforces fixed size
pub trait SharedComponent : Sized {

}

/// system component.
/// trait `Sized` enforces fixed size
pub trait SystemComponent : Sized {

}