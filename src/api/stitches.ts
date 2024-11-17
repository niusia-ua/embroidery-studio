import { invoke } from "@tauri-apps/api/core";
import type { FullStitch, Line, Node, PartStitch } from "#/types/pattern/pattern";

export type Stitch = { full: FullStitch } | { part: PartStitch } | { node: Node } | { line: Line };

export const addStitch = (patternKey: string, stitch: Stitch) => invoke<void>("add_stitch", { patternKey, stitch });
