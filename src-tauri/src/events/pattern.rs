use borsh::{BorshDeserialize, BorshSerialize};
use tauri::{AppHandle, Emitter, Listener, Manager, WebviewWindow};

use crate::{
  pattern::{Stitch, StitchConflicts},
  state::{AppStateType, PatternKey},
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
struct EventStitchPayload<T> {
  pattern_key: PatternKey,
  payload: T,
}

static EVENT_STITCH_CREATE: &str = "pattern:stitch:create";
static EVENT_STITCH_REMOVE: &str = "pattern:stitch:remove";

pub fn setup_event_handlers(window: &WebviewWindow, app_handle: &AppHandle) {
  log::trace!("Setting up pattern event handlers");

  let win = window.clone();
  let handle = app_handle.clone();
  window.clone().listen(EVENT_STITCH_CREATE, move |e| {
    log::trace!("Received stitch create event");
    let state = handle.state::<AppStateType>();
    let mut state = state.write().unwrap();

    let payload = serde_json::from_str::<Vec<u8>>(e.payload()).unwrap();
    let EventStitchPayload { pattern_key, payload } =
      borsh::from_slice::<EventStitchPayload<Stitch>>(&payload).unwrap();
    // This is safe because the event is only emitted when the pattern exists.
    let pattern = state.patterns.get_mut(&pattern_key).unwrap();

    emit_remove_stitches(&win, pattern_key, pattern.add_stitch(payload));
  });

  let handle = app_handle.clone();
  window.clone().listen(EVENT_STITCH_REMOVE, move |e| {
    log::trace!("Received stitch remove event");
    let state = handle.state::<AppStateType>();
    let mut state = state.write().unwrap();

    let payload = serde_json::from_str::<Vec<u8>>(e.payload()).unwrap();
    let EventStitchPayload { pattern_key, payload } =
      borsh::from_slice::<EventStitchPayload<Stitch>>(&payload).unwrap();
    // This is safe because the event is only emitted when the pattern exists.
    let pattern = state.patterns.get_mut(&pattern_key).unwrap();
    pattern.remove_stitch(payload);
  });
}

fn emit_remove_stitches(window: &WebviewWindow, pattern_key: PatternKey, payload: StitchConflicts) {
  log::trace!("Emitting remove stitches event");
  let payload = EventStitchPayload { pattern_key, payload };
  let payload = borsh::to_vec(&payload).unwrap();
  window.emit("pattern:stitches:remove", payload).unwrap();
}
