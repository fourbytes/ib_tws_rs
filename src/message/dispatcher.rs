//use super::identifiable::Identifiable;
use futures::sync::{mpsc, oneshot};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct Dispatcher<I: Hash + Eq, R, E> {
    oneshot_reply: HashMap<I, oneshot::Sender<Result<R, E>>>,
    stream_reply: HashMap<I, mpsc::UnboundedSender<Result<R, E>>>,
}

impl<I: Hash + Eq, R, E> Dispatcher<I, R, E> {
    pub(crate) fn new() -> Self {
        Dispatcher {
            oneshot_reply: HashMap::new(),
            stream_reply: HashMap::new(),
        }
    }

    #[inline(always)]
    pub(crate) fn register_oneshot(
        &mut self,
        id: I,
        tx: oneshot::Sender<Result<R, E>>,
    ) -> Option<oneshot::Sender<Result<R, E>>> {
        self.oneshot_reply.insert(id, tx)
    }

    #[inline(always)]
    pub fn register_stream(
        &mut self,
        id: I,
        tx: mpsc::UnboundedSender<Result<R, E>>,
    ) -> Option<mpsc::UnboundedSender<Result<R, E>>> {
        self.stream_reply.insert(id, tx)
    }

    #[inline(always)]
    pub(crate) fn container_in_oneshot(&self, id: &I) -> bool {
        self.oneshot_reply.contains_key(id)
    }

    #[inline(always)]
    pub(crate) fn container_in_stream(&self, id: &I) -> bool {
        self.stream_reply.contains_key(id)
    }

    #[inline(always)]
    pub(crate) fn unregister_oneshot(&mut self, id: &I) -> Option<oneshot::Sender<Result<R, E>>> {
        self.oneshot_reply.remove(id)
    }

    #[inline(always)]
    pub(crate) fn unregister_stream(
        &mut self,
        id: &I,
    ) -> Option<mpsc::UnboundedSender<Result<R, E>>> {
        self.stream_reply.remove(id)
    }

    pub(crate) fn reply_to_stream(
        &self,
        id: &I,
        response: Result<R, E>,
    ) -> Result<(), Result<R, E>> {
        let ret = self.stream_reply.get(id);
        if let Some(tx) = ret {
            match tx.unbounded_send(response) {
                Err(v) => return Err(v.into_inner()),
                _ => return Ok(()),
            }
        }
        Err(response)
    }

    pub(crate) fn reply_to_oneshot(
        &mut self,
        id: &I,
        response: Result<R, E>,
    ) -> Result<(), Result<R, E>> {
        let ret = self.oneshot_reply.remove(&id);
        if let Some(tx) = ret {
            match tx.send(response) {
                Err(v) => return Err(v),
                _ => return Ok(()),
            }
        }
        Err(response)
    }

    pub(crate) fn reply_to(&mut self, id: &I, response: Result<R, E>) -> Result<(), Result<R, E>> {
        if self.container_in_stream(id) {
            return self.reply_to_stream(id, response);
        }

        if self.container_in_oneshot(id) {
            return self.reply_to_oneshot(id, response);
        }

        Err(response)
    }
}
