mod client;

#[cfg(feature = "mlb")]
pub use client::mlb::Client as MlbClient;

#[cfg(feature = "nhl")]
pub use client::nhl::Client as NhlClient;

pub mod model;

#[cfg(test)]
mod tests;
