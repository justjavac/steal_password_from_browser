import { useState } from "react";
import { invoke } from "@tauri-apps/api";

import { Browserslist } from "./Browserslist";
import { type Password, Passwordslist } from "./Passwordslist";

function App() {
  const [passwords, setPasswords] = useState<Password[]>();

  function handleClick(browser: string) {
    invoke<Password[]>("passwordslist", { browser }).then(setPasswords);
  }

  return (
    <div className="h-screen flex gap-2 divide-x divide-gray-700">
      <Browserslist onClick={handleClick} />
      {passwords == null ? <Empty /> : <Passwordslist passwords={passwords} />}
    </div>
  );
}

function Empty() {
  return (
    <div className="w-full h-full flex items-center justify-center text-red-400">
      <h1>Click left to select a browser</h1>
    </div>
  )
}

export default App;
