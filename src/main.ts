import { createApp } from "vue";
import { PrimeVue } from "@primevue/core";
import { Tooltip, ConfirmationService } from "primevue";
import Aura from "@primevue/themes/aura";
import { createPinia } from "pinia";
import { createPersistedState } from "pinia-plugin-persistedstate";

import "primeicons/primeicons.css";
import "./assets/styles.css";

import App from "./App.vue";

const pinia = createPinia();
pinia.use(createPersistedState({ storage: sessionStorage, auto: true }));

const app = createApp(App);
app.use(pinia);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      cssLayer: {
        name: "primevue",
        order: "tailwind-base, primevue, tailwind-utilities",
      },
    },
  },
});
app.use(ConfirmationService);
app.directive("tooltip", Tooltip);

app.mount("#app");
