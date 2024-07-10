import React from "react";

interface ItemProps {
    text: string;
    value: number;
    unit: string;
}

const Item: React.FC<ItemProps> = ({ text, value, unit }) => {
    return (
        <>
            <div className="w-full h-full bg-item odd:bg-item-odd flex items-center justify-between px-2 text-[10px]">
                <p className="text-neutral-300 text-[9px]">{text}</p>

                {value !== -1.0
                    ?<div className="flex justify-end font-bold">
                        <p className="text-usage">{value}</p>

                        <div className="flex justify-end w-[16px]">
                            <p className="text-neutral-500">{unit}</p>
                        </div>
                    </div>
                    : <p className="text-neutral-500 font-bold">N/A</p>
                }
            </div>
        </>
    );
};

export default Item;
