import { PrimeVue } from "@primevue/core";
import { mount } from "@vue/test-utils";
import { describe, expect, test } from "vitest";
import { createTestingPinia } from "@pinia/testing";
import PalettePanel from "./PalettePanel.vue";

describe("PalettePanel", () => {
  test("renders correctly", () => {
    const palettePanel = mount(PalettePanel, { global: { plugins: [createTestingPinia(), PrimeVue] } });
    expect(palettePanel.exists()).toBe(true);
  });
});
