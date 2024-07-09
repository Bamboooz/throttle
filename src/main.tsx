import React from "react";
import ReactDOM from "react-dom/client";
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";

import App from "./App";

import "./styles.css";

window.onload = async () => {
    await unregister("Alt+T");
        
    await register("Alt+T", async () => {
        const visible = await appWindow.isVisible();

        if (visible) {
            appWindow.hide();
        } else {
            appWindow.show();
        }
    });
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
