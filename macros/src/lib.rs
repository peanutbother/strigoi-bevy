#[allow(unused)]
pub use proc_macros::*;

/// Boilerplate to create a bevy plugin
#[macro_export]
macro_rules! plugin {
    ($(#[$($attr:tt)*])* $plugin_name:ident, $app:ident => {
        $($fn_body:tt)*
    }) => {
        $(#[$($attr)*])*
        pub struct $plugin_name;
        impl Plugin for $plugin_name {
            fn build(&self, $app: &mut App) {
                $($fn_body)*
            }
        }
    };
}

/// Registers an LDTK entity using the blueprint pattern (post processing the entity in a designated system) with its `bindings::IDENTIFIER`
#[macro_export]
macro_rules! register_entity {
    ($app:ident, $bundle:ty) => {
        $app.register_ldtk_entity::<$bundle>(bindings::IDENTIFIER)
            .add_systems(Update, bindings::process_entity)
    };
}

/// Registers an ldtk int cell bundle using the module's `bindings::INT_GRID_VALUE`
#[macro_export]
macro_rules! register_int_cell {
    ($app:ident, $bundle:ty) => {
        $app.register_ldtk_int_cell::<$bundle>(bindings::INT_GRID_VALUE)
    };
}

/// Register one or more types with bevy
#[macro_export]
macro_rules! register_types {
    ($app:ident, $($register_type:ty),*) => {
        $app
            $(.register_type::<$register_type>())*
    };
}

/// This macro inverts a mutable boolean variable in place
#[macro_export]
macro_rules! invert {
    ($var:ident) => {
        $var = !$var
    };
    ($var:expr) => {
        $var = !$var
    };
}
