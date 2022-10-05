import App from "../static/build/ssr.js";
import { parse } from "node-html-parser";
import template from "../static/index.html";

export const ssrEntry = (path) => {
  const root = parse(template);
  const { html } = App.render({url: `/${path}`});

  root.querySelector('body').set_content(html);

  return root.toString();
}
