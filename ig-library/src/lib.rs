#![allow(non_camel_case_types)]

/// The core of igAlchemy. Contains structs, enums, and functions critical to the engine and is a dependency for all other parts of the engine.
pub mod core;

/// At the moment stores the [IG_GFX_PLATFORM](gfx::ig_gfx_platform::IG_GFX_PLATFORM) Enum critical to loading the [igRegistry](core::ig_registry::igRegistry) inside the core module.
pub mod gfx;
/// Intended for code/the user of alchemy to add their code under the client (c prefix) for cleanliness. In the end only used by Alchemy Laboratory so this is alchemy laboratory specific code mostly.
pub mod client;
/// Tfb specific code used in their version of the engine
pub mod tfb_game;

pub mod util;

#[cfg(test)]
mod tests;