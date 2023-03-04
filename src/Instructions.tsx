import {useState} from "react";
import {MdArrowBack} from "react-icons/md";
import {Link} from "react-router-dom";
import {GoChevronDown, GoChevronRight} from "react-icons/go";
import MadeBy from "./components/MadeBy";

const Instructions = () => {
  const [isEditingOpen, setIsEditingOpen] = useState(false);
  const [isRunningOpen, setIsRunningOpen] = useState(false);

  const toggleEditing = () => {
    setIsEditingOpen(!isEditingOpen);
    setIsRunningOpen(false);
  };

  const toggleRunning = () => {
    setIsRunningOpen(!isRunningOpen);
    setIsEditingOpen(false);
  };

  return (
    <div className="mx-4 font-montserrat" style={{height: "400px"}}>
      <div className=" flex flex-row text-xl font-bold mt-4 place-items-center">
        <p className="text-3xl">
          Instructions
        </p>
        <a
          className=" ml-4 text-blue-500 text-xl"
          href=" https://github.com/juhamikael/FileOrganizer" target="_blank">
          Read more
        </a>
      </div>
      <div
        className={`${!isEditingOpen ? "hover:text-blue-500" : "hover:text-red-500"} text-lg font-bold mt-4 cursor-pointer h-fit `}
        onClick={toggleEditing}>
        <div className="flex flex-row place-items-center">
          Editing config file
          <span className="ml-2">{isEditingOpen ? <GoChevronDown/> : <GoChevronRight/>}</span>
        </div>
      </div>
      {isEditingOpen && (
        <div className="text-sm ml-4">
          <ol className="list-decimal">
            <li>Open the configuration file with any text editor</li>
            <li>Change the values to your liking</li>
            <li>
              <strong>IMPORTANT! </strong>Keep the original format
            </li>
            <li>Save the file</li>
          </ol>
        </div>
      )}
      <div
        className={`${!isRunningOpen ? "hover:text-blue-500" : "hover:text-red-500"} text-lg font-bold mt-4 cursor-pointer h-fit `}
        onClick={toggleRunning}>

        <div className="flex flex-row place-items-center">
          Running the program
          <span className="ml-2">{isRunningOpen ? <GoChevronDown/> : <GoChevronRight/>}</span>
        </div>
      </div>
      {isRunningOpen && (
        <div className="text-sm ml-4">
          <ol className="list-decimal">
            <li>At first the button is disabled</li>
            <li>You need to enter any valid path to the input field</li>
            <li>
              Button becomes <strong>enabled</strong>, no matter what your input
              is
            </li>
            <li>Click the button to run the program</li>
            <li>
              If your path is valid, the program will run and organize your
              files to corresponding folders
            </li>
            <li>
              After run, the button will be disabled again and you will see the
              output on the screen
            </li>
          </ol>
        </div>
      )}
      <Link
        to="/"
        className="flex flex-row fixed bottom-0 place-items-center mb-6 text-white"
        style={{
          color: "white",
        }}
      >
        <MdArrowBack size={40} className="hover:text-blue-500"/>
      </Link>

      <MadeBy/>

    </div>

  );
};

export default Instructions;
