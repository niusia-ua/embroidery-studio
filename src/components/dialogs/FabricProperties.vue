<template>
  <div class="grid grid-flow-col grid-cols-2 grid-rows-2 gap-x-2">
    <Fieldset legend="Count" pt:content:class="grid grid-flow-col grid-cols-2 grid-rows-2 gap-4">
      <FloatLabel variant="on">
        <InputNumber
          id="stitches-horizontally"
          v-model="fabric.spi[0]"
          :show-buttons="true"
          :allow-empty="false"
          :min="1"
          :input-style="{ background: dt('dialog.background') }"
          @value-change="
            (value) => {
              if (squareStitches) fabric.spi[1] = value;
            }
          "
        />
        <label for="stitches-horizontally">Horizontally</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <InputNumber
          id="stitches-vertically"
          v-model="fabric.spi[1]"
          :disabled="squareStitches"
          :show-buttons="true"
          :allow-empty="false"
          :min="1"
          :input-style="{ background: dt('dialog.background') }"
        />
        <label for="stitches-vertically">Vertically</label>
      </FloatLabel>

      <label class="flex items-center gap-2">
        <!-- TODO: add support for non-square stitches. -->
        <!-- Currently, we are not supporting non-square stitches. -->
        <Checkbox v-model="squareStitches" binary :disabled="true" />
        Square stitches
      </label>
    </Fieldset>

    <Fieldset legend="Size">
      <div>
        <label class="flex items-center gap-2">
          <RadioButton v-model="fabricSizeOption" value="final-size" />
          Specify the final size:
        </label>

        <div class="mx-8 my-4 flex items-center gap-2">
          <FloatLabel variant="on">
            <InputNumber
              id="size-width"
              v-model="fabricSizeFinal.width"
              :allow-empty="false"
              :min="0.1"
              :step="fabricSizeMeasurement === 'inches' ? 0.1 : 1"
              :input-style="{ background: dt('dialog.background') }"
            />
            <label for="size-width">Width</label>
          </FloatLabel>

          by

          <FloatLabel variant="on">
            <InputNumber
              id="size-height"
              v-model="fabricSizeFinal.height"
              :allow-empty="false"
              :min="0.1"
              :step="fabricSizeMeasurement === 'inches' ? 0.1 : 1"
              :input-style="{ background: dt('dialog.background') }"
            />
            <label for="size-height">Height</label>
          </FloatLabel>

          <label class="flex items-center gap-2">
            <RadioButton v-model="fabricSizeMeasurement" value="inches" />
            inches
          </label>

          <label class="flex items-center gap-2">
            <RadioButton v-model="fabricSizeMeasurement" value="mm" />
            mm
          </label>
        </div>
      </div>

      <div>
        <label class="flex items-center gap-2">
          <RadioButton v-model="fabricSizeOption" value="stitches" />
          Specify the size in stitches:
        </label>

        <div class="mx-8 my-4 flex items-center gap-2">
          <FloatLabel variant="on">
            <InputNumber
              id="size-width"
              v-model="fabricSizeStitches.width"
              :allow-empty="false"
              :min="1"
              :input-style="{ background: dt('dialog.background') }"
            />
            <label for="size-width">Width</label>
          </FloatLabel>

          by

          <FloatLabel variant="on">
            <InputNumber
              id="size-height"
              v-model="fabricSizeStitches.height"
              :allow-empty="false"
              :min="1"
              :input-style="{ background: dt('dialog.background') }"
            />
            <label for="size-height">Height</label>
          </FloatLabel>

          stitches
        </div>
      </div>

      <p>
        Size (WxH):
        {{ patternProperties.width }}x{{ patternProperties.height }} stitches,
        {{ stitches2inches(patternProperties.width, fabric.spi[0]) }}x{{
          stitches2inches(patternProperties.height, fabric.spi[1])
        }}
        inches ({{ stitches2mm(patternProperties.width, fabric.spi[0]) }}x{{
          stitches2mm(patternProperties.height, fabric.spi[1])
        }}
        mm)
      </p>
    </Fieldset>

    <Fieldset legend="Color">
      <Listbox
        :model-value="{ name: fabric.name, color: fabric.color }"
        :options="fabricColors"
        scroll-height="100%"
        empty-message="No fabric colors found"
        :dt="{ list: { header: { padding: '4px 8px' } } }"
        pt:root:class="flex flex-col h-full rounded-none border-0"
        :pt:root:style="{ background: dt('dialog.background') }"
        pt:list-container:class="grow"
        pt:list:class="grid gap-1"
        :pt:list:style="{ gridTemplateColumns: `repeat(8, minmax(0px, 1fr))` }"
        pt:option:class="p-0"
        @value-change="
          ({ name, color }) => {
            fabric.name = name;
            fabric.color = color;
          }
        "
      >
        <template #option="{ option, selected }">
          <div
            v-tooltip="{ value: option.name, showDelay: 200 }"
            class="h-8 w-full"
            :style="{
              backgroundColor: `#${option.color}`,
              boxShadow: selected
                ? `inset 0 0 0 2px #${option.color}, inset 0 0 0 4px ${contrastColor(option.color)}`
                : '',
            }"
          ></div>
        </template>
      </Listbox>

      <p>Selected color: {{ fabric.name }}</p>
    </Fieldset>

    <Fieldset legend="Kind">
      <Select
        v-model="fabric.kind"
        editable
        :options="fabricKinds"
        :pt:root:style="{ background: dt('dialog.background') }"
      />
    </Fieldset>
  </div>
