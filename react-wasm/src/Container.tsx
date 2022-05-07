import {useContext, useEffect, useRef, useState} from "react";
import Canvas from "./Canvas";

import init, {NF_solve, NFDH_solve} from "rust-tsumekomi";
import {useMountEffect} from "./utils";

type problemResult = {
    squares: Array<Array<number>> | null
    height: number
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
    const [problemResult, setProblemResult] = useState<problemResult>({squares: null, height: 0})
    const [dataset, setDataSet] = useState<DataSet>(generate_random_dataset(100, 400))

    const [selAlgo, setSelAlgo] = useState<string>("NF");
    // const [squares, setSquares] = useState<Array<Array<number>>>([])
    // const [initialized, setInitialized] = useState<boolean>(false)

    const reSolve = (algo: string, dataset: DataSet) => {
        let fnc;
        if (algo === "NFDH") {
            fnc = NFDH_solve
        } else {
            fnc = NF_solve
        }
        init().then(() => {
            const res = fnc(dataset);
            setProblemResult({
                squares: res.pos_list,
                height: res.height
            })
        })
    }

    const changeAlgo = (name: string) => {
        setSelAlgo((v) => name)
        reSolve(name, dataset);
    }

    let changeRandomDataset = () => {
        const dataset = generate_random_dataset(100, 400)
        setDataSet(() => dataset)
        reSolve(selAlgo, dataset);
    }
    useMountEffect(() => {
        reSolve(selAlgo, dataset)
    })

    const _label_style={
        margin:"5px auto",
        backgroundColor: "white",
        color: "black",
        width: ((dataset.width <400)?400:dataset.width)+"px",
    }
    return (
        <>
            <div>
                <label>
                    <input
                        name="algo"
                        type="radio"
                        onChange={() => changeAlgo("NF")}
                        value="NF"
                        checked={selAlgo === "NF"}
                    />
                    NF法
                </label>
                <label>
                    <input
                        name="algo"
                        type="radio"
                        value="NFDH"
                        checked={selAlgo === "NFDH"}
                        onChange={() => changeAlgo("NFDH")}
                    />
                    NFDH法
                </label>
                <button onClick={changeRandomDataset}>ランダムデータ更新</button>
            </div>

            <div>
                <div style={_label_style}>height: {problemResult.height}</div>

                <Canvas width={dataset.width} {...problemResult}/>
            </div>
        </>
    )
}
export default Container