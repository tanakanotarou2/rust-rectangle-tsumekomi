import {useContext, useEffect, useRef, useState} from "react";
import Canvas from "./Canvas";

import init, {solve} from "rust-tsumekomi";

type CanvasState = {
    squares: Array<Array<number>> | null
    width: number
}

/**
 * 全体的な管理を行う
 *
 * このコンポーネントで扱う情報
 * - 現在の表示する盤面の状態
 * - 使用するテストデータなど
 * などなど
 */
const Container = () => {
    const [canvasState, setCanvasState] = useState<CanvasState>({squares: null, width: 200})
    // const [squares, setSquares] = useState<Array<Array<number>>>([])
    // const [initialized, setInitialized] = useState<boolean>(false)

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
            width: canvasState.width
        }
        init().then(() => {
            const res = solve(params);
            setCanvasState({squares:res.pos_list,width:canvasState.width})
        })
    }

    return (
        <>
            <button onClick={solve_random}>random</button>
            <Canvas {...canvasState}/>
        </>
    )
}
export default Container