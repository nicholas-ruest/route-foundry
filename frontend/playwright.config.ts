import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "e2e",
  fullyParallel: false,
  workers: 1,
  use: {
    baseURL: "http://127.0.0.1:14273",
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  reporter: [["html", { open: "never" }], ["list"]],
});
