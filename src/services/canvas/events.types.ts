import type { Point } from "pixi.js";
import type { Stitch } from "#/schemas/pattern/pattern";

export const enum EventType {
  AddStitch = "add_stitch",
  RemoveStitch = "remove_stitch",
}

/**
 * Represents the data for the `AddStitch` event.
 *
 * It has the `start` and `end` points of the stitch.
 * If the stitch is "single-point" (i.e. cross stitch, petite, bead, etc.) then these points will be the same.
 * If the stitch is "double-point" (i.e. back and straight stitch) then these points will be different.
 */
export interface AddStitchData {
  /** The stage of the event. */
  stage: AddStitchEventStage;

  /** The point where the event started. */
  start: Point;

  /** The point where the event ended. */
  end: Point;

  /** Whether the stitch should be drawn in its "alternative" view. */
  alt: boolean;

  /** Whether the stitch should be drawn in its previous view (i.e. in the same direction). */
  fixed: boolean;
}

export const enum AddStitchEventStage {
  Start = "start",
  Continue = "continue",
  End = "end",
}

export type RemoveStitchData = Stitch;
