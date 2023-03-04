import { ToastContainer } from "react-toastify";
import MainPage from "./MainPage";
import Instructions from "./Instructions";
import { Link, Route, BrowserRouter, Routes } from "react-router-dom";

const App = () => {
  return (
    <>
      <ToastContainer />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<MainPage />} />
          <Route path="/instructions" element={<Instructions />} />
        </Routes>
      </BrowserRouter>
    </>
  );
};

export default App;
