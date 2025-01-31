//! Code for allowing workers to help with crafting.

use crate::organisms::energy::VigorModifier;
use bevy::{prelude::*, utils::HashMap};

use std::fmt::Display;

/// The number of workers present / allowed at this structure.
#[derive(Component, Debug, Clone, PartialEq)]
pub(crate) struct WorkersPresent {
    /// The list of workers present
    workers: HashMap<Entity, VigorModifier>,

    /// The maximum number of workers allowed
    allowed: u8,
}

impl WorkersPresent {
    /// Create a new [`WorkersPresent`] with the provided maximum number of workers allowed.
    pub(crate) fn new(allowed: u8) -> Self {
        Self {
            workers: HashMap::new(),
            allowed,
        }
    }

    /// Are more workers needed?
    pub(crate) fn needs_more(&self) -> bool {
        self.current() < self.allowed
    }

    /// The current number of workers present.
    pub(crate) fn current(&self) -> u8 {
        self.workers.len() as u8
    }

    /// The current number of effective workers, when taking into account the [`VigorModifier`].
    pub(crate) fn effective_workers(&self) -> f32 {
        self.workers
            .values()
            .map(|vigor_modifier| vigor_modifier.ratio())
            .sum()
    }

    /// Adds a worker to this structure if there is room.
    pub(crate) fn add_worker(
        &mut self,
        worker_entity: Entity,
        vigor_modifier: VigorModifier,
    ) -> Result<(), ()> {
        if self.needs_more() {
            self.workers.insert(worker_entity, vigor_modifier);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Removes a worker from this structure
    pub(crate) fn remove_worker(&mut self, worker_entity: Entity) {
        self.workers.remove(&worker_entity);
    }
}

impl Display for WorkersPresent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{present} ({effective_workers}) / {allowed}",
            present = self.current(),
            effective_workers = self.effective_workers(),
            allowed = self.allowed
        )
    }
}
