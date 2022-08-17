import { useState } from "react";

export interface Password {
  url: string;
  username: string;
  password: string;
}

export function Passwordslist({ passwords }: { passwords: readonly Password[] }) {
  const [search, setSearch] = useState<string>("");

  return (
    <div className="flex flex-col gap-5 flex-1">
      <header className="flex gap-3 items-center justify-between pt-5 px-5">
        <h2 className="font-medium text-lg !leading-none text-black dark:text-white">
          <strong>{passwords.length}</strong> passwords found
        </h2>
        <label className="inline-flex items-center justify-between gap-1 px-3 transition-colors duration-150 ease-in-out group bg-gray-100 dark:bg-gray-700 border border-transparent focus-within:bg-white focus-within:border focus-within:border-gray-400 rounded-sm h-9 w-70">
          <input
            placeholder="Search..."
            autoComplete="off"
            className="flex-1 bg-transparent placeholder:text-gray-400 border-none outline-none min-w-10 dark:text-white"
            type="text"
            onChange={(e) => setSearch(e.target.value)}
          />
          <svg
            className="text-gray-400"
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M14.2716 13.1684L11.3313 10.2281C12.0391 9.28573 12.4213 8.13865 12.42 6.96C12.42 3.94938 9.97062 1.5 6.96 1.5C3.94938 1.5 1.5 3.94938 1.5 6.96C1.5 9.97062 3.94938 12.42 6.96 12.42C8.13865 12.4213 9.28573 12.0391 10.2281 11.3313L13.1684 14.2716C13.3173 14.4046 13.5114 14.4756 13.711 14.47C13.9105 14.4645 14.1004 14.3827 14.2415 14.2415C14.3827 14.1004 14.4645 13.9105 14.47 13.711C14.4756 13.5114 14.4046 13.3173 14.2716 13.1684V13.1684ZM3.06 6.96C3.06 6.18865 3.28873 5.43463 3.71727 4.79328C4.14581 4.15192 4.7549 3.65205 5.46753 3.35687C6.18017 3.06169 6.96433 2.98446 7.72085 3.13494C8.47738 3.28542 9.17229 3.65686 9.71772 4.20228C10.2631 4.74771 10.6346 5.44262 10.7851 6.19915C10.9355 6.95567 10.8583 7.73983 10.5631 8.45247C10.2679 9.1651 9.76808 9.77419 9.12672 10.2027C8.48537 10.6313 7.73135 10.86 6.96 10.86C5.92604 10.8588 4.93478 10.4475 4.20365 9.71635C3.47253 8.98522 3.06124 7.99396 3.06 6.96V6.96Z"
              fill="currentColor"
            >
            </path>
          </svg>
        </label>
      </header>
      <div className="h-full border-t border-gray-700 overscroll-contain overflow-auto">
        <table className="w-full table-fixed border-separate border-spacing-2 mx-4">
          <tbody>
            {passwords.filter((x) => x.url.includes(search))?.map((x) => (
              <tr key={x.url}>
                <td className="text-sm dark:text-white">{x.url}</td>
                <td className="text-sm dark:text-white">{x.username}</td>
                <td className="text-sm dark:text-white">{x.password}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
