import { emit } from "@tauri-apps/api/event";
import { borshSerialize } from "borsher";
import { type CreatedStitchPayload, CreatedStitchEventPayloadSchema } from "#/schemas/events/pattern";

export function emitStitchCreated(patternKey: string, payload: CreatedStitchPayload) {
  const buffer = borshSerialize(CreatedStitchEventPayloadSchema, { patternKey, payload });
  return emit("pattern:stitch:create", buffer.toJSON().data);
}

export function emitStitchRemoved(patternKey: string, payload: CreatedStitchPayload) {
  const buffer = borshSerialize(CreatedStitchEventPayloadSchema, { patternKey, payload });
  return emit("pattern:stitch:remove", buffer.toJSON().data);
}
