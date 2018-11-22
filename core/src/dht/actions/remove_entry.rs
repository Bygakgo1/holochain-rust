extern crate futures;
extern crate serde_json;
use action::{Action, ActionWrapper};
use context::Context;
use futures::Future;
use holochain_core_types::{cas::content::Address, error::HolochainError};
use instance::dispatch_action;
use std::sync::{mpsc::SyncSender, Arc};

/// Remove Entry Action Creator
///
/// Returns a future that resolves to an Ok(ActionWrapper) or an Err(HolochainError).
pub fn remove_entry(
    context: &Arc<Context>,
    action_channel: &SyncSender<ActionWrapper>,
    address: Address,
) -> RemoveEntryFuture {
    let action_wrapper = ActionWrapper::new(Action::RemoveEntry(address));
    dispatch_action(action_channel, action_wrapper.clone());
    RemoveEntryFuture {
        context: context.clone(),
        action: action_wrapper,
    }
}

/// RemoveEntryFuture resolves to ActionResponse
/// Tracks the state for a response to its ActionWrapper
pub struct RemoveEntryFuture {
    context: Arc<Context>,
    action: ActionWrapper,
}

impl Future for RemoveEntryFuture {
    type Item = ();
    type Error = HolochainError;

    fn poll(
        &mut self,
        cx: &mut futures::task::Context<'_>,
    ) -> Result<futures::Async<Self::Item>, Self::Error> {
        //
        // TODO: connect the waker to state updates for performance reasons
        // See: https://github.com/holochain/holochain-rust/issues/314
        //
        cx.waker().wake();
        if let Some(state) = self.context.state() {
            match state.dht().actions().get(&self.action) {
                Some(Ok(_)) => Ok(futures::Async::Ready(())),
                Some(Err(e)) => Err(e.clone()),
                None => Ok(futures::Async::Pending),
            }
        } else {
            Ok(futures::Async::Pending)
        }
    }
}