import { useEffect, useState } from "preact/hooks";

import { getBrowsers } from "./tauri";
import type { Browser } from "./tauri";

export interface BrowserslistProps {
  onClick: (browser: string) => void;
}

export function Browserslist(props: BrowserslistProps) {
  const [browsers, setBrowsers] = useState<Browser[]>([]);
  const [active, setActive] = useState<string>();

  useEffect(() => {
    getBrowsers().then(setBrowsers);
  }, []);

  return (
    <ul class="w-64 pl-2 pt-5">
      {browsers.map((x) => (
        <li
          key={x.name}
          class={`px-4 py-2 rounded-sm ${
            active === x.name ? "bg-gray-600 font-medium dark:text-gray-300" : "bg-transparent"
          } ${
            x.installed
              ? "cursor-pointer hover:text-gray-800 dark:text-white dark:hover:text-gray-300 hover:font-medium"
              : "text-gray-400 dark:text-gray-400"
          }`}
          onClick={() => {
            if (!x.installed) return;
            setActive(x.name);
            props.onClick(x.name);
          }}
        >
          {x.name}
        </li>
      ))}
    </ul>
  );
}
