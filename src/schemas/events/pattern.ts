import { BorshSchema } from "borsher";
import { FullStitchSchema, LineSchema, NodeSchema, PartStitchSchema } from "../pattern";
import type { FullStitch, Line, Node, PartStitch } from "../pattern";

export interface StitchEventPayload<T> {
  patternKey: string;
  payload: T;
}

export interface CreatedStitchPayload {
  fullstitch?: FullStitch;
  partstitch?: PartStitch;
  line?: Line;
  node?: Node;
}

const CreatedStitchPayloadSchema = BorshSchema.Enum({
  fullstitch: FullStitchSchema,
  partstitch: PartStitchSchema,
  line: LineSchema,
  node: NodeSchema,
});

export const CreatedStitchEventPayloadSchema = BorshSchema.Struct({
  patternKey: BorshSchema.String,
  payload: CreatedStitchPayloadSchema,
});

export interface RemovedStitchPayload {
  fullstitches?: FullStitch[];
  partstitches?: PartStitch[];
  line?: Line;
  node?: Node;
}

export const RemovedStitchPayloadSchema = BorshSchema.Enum({
  fullstitches: BorshSchema.Vec(FullStitchSchema),
  partstitches: BorshSchema.Vec(PartStitchSchema),
  line: LineSchema,
  node: NodeSchema,
});

export const RemovedStitchEventPayloadSchema = BorshSchema.Struct({
  patternKey: BorshSchema.String,
  payload: RemovedStitchPayloadSchema,
});
