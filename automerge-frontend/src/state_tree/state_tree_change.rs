use super::StateTreeComposite;
use automerge_protocol as amp;

/// Represents a change to the state tree. This is used to represent values which have changed
/// (the type T usually ends up being either a MultiValue or a StateTreeComposite) along with
/// changes that need to be made to indexes maintained by the state tree (the object id -> object
/// value index for example).
#[derive(Clone)]
pub struct StateTreeChange<T> {
    value: T,
    index_updates: Option<im::HashMap<amp::ObjectID, StateTreeComposite>>,
}

impl<T> StateTreeChange<T>
where
    T: Clone,
{
    pub(super) fn pure(value: T) -> StateTreeChange<T> {
        StateTreeChange {
            value,
            index_updates: None,
        }
    }

    pub(super) fn value(&self) -> &T {
        &self.value
    }

    pub(super) fn index_updates(&self) -> Option<&im::HashMap<amp::ObjectID, StateTreeComposite>> {
        self.index_updates.as_ref()
    }

    pub(super) fn map<F, G>(self, f: F) -> StateTreeChange<G>
    where
        F: Fn(T) -> G,
    {
        StateTreeChange {
            value: f(self.value),
            index_updates: self.index_updates,
        }
    }

    pub(super) fn and_then<F, G>(self, f: F) -> StateTreeChange<G>
    where
        F: Fn(T) -> StateTreeChange<G>,
    {
        let diff = f(self.value.clone());
        let result = self.with_updates(diff.index_updates.clone());
        StateTreeChange {
            value: diff.value,
            index_updates: result.index_updates,
        }
    }

    pub(super) fn with_updates(
        self,
        updates: Option<im::HashMap<amp::ObjectID, StateTreeComposite>>,
    ) -> StateTreeChange<T> {
        match (updates, self.index_updates) {
            (Some(updates), Some(existing_updates)) => StateTreeChange {
                value: self.value,
                index_updates: Some(updates.union(existing_updates)),
            },
            (Some(updates), None) => StateTreeChange {
                value: self.value,
                index_updates: Some(updates),
            },
            (None, Some(existing_updates)) => StateTreeChange {
                value: self.value,
                index_updates: Some(existing_updates),
            },
            (None, None) => StateTreeChange {
                value: self.value,
                index_updates: None,
            },
        }
    }
}
