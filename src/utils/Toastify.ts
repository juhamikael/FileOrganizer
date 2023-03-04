import {toast, ToastOptions} from "react-toastify";

const notify = (text: string, type: string, position = "top-center", fontSize = 1) => {
  if (text.includes("Error:")) {
    text = text.split("Error:")[1];
  }
  if (text.includes("Success:")) {
    text = text.split("Success:")[1];
  }

  const toastOptions: ToastOptions = {
    position: position as ToastOptions["position"],
    autoClose: 5000,
    hideProgressBar: false,
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    progress: undefined,
    theme: "dark",
    style: {
      fontSize: `${fontSize}rem`,
      fontFamily: "Montserrat",
      backgroundColor: "#0f0f0f",
    },
  };

  if (type === "error") {
    toast.error(text, toastOptions);
  } else if (type === "success") {
    toast.success(text, toastOptions);
  }
};

export {notify};
