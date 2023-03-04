import {CSSProperties} from "react";

const styles = (isOpen: boolean) => {
    const panelStyle: CSSProperties = {
        position: "fixed",
        top: 0,
        left: 0,
        height: "100vh",
        width: "14rem",
        background: " #0f0f0f",
        boxShadow: "0px 0px 20px rgba(0, 0, 0, 0.3)",
        transform: isOpen ? "translateX(0)" : "translateX(-100%)",
        transition: "transform 0.3s ease-in-out",
        fontSize: "0.8rem",
        zIndex: 9999,
    };

    const overlayStyle: CSSProperties = {
        position: "fixed",
        top: 0,
        left: 0,
        height: "100vh",
        width: "100vw",
        background: "rgba(15, 15, 15, 0.5)",
        opacity: isOpen ? 1 : 0,
        visibility: isOpen ? "visible" : "hidden",
        transition: "opacity 0.3s ease-in-out, visibility 0.3s ease-in-out",
        zIndex: 9998,
    }
    return {panelStyle, overlayStyle}
};

export default styles;
