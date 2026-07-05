use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::config::CursorConfig;
use crate::cursor::CursorCache;
use crate::git::RepoPairState;
use crate::scanner::ScannedRepo;

pub struct DetailLoadResult {
    pub generation: u64,
    pub index: usize,
    pub result: Result<RepoPairState, String>,
}

pub struct DetailLoader {
    generation: u64,
    loading_for: Option<usize>,
    tx: Sender<DetailLoadResult>,
    rx: Receiver<DetailLoadResult>,
}

impl DetailLoader {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            generation: 0,
            loading_for: None,
            tx,
            rx,
        }
    }

    pub fn is_loading(&self, selected: usize) -> bool {
        self.loading_for == Some(selected)
    }

    /// Start loading detail for `index` on a background thread.
    pub fn spawn(
        &mut self,
        index: usize,
        entry: ScannedRepo,
        remote_available: bool,
        cursor: CursorConfig,
    ) {
        self.generation = self.generation.wrapping_add(1);
        let gen = self.generation;
        self.loading_for = Some(index);

        let tx = self.tx.clone();
        thread::spawn(move || {
            let mut cache = CursorCache::default();
            let result = crate::git::classify_pair(
                &entry,
                remote_available,
                &cursor,
                true,
                &mut cache,
            )
            .map_err(|e| e.to_string());

            let _ = tx.send(DetailLoadResult {
                generation: gen,
                index,
                result,
            });
        });
    }

    /// Apply completed load if it matches the current generation and selection.
    pub fn poll(
        &mut self,
        selected: usize,
    ) -> Option<(usize, Result<RepoPairState, String>)> {
        let mut latest: Option<DetailLoadResult> = None;

        while let Ok(msg) = self.rx.try_recv() {
            if msg.generation == self.generation {
                latest = Some(msg);
            }
        }

        let msg = latest?;
        if msg.index != selected {
            return None;
        }

        self.loading_for = None;
        Some((msg.index, msg.result))
    }

    pub fn cancel(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.loading_for = None;
        while self.rx.try_recv().is_ok() {}
    }
}

impl Default for DetailLoader {
    fn default() -> Self {
        Self::new()
    }
}
