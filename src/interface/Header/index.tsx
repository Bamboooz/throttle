import React from "react";

const Header: React.FC = () => {
    return (
        <>
            <div className="w-full h-[24px] bg-header flex items-center justify-center shrink-0">
                <h1 className="text-neutral-100 text-[10px]">throttle (Alt+T)</h1>
            </div>
        </>
    );
};

export default Header;
