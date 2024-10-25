import { BorshSchema } from "borsher";

const FontSchema = BorshSchema.Struct({
  name: BorshSchema.String,
  size: BorshSchema.u16,
  weight: BorshSchema.u16,
  italic: BorshSchema.bool,
});

const PageMarginsSchema = BorshSchema.Struct({
  left: BorshSchema.f32,
  right: BorshSchema.f32,
  top: BorshSchema.f32,
  bottom: BorshSchema.f32,
  header: BorshSchema.f32,
  footer: BorshSchema.f32,
});

export const PrintSettingsSchema = BorshSchema.Struct({
  font: FontSchema,
  header: BorshSchema.String,
  footer: BorshSchema.String,
  margins: PageMarginsSchema,
  show_page_numbers: BorshSchema.bool,
  show_adjacent_page_numbers: BorshSchema.bool,
  center_chart_on_pages: BorshSchema.bool,
});
