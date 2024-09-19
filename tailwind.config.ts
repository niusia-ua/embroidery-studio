import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.vue"],
  plugins: [require("tailwindcss-primeui")],
} satisfies Config;
