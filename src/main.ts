import { PrimeVue } from "@primevue/core";
import Aura from "@primevue/themes/aura";
import { createPinia } from "pinia";
import { createApp } from "vue";

import "primeflex/primeflex.css";
import "primeicons/primeicons.css";
import "./assets/styles.css";

import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(PrimeVue, {
  theme: { preset: Aura },
});

app.mount("#app");
