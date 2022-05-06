import {useContext, useEffect, useRef, useState} from "react";
import Canvas from "./Canvas";

import init, {solve} from "rust-tsumekomi";

const Container = () => {
    const [squares, setSquares] = useState<Array<Array<number>>>([])
    const [initialized, setInitialized] = useState<boolean>(false)
    let width=400;

    let solve_random = () => {
        let sq = [];

        for (let i = 0; i < 20; i++) {
            sq.push([
                i,
                Math.floor(Math.random() * 100) + 1,
                Math.floor(Math.random() * 80) + 1,
            ])
        }
        const params = {
            squares: sq,
            width: width
        }
        init().then(() => {
            const res = solve(params);
            console.log(res.pos_list)
            setSquares(res.pos_list)
        })
    }

    useEffect(() => {

        if (!initialized) {
            solve_random()
            setInitialized(true)
        }
    })

    return (
        <>

            <Canvas width={width} squares={squares}/>
        </>
    )
}
export default Container