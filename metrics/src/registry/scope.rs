use crate::common::{MetricScope, MetricScopeHandle};
use parking_lot::RwLock;
use std::collections::HashMap;

struct Inner {
    id: u64,
    forward: HashMap<MetricScope, MetricScopeHandle>,
    backward: HashMap<MetricScopeHandle, MetricScope>,
}

impl Inner {
    pub fn new() -> Self {
        Inner {
            id: 1,
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
}

pub(crate) struct ScopeRegistry {
    inner: RwLock<Inner>,
}

impl ScopeRegistry {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn register(&self, scope: MetricScope) -> u64 {
        let mut wg = self.inner.write();

        // If the key is already registered, send back the existing scope ID.
        if wg.forward.contains_key(&scope) {
            return wg.forward.get(&scope).cloned().unwrap();
        }

        // Otherwise, take the current scope ID for this registration, store it, and increment
        // the scope ID counter for the next registration.
        let scope_id = wg.id;
        let _ = wg.forward.insert(scope.clone(), scope_id);
        let _ = wg.backward.insert(scope_id, scope);
        wg.id += 1;
        scope_id
    }

    pub fn get(&self, scope_id: MetricScopeHandle) -> MetricScope {
        // See if we have an entry for the scope ID, and clone the scope if so.
        let rg = self.inner.read();
        rg.backward
            .get(&scope_id)
            .cloned()
            .unwrap_or(MetricScope::Root)
    }
}