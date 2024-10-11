import { emit } from "@tauri-apps/api/event";
import { type CreatedStitchPayload } from "#/types/events/pattern";

export function emitStitchCreated(patternKey: string, payload: CreatedStitchPayload) {
  return emit("pattern:stitch:create", { patternKey, payload });
}

export function emitStitchRemoved(patternKey: string, payload: CreatedStitchPayload) {
  return emit("pattern:stitch:remove", { patternKey, payload });
}
