import {MdClose} from "react-icons/md";
import {useEffect, useState} from "react";
import {notify} from "../utils/Toastify";
import {invoke} from "@tauri-apps/api/tauri";
import {Switch} from "@material-tailwind/react";
import {Link} from "react-router-dom";
import styles from "./Styles"

interface SidePanelProps {
  backupState: boolean;
  isOpen: boolean;
  onClose: () => void;
  setBackUpStatus: (status: boolean) => void;
}

function SidePanel({isOpen, onClose, backupState, setBackUpStatus}: SidePanelProps,) {
  const [openConfigMsg, setOpenConfigMsg] = useState("");
  const [switchState, setSwitchState] = useState(false);
  useEffect(() => {
    if (openConfigMsg.startsWith("Error:")) {
      notify(openConfigMsg, "error", "center-right", 0.8);
    } else if (openConfigMsg.startsWith("Success:")) {
      notify(openConfigMsg, "success", "center-right", 0.8);
    }
  }, [openConfigMsg]);

  const openConfig = async () => {
    await setOpenConfigMsg(await invoke("open_config_file"));
  };

  const onChangeHandler = async () => {
    setBackUpStatus(!switchState);
    await setSwitchState(!switchState);
    console.log(!backupState)
  };

  useEffect(() => {
    setSwitchState(backupState);
  }, [backupState]);

  return (
    <>
      <div style={styles(isOpen).panelStyle}>
        <div className="flex justify-center">
          <div className="flex flex-col font-montserrat">
            <button
              onClick={openConfig}
              className="mb-4 mt-5 w-46 text-white rounded-md border-transparent py-2 px-4 transition duration-150
              ease-in-out shadow-sm no-border
              hover:text-white
              hover:bg-blue-700"
            >
              Open configuration file
            </button>
            <Link
              to="/instructions"
              className="mb-4 w-46 text-white rounded-md border-transparent py-2 px-4 font-medium transition
              duration-150 ease-in-out shadow-sm
              hover:text-white
              hover:bg-blue-700
              "
            >
              Open instructions
            </Link>
          </div>
        </div>

        <div className="ml-5 flex font-montserrat font-medium space-y-4">
          <div className="flex flex-row mx-5 ">
            <Switch onChange={onChangeHandler} checked={switchState}/>
            <p className="text-white mx-4">Enable Backup</p>
          </div>
        </div>
        <div className="ml-6 flex flex-row place-items-center font-montserrat font-bold"></div>
        <div className="mt-5 fixed bottom-0">
          <button
            className="mb-4 ml-4 w-fit right-0 rounded-md border-transparent py-2 px-4 hover:border-red-500
            text-sm font-medium text-gray-700 transition duration-150 ease-in-out shadow-sm"
            onClick={onClose}
          >
            <MdClose size={20} className="text-red-500"/>
          </button>
        </div>
      </div>
      <div style={styles(isOpen).overlayStyle} onClick={onClose}></div>
    </>
  );
}

export default SidePanel;
