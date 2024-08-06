import { createApp } from "vue";
import { PrimeVue } from "@primevue/core";
import Aura from "@primevue/themes/aura";

import "primeflex/primeflex.css";
import "primeicons/primeicons.css";

import App from "./App.vue";

const app = createApp(App);

app.use(PrimeVue, {
  theme: { preset: Aura },
});

app.mount("#app");
