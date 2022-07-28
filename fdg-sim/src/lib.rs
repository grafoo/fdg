#![doc = include_str!("../README.md")]

/// Forces that define how your graph moves.
pub mod force;

mod graph;
mod simulation;

#[cfg(feature = "json")]
/// [`Serialize`](serde::Serialize) and [`Deserialize`](serde::Deserialize) [`ForceGraph`] for json.
pub mod json;

pub use glam;
pub use petgraph;

pub use {
    graph::{ForceGraph, ForceGraphHelper},
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
};
