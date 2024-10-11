import type { FullStitch, Line, Node, PartStitch } from "../pattern";

export interface StitchEventPayload<T> {
  patternKey: string;
  payload: T;
}

export interface CreatedStitchPayload {
  full?: FullStitch;
  part?: PartStitch;
  node?: Node;
  line?: Line;
}

export interface RemovedStitchPayload {
  fullstitches?: FullStitch[];
  partstitches?: PartStitch[];
  line?: Line;
  node?: Node;
}
