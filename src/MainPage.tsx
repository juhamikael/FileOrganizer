import {useState, useEffect} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "react-toastify/dist/ReactToastify.css";
import SidePanel from "./components/SidePanel";
import {notify} from "./utils/Toastify";
import "./App.css";
import {FaAlignLeft} from "react-icons/fa";
import MadeBy from "./components/MadeBy";

function MainPage() {
  const [fileOrganized, setfileOrganized] = useState("");
  const [path, setPath] = useState("");
  const [isInputEmpty, setIsInputEmpty] = useState<boolean>(true);
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [backUpStatus, setBackUpStatus] = useState<boolean>(true);

  useEffect(() => {
    if (path.length > 0) {
      setIsInputEmpty(false);
    } else {
      setIsInputEmpty(true);
    }
  }, [path]);

  useEffect(() => {
    if (isInputEmpty) {
      setIsInputEmpty(true);
    } else {
      setIsInputEmpty(false);
    }
  }, [isInputEmpty]);

  useEffect(() => {
    if (fileOrganized.startsWith("Error:")) {
      notify(fileOrganized, "error");
    } else if (
      fileOrganized.length > 0 &&
      !fileOrganized.startsWith("Error:")
    ) {
      notify(fileOrganized, "success");
    }
  }, [fileOrganized]);

  async function organizeFiles() {
    setfileOrganized(await invoke("organize_files", {path: path, is_backup: backUpStatus}));
    setPath("");
  }

  const buttonStyle =
    "rounded-md border-transparent py-2 px-4 text-sm font-medium text-gray-700 bg-white transition duration-150 ease-in-out shadow-sm";

  return (
    <div className="">
      <div className="flex justify-center relative">
        <div className="flex flex-col mt-6">
          {isOpen && (
            <SidePanel isOpen={isOpen as boolean} onClose={() => setIsOpen(false)}
                       backupState={backUpStatus as boolean} setBackUpStatus={setBackUpStatus}
            />
          )}
          <div
            className="w-fit p-2 absolute left-0 mx-4 mt-5 cursor-pointer hover:text-blue-500"
            onClick={() => setIsOpen(true)}
          >
            <FaAlignLeft className="text-2xl "/>
          </div>

          <h1 className="text-3xl my-5 font-montserrat font-bold">
            File Organizer
          </h1>
        </div>
      </div>

      <div className="flex flex-col mt-10">
        <div className="flex justify-center">
          <input
            className="bg-gray-200 w-full mx-6 text-black font-montserrat text-center h-10"
            onChange={(e) => setPath(e.currentTarget.value)}
            placeholder="Enter folder to organize..."
            value={path}
          />
        </div>
        <button
          className={`${
            isInputEmpty ? "input-empty" : "input-filled"
          }  ${buttonStyle} mt-5 mx-4 text-black font-montserrat text-center h-10 `}
          disabled={isInputEmpty}
          type="button"
          onClick={() => organizeFiles()}
        >
          {isInputEmpty
            ? "Enter valid path to folder to organize"
            : "Click to organize files"}
        </button>
      </div>
      <MadeBy/>
    </div>
  );
}

export default MainPage;
