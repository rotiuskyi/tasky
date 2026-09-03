pub mod area;
pub mod menu;
pub mod select;

/// The corner radius of the controls: buttons, fields, menus.
///
/// Surfaces are rounded more than that; see [`area::card`].
pub const CONTROL_RADIUS: u32 = 2;
