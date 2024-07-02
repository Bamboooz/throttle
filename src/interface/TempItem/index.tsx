import React from "react";
import { cn } from "../../utils/cn";

interface TempItemProps {
    text: string;
    value: number;
}

const TempItem: React.FC<TempItemProps> = ({ text, value }) => {
    const color = value < 33.33 ? "text-green-600" : (value < 66.66 ? "text-yellow-500" : "text-red-600");

    return (
        <>
            <div className="w-full h-full flex items-center justify-between px-2">
                <p className="text-neutral-100 text-[9px]">{text}</p>

                <p className={cn("text-[9px] font-bold", color)}>
                    {value}
                    <span className="text-neutral-500 font-black"> °C</span>
                </p>
            </div>
        </>
    );
};

export default TempItem;
