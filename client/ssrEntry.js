import App from "../static/build/ssr.js";

export const ssrEntry = (props) => {
  const params = props ? JSON.parse(props) : {};
  const { html } = App.render(params);
  return html;
}
