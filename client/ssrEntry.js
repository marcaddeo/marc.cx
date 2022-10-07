import App from "../static/build/ssr.js";

export const ssrEntry = (path) => {
  const { html } = App.render({url: `/${path}`});
  return html;
}
