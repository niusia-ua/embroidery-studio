import { describe, expect, test } from "vitest";
import { mount } from "@vue/test-utils";
import { createTestingPinia } from "@pinia/testing";
import { PrimeVue } from "@primevue/core";
import Popover from "primevue/popover";
import PalettePanel from "./PalettePanel.vue";
import type { Blend, PaletteItem } from "#/types/pattern/pattern";
import ToggleSwitch from "primevue/toggleswitch";
import Checkbox from "primevue/checkbox";

describe("PalettePanel", () => {
  const BLENDS: Blend[] = [
    {
      brand: "Anchor",
      number: "9159",
      strands: 1,
    },
    {
      brand: "Madeira",
      number: "0705",
      strands: 1,
    },
  ];
  const PALETTE: PaletteItem[] = [
    {
      brand: "DMC",
      number: "310",
      name: "Black",
      color: "2C3225",
      strands: {},
    },
    {
      brand: "Anchor",
      number: "9159",
      name: "Glacier Blue",
      color: "B2D8E5",
      strands: {},
    },
    {
      brand: "Madeira",
      number: "0705",
      name: "Plum-DK",
      color: "901b6b",
      strands: {},
    },
    {
      brand: "Blends",
      number: "",
      name: "",
      color: "A382AE",
      blends: BLENDS,
      strands: {},
    },
  ];

  test("renders an empty palette", () => {
    const wrapper = mount(PalettePanel, { global: { plugins: [createTestingPinia(), PrimeVue] } });
    expect(wrapper.exists()).toBe(true);
  });

  test("renders palette", () => {
    const wrapper = mount(PalettePanel, {
      props: { palette: PALETTE },
      global: { plugins: [createTestingPinia(), PrimeVue] },
    });

    const paletteItems = wrapper.findAll("ul > li > div");
    expect(paletteItems.length).toBe(PALETTE.length);

    PALETTE.forEach(({ name }, i) => {
      const pi = paletteItems[i];
      expect(pi.text()).toContain(name);
      expect(pi.find("div").isVisible()).toBe(true);
    });
  });

  test("hides palette items' titles", async () => {
    const wrapper = mount(PalettePanel, {
      props: { palette: PALETTE },
      global: { plugins: [createTestingPinia(), PrimeVue] },
    });
    const popover = wrapper.findComponent(Popover);
    const paletteItemsTitles = wrapper.findAll("ul > li > div > div");

    // The palette items' titles are visible by default.
    for (const title of paletteItemsTitles) expect(title.isVisible()).toBe(true);

    // Open the popover.
    await wrapper.get("button").trigger("click");

    // Enable the "color only" mode.
    await popover.getComponent(ToggleSwitch).get("input").trigger("change");

    // The palette items' titles are invisible when the "color only" mode is enabled.
    for (const title of paletteItemsTitles) expect(title.isVisible()).toBe(false);
  });

  test("updates palette items' titles", async () => {
    const wrapper = mount(PalettePanel, {
      props: { palette: PALETTE },
      global: { plugins: [createTestingPinia(), PrimeVue] },
    });
    const popover = wrapper.findComponent(Popover);
    const paletteItemsTitles = wrapper.findAll("ul > li > div > div");
    const initialTexts = paletteItemsTitles.map((title) => title.text());

    // Open the popover.
    await wrapper.get("button").trigger("click");
    const checkboxes = popover.findAllComponents(Checkbox);

    // Hide the color brand.
    const checkboxBrand = checkboxes[0].get("input");
    await checkboxBrand.trigger("change");
    paletteItemsTitles.forEach((title, i) => {
      expect(title.text()).not.toEqual(initialTexts[i]);
    });
    await checkboxBrand.trigger("change");

    // Hide the color number.
    const checkboxNumber = checkboxes[1].get("input");
    await checkboxNumber.trigger("change");
    paletteItemsTitles.forEach((title, i) => {
      expect(title.text()).not.toEqual(initialTexts[i]);
    });
    await checkboxNumber.trigger("change");

    // Hide the color name.
    const checkboxName = checkboxes[2].get("input");
    await checkboxName.trigger("change");
    paletteItemsTitles.forEach((title, i) => {
      // The last palette item is a blend, so its title is not affected by the "color name" option.
      if (i !== 3) expect(title.text()).not.toEqual(initialTexts[i]);
    });
    await checkboxName.trigger("change");

    // Now, the palette items' titles are back to their initial state.
    paletteItemsTitles.forEach((title, i) => {
      expect(title.text()).toEqual(initialTexts[i]);
    });
  });
});
