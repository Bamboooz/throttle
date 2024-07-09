import React from "react";

interface ItemProps {
    text: string;
    value: number;
    unit: string;
}

const Item: React.FC<ItemProps> = ({ text, value, unit }) => {
    return (
        <>
            <div className="w-full h-full bg-item odd:bg-item-odd flex items-center justify-between px-2">
                <p className="text-neutral-300 text-[9px]">{text}</p>

                <div className="flex text-[10px]">
                    <p className="text-usage font-bold">{value}</p>

                    <div className="w-[16px] flex justify-center">
                        <p className="text-neutral-500 font-black"> {unit}</p>
                    </div>
                </div>
            </div>
        </>
    );
};

export default Item;