</template>

<script setup lang="ts">
  import { path } from "@tauri-apps/api";
  import { readTextFile } from "@tauri-apps/plugin-fs";
  import { computed, inject, onMounted, reactive, ref, watch, type Ref } from "vue";
  import { dt } from "@primevue/themes";
  import { Checkbox, Fieldset, FloatLabel, InputNumber, Listbox, RadioButton, Select } from "primevue";
  import type { DynamicDialogInstance } from "primevue/dynamicdialogoptions";
  import { inches2mm, mm2inches, size2stitches, stitches2inches, stitches2mm } from "#/utils/measurement";
  import { contrastColor } from "#/utils/color";
  import type { Fabric, PatternProperties } from "#/schemas/pattern";

  const dialogRef = inject<Ref<DynamicDialogInstance>>("dialogRef")!;

  // Copy the data from the dialog reference to a reactive objects.
  const patternProperties = reactive<PatternProperties>({ ...dialogRef.value.data.patternProperties });
  const fabric = reactive<Fabric>({ ...dialogRef.value.data.fabric });

  const squareStitches = ref(true);

  const fabricSizeOption = ref<"final-size" | "stitches">("final-size");
  const fabricSizeMeasurement = ref<"inches" | "mm">("inches");
  const fabricSizeFinal = reactive({ width: 4.29, height: 5.71 }); // 60x80 stitches in inches
  const fabricSizeStitches = reactive({
    width: patternProperties.width ?? 60,
    height: patternProperties.height ?? 80,
  });

  const fabricSize = computed(() => {
    const width = fabricSizeMeasurement.value === "inches" ? fabricSizeFinal.width : mm2inches(fabricSizeFinal.width);
    const height =
      fabricSizeMeasurement.value === "inches" ? fabricSizeFinal.height : mm2inches(fabricSizeFinal.height);
    return { width, height };
  });

  const patternSize = computed(() => {
    if (fabricSizeOption.value === "final-size") {
      return {
        width: size2stitches(fabricSize.value.width, fabric.spi[0]),
        height: size2stitches(fabricSize.value.height, fabric.spi[1]),
      };
    } else {
      return {
        width: fabricSizeStitches.width,
        height: fabricSizeStitches.height,
      };
    }
  });

  watch(patternSize, (size) => {
    patternProperties.width = size.width;
    patternProperties.height = size.height;
  });

  watch(fabricSizeMeasurement, (measurement) => {
    const { width, height } = fabricSizeFinal;
    fabricSizeFinal.width = measurement === "inches" ? mm2inches(width) : inches2mm(width);
    fabricSizeFinal.height = measurement === "inches" ? mm2inches(height) : inches2mm(height);
  });

  const fabricColors = ref<{ name: string; color: string }[]>([]);
  const fabricKinds = ref(["Aida", "Evenweave", "Linen"]);

  onMounted(async () => {
    fabricColors.value = JSON.parse(await readTextFile(await path.resolveResource("resources/fabric-colors.json")));
  });
</script>
