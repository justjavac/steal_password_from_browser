declare global {
  interface Window {
    __TAURI__: {
      invoke<T>(cmd: string, args?: unknown): Promise<T>;
    };
  }
}

export interface Browser {
  name: string;
  installed: boolean;
}

/**
 * [url, username, password]
 */
export type Password = [string, string, string];

export function getBrowsers(): Promise<Browser[]> {
  if (window.__TAURI__) {
    return window.__TAURI__.invoke<Browser[]>("browserslist");
  }

  return Promise.resolve([
    { name: "Chrome", installed: true },
    { name: "Chrome Dev", installed: false },
    { name: "Chrome Beta", installed: true },
    { name: "Chrome Canary", installed: true },
    { name: "Edge", installed: true },
    { name: "Edge Dev", installed: false },
    { name: "Edge Beta", installed: false },
    { name: "Edge Canary", installed: false },
    { name: "Edge Dev Preview", installed: false },
    { name: "Edge Preview", installed: false },
  ]);
}

export function getPaswords(browser: string): Promise<Password[]> {
  if (window.__TAURI__) {
    return window.__TAURI__.invoke<Password[]>("passwordslist", { browser });
  }

  return Promise.resolve([
    ["https://github.com", "admin", "123456"],
    ["https://google.com", "root", "123456"],
    ["https://baidu.com", "justjavac", "123456"],
  ]);
}
