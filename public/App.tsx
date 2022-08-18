import { useState } from "preact/hooks";
import { Browserslist } from "./Browserslist";
import { Passwordslist } from "./Passwordslist";
import { getPaswords } from "./tauri";
import type { Password } from "./tauri";

function App() {
  const [passwords, setPasswords] = useState<Password[]>();

  function handleClick(browser: string) {
    getPaswords(browser).then(setPasswords);
  }

  return (
    <div class="h-screen flex gap-2 divide-x divide-gray-700">
      <Browserslist onClick={handleClick} />
      {passwords == null ? <Empty /> : <Passwordslist passwords={passwords} />}
    </div>
  );
}

function Empty() {
  return (
    <div class="w-full h-full flex items-center justify-center text-red-400">
      <h1>Click left to select a browser</h1>
    </div>
  );
}

export default App;
