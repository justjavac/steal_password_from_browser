import { apply, setup } from "@twind/preact";
import { render } from "preact";
import App from "./App";
import reportWebVitals from "./reportWebVitals";

setup({
  props: {
    className: true,
  },
  preflight: (preflight) => ({
    ...preflight,
    body: apply("bg-white dark:bg-gray-800"),
  }),
  darkMode: "class",
  mode: process.env.NODE_ENV === "production" ? "silent" : "warn",
});

render(<App />, document.body);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals(console.log);
