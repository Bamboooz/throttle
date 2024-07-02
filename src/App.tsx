import React from "react";

import Header from "./interface/Header";
import UsageItem from "./interface/UsageItem";
import TempItem from "./interface/TempItem";

const App: React.FC = () => {
    return (
        <>
            <div className="w-screen h-screen pt-1 pr-1">
                <div className="w-full h-full flex flex-col bg-main border border-neutral-900 rounded-md">
                    <Header />

                    <UsageItem text="CPU Usage" value={19.7} />
                    <TempItem text="CPU Temp" value={19.7} />
                    <UsageItem text="GPU Usage" value={63.1} />
                    <TempItem text="GPU Usage" value={19.7} />
                    <UsageItem text="RAM Usage" value={67.2} />
                </div>
            </div>
        </>
    );
};

export default App;
