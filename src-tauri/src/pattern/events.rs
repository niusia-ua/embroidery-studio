use borsh::{BorshDeserialize, BorshSerialize};
use tauri::{AppHandle, Manager, Window};

use super::{FullStitch, FullStitchKind, Line, Node, PartStitch, PartStitchKind, PatternKey};
use crate::state::AppStateType;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
struct EventStitchPayload<T> {
  pattern_key: PatternKey,
  stitch: T,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
enum CreatedStitchPayload {
  FullStitch(FullStitch),
  PartStitch(PartStitch),
  Line(Line),
  Node(Node),
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
enum RemovedStitchPayload {
  FullStitches(Vec<FullStitch>),
  PartStitches(Vec<PartStitch>),
  Line(Line),
  Node(Node),
}

static EVENT_STITCH_CREATE: &str = "pattern:stitch:create";
static EVENT_STITCH_REMOVE: &str = "pattern:stitch:remove";

pub fn setup_pattern_event_handlers(window: Window, app_handle: AppHandle) {
  log::trace!("Setting up pattern event handlers");

  let handle = app_handle.clone();
  window.clone().listen(EVENT_STITCH_REMOVE, move |e| {
    log::trace!("Received stitch remove event");
    let state = handle.state::<AppStateType>();
    let mut state = state.write().unwrap();

    let payload = e.payload().unwrap();
    let payload = serde_json::from_str::<Vec<u8>>(payload).unwrap();
    let payload = borsh::from_slice::<EventStitchPayload<RemovedStitchPayload>>(&payload).unwrap();
    // This is safe because the event is only emitted when the pattern exists.
    let pattern = state.patterns.get_mut(&payload.pattern_key).unwrap();

    match payload.stitch {
      RemovedStitchPayload::FullStitches(fullstitches) => {
        for fullstitch in fullstitches {
          pattern.fullstitches.remove(&fullstitch);
        }
      }
      RemovedStitchPayload::PartStitches(partstitches) => {
        for partstitch in partstitches {
          pattern.partstitches.remove(&partstitch);
        }
      }
      RemovedStitchPayload::Line(line) => {
        pattern.lines.remove(&line);
      }
      RemovedStitchPayload::Node(node) => {
        pattern.nodes.remove(&node);
      }
    }
  });

  let handle = app_handle.clone();
  window.clone().listen(EVENT_STITCH_CREATE, move |e| {
    log::trace!("Received stitch create event");
    let state = handle.state::<AppStateType>();
    let mut state = state.write().unwrap();

    let payload = e.payload().unwrap();
    let payload = serde_json::from_str::<Vec<u8>>(payload).unwrap();
    let payload = borsh::from_slice::<EventStitchPayload<CreatedStitchPayload>>(&payload).unwrap();
    // This is safe because the event is only emitted when the pattern exists.
    let pattern = state.patterns.get_mut(&payload.pattern_key).unwrap();

    match payload.stitch {
      CreatedStitchPayload::FullStitch(fullstitch) => match fullstitch.kind {
        FullStitchKind::Full => {
          emit_remove_partstitches(
            &window,
            payload.pattern_key.clone(),
            pattern.partstitches.find_conflicts_with_full_stitch(&fullstitch),
          );

          let mut conflicting_fullstitches = pattern.fullstitches.find_conflicts_with_full_stitch(&fullstitch);
          if let Some(fullstitch) = pattern.fullstitches.insert(fullstitch) {
            if fullstitch.kind == FullStitchKind::Full {
              conflicting_fullstitches.push(fullstitch);
            }
          }
          emit_remove_fullstitches(&window, payload.pattern_key.clone(), conflicting_fullstitches);
        }
        FullStitchKind::Petite => {
          emit_remove_partstitches(
            &window,
            payload.pattern_key.clone(),
            pattern.partstitches.find_conflicts_with_petite_stitch(&fullstitch),
          );

          let mut conflicting_fullstitches = Vec::new();
          if let Some(fullstitch) = pattern.fullstitches.find_conflicts_with_petite_stitch(&fullstitch) {
            conflicting_fullstitches.push(fullstitch);
          }
          if let Some(fullstitch) = pattern.fullstitches.insert(fullstitch) {
            conflicting_fullstitches.push(fullstitch);
          }
          emit_remove_fullstitches(&window, payload.pattern_key.clone(), conflicting_fullstitches);
        }
      },
      CreatedStitchPayload::PartStitch(partstitch) => match partstitch.kind {
        PartStitchKind::Half => {
          emit_remove_fullstitches(
            &window,
            payload.pattern_key.clone(),
            pattern.fullstitches.find_conflicts_with_half_stitch(&partstitch),
          );

          let mut conflicting_partstitches = pattern.partstitches.find_conflicts_with_half_stitch(&partstitch);
          if let Some(partstitch) = pattern.partstitches.insert(partstitch) {
            conflicting_partstitches.push(partstitch);
          }
          emit_remove_partstitches(&window, payload.pattern_key.clone(), conflicting_partstitches);
        }
        PartStitchKind::Quarter => {
          emit_remove_fullstitches(
            &window,
            payload.pattern_key.clone(),
            pattern.fullstitches.find_conflicts_with_quarter_stitch(&partstitch),
          );

          let mut conflicting_partstitches = Vec::new();
          if let Some(partstitch) = pattern.partstitches.find_conflicts_with_quarter_stitch(&partstitch) {
            conflicting_partstitches.push(partstitch);
          }
          if let Some(partstitch) = pattern.partstitches.insert(partstitch) {
            conflicting_partstitches.push(partstitch);
          }
          emit_remove_partstitches(&window, payload.pattern_key.clone(), conflicting_partstitches);
        }
      },
      CreatedStitchPayload::Line(line) => {
        emit_remove_line(&window, payload.pattern_key.clone(), pattern.lines.insert(line))
      }
      CreatedStitchPayload::Node(node) => {
        emit_remove_node(&window, payload.pattern_key.clone(), pattern.nodes.insert(node))
      }
    };
  });
}

fn emit_remove_fullstitches(window: &Window, pattern_key: PatternKey, fullstitches: Vec<FullStitch>) {
  if fullstitches.is_empty() {
    return;
  }
  log::trace!("Emitting remove fullstitches event");
  let payload = EventStitchPayload {
    pattern_key,
    stitch: RemovedStitchPayload::FullStitches(fullstitches),
  };
  let payload = borsh::to_vec(&payload).unwrap();
  window.emit_and_trigger(EVENT_STITCH_REMOVE, payload).unwrap();
}

fn emit_remove_partstitches(window: &Window, pattern_key: PatternKey, partstitches: Vec<PartStitch>) {
  if partstitches.is_empty() {
    return;
  }
  log::trace!("Emitting remove partstitches event");
  let payload = EventStitchPayload {
    pattern_key,
    stitch: RemovedStitchPayload::PartStitches(partstitches),
  };
  let payload = borsh::to_vec(&payload).unwrap();
  window.emit_and_trigger(EVENT_STITCH_REMOVE, payload).unwrap();
}

fn emit_remove_line(window: &Window, pattern_key: PatternKey, line: Option<Line>) {
  if let Some(line) = line {
    log::trace!("Emitting remove line event");
    let payload = EventStitchPayload {
      pattern_key,
      stitch: RemovedStitchPayload::Line(line),
    };
    let payload = borsh::to_vec(&payload).unwrap();
    window.emit_and_trigger(EVENT_STITCH_REMOVE, payload).unwrap();
  }
}

fn emit_remove_node(window: &Window, pattern_key: PatternKey, node: Option<Node>) {
  if let Some(node) = node {
    log::trace!("Emitting remove node event");
    let payload = EventStitchPayload {
      pattern_key,
      stitch: RemovedStitchPayload::Node(node),
    };
    let payload = borsh::to_vec(&payload).unwrap();
    window.emit_and_trigger(EVENT_STITCH_REMOVE, payload).unwrap();
  }
}
