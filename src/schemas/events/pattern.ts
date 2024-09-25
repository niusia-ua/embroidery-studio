import { BorshSchema } from "borsher";
import { FullStitchSchema, LineSchema, NodeSchema, PartStitchSchema } from "../pattern";
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

const CreatedStitchPayloadSchema = BorshSchema.Enum({
  full: FullStitchSchema,
  part: PartStitchSchema,
  node: NodeSchema,
  line: LineSchema,
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

export const RemovedStitchPayloadSchema = BorshSchema.Struct({
  fullstitches: BorshSchema.Vec(FullStitchSchema),
  partstitches: BorshSchema.Vec(PartStitchSchema),
  line: BorshSchema.Option(LineSchema),
  node: BorshSchema.Option(NodeSchema),
});

export const RemovedStitchEventPayloadSchema = BorshSchema.Struct({
  patternKey: BorshSchema.String,
  payload: RemovedStitchPayloadSchema,
});
