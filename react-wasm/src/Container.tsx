import {useContext, useEffect, useRef, useState} from "react";
import Canvas from "./Canvas";

import init, {solve} from "rust-tsumekomi";

type CanvasState = {
    squares: Array<Array<number>> | null
    width: number
}

type DataSet = {
    squares: Array<Array<number>> | null
    width: number
}

const generate_random_dataset = (num: number, width: number) => {
    let sq = [];

    for (let i = 0; i < num; i++) {
        sq.push([
            i,
            Math.floor(Math.random() * 100) + 1,
            Math.floor(Math.random() * 80) + 1,
        ])
    }
    return {
        squares: sq,
        width: width
    }

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
    const [dataset, setDataSet] = useState<DataSet>({squares: [], width: 200})
    // const [squares, setSquares] = useState<Array<Array<number>>>([])
    // const [initialized, setInitialized] = useState<boolean>(false)

    let update_random_dataset = () => {
        const dataset = generate_random_dataset(20, 200)
        setDataSet(dataset)

        init().then(() => {
            const res = solve(dataset);
            setCanvasState({squares: res.pos_list, width: canvasState.width})
        })
    }

    return (
        <>
            <button onClick={update_random_dataset}>random</button>
            <Canvas {...canvasState}/>
        </>
    )
}
export default Container