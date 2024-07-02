import React from "react";

import icon from "../../assets/icon.png";

const Header: React.FC = () => {
    return (
        <>
            <div className="w-full h-8 bg-header flex items-center justify-center gap-1 shrink-0">
                <img src={icon} className="h-[12px] w-[12px]" />

                <h1 className="text-neutral-100 text-[10px]">throttle (Alt+T)</h1>
            </div>
        </>
    );
};

export default Header;
