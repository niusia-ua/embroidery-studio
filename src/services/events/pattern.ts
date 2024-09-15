import { appWindow } from "@tauri-apps/api/window";
import { borshSerialize } from "borsher";
import { type CreatedStitchPayload, CreatedStitchEventPayloadSchema } from "#/schemas/events/pattern";

export function emitStitchCreated(patternKey: string, payload: CreatedStitchPayload) {
  const buffer = borshSerialize(CreatedStitchEventPayloadSchema, { patternKey, payload });
  return appWindow.emit("pattern:stitch:create", buffer.toJSON().data);
}
