import { PrimeVue } from "@primevue/core";
import Aura from "@primevue/themes/aura";
import { createPinia } from "pinia";
import { createPersistedState } from "pinia-plugin-persistedstate";
import ConfirmationService from "primevue/confirmationservice";
import { createApp } from "vue";

import "primeflex/primeflex.css";
import "primeicons/primeicons.css";
import "./assets/styles.css";

import App from "./App.vue";

const pinia = createPinia();
pinia.use(createPersistedState({ storage: sessionStorage, auto: true }));

const app = createApp(App);
app.use(pinia);
app.use(PrimeVue, { theme: { preset: Aura } });
app.use(ConfirmationService);

app.mount("#app");
