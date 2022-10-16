import App from "./App.svelte";

const app: App = new App({
  target: document.body,
  hydrate: true,
});

export default app;
