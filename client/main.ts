import App from "./App.svelte";

const app: App = new App({
  target: document.body,
  hydrate: true,
});

export { default as CodeBlock } from "./components/CodeBlock.wc.svelte";
export { default as Alert } from "./components/Alert.wc.svelte";
export default app;
