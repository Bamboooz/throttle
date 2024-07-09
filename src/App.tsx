import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

import Header from "./interface/Header";
import Item from "./interface/Item";

const App: React.FC = () => {
    const [hwInfo, setHwInfo] = useState({
        cpu_usage: 0,
        cpu_temp: 0,
        gpu_usage: 0,
        gpu_temp: 0,
        ram_usage: 0,
    });

    useEffect(() => {
        const fetchHwInfo = async () => {

        };

        fetchHwInfo();
        
        const interval = setInterval(fetchHwInfo, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <>
            <div className="w-screen h-screen flex flex-col">
                <Header />

                <Item text="CPU Usage" value={hwInfo.cpu_usage} unit="%" />
                <Item text="CPU Temp" value={hwInfo.cpu_temp} unit="°C" />
                <Item text="GPU Usage" value={hwInfo.gpu_usage} unit="%" />
                <Item text="GPU Temp" value={hwInfo.gpu_temp} unit="°C" />
                <Item text="RAM Usage" value={hwInfo.ram_usage} unit="%" />
            </div>
        </>
    );
};

export default App;
