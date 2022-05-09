import {useContext, useEffect, useRef, useState} from "react";
import Canvas from "./Canvas";
import './Container.css';

import init, {NF_solve, NFDH_solve, BLF_solve} from "rust-tsumekomi";
import {useMountEffect} from "./utils";

type ProblemResult = {
    squares: Array<Array<number>> | null
    width: number
    height: number
}

const generate_random_dataset = (num: number, width: number) => {
    let sq = [];

    for (let i = 0; i < num; i++) {
        sq.push([
            Math.floor(Math.random() * 100) + 1,
            Math.floor(Math.random() * 80) + 1,
        ])
    }
    return {
        squares: sq,
        width: width
    }

}

const parseInputText = (txt: string) => {
    const lines = txt.trim().split("\n")
    const [n, w] = lines[0].split(" ").map(v => parseInt(v))
    let squares = lines.slice(1).map(v => v.split(" ").map(v => parseInt(v)))
    return {
        n,
        width:w,
        squares
    }
}


const parseOutputText = (txt: string, inputData): ProblemResult => {
    const lines = txt.trim().split("\n")
    let xy = lines.map(v => v.split(" ").map(v => parseInt(v)))
    let resPos = []
    const squares = inputData.squares
    let max_height = 0
    for (let i = 0; i < inputData.n; i++) {
        const top = xy[i][1] + squares[i][1]
        if (max_height < top) max_height = top;
        resPos.push([
            xy[i][0],
            xy[i][1],
            squares[i][0],
            squares[i][1]
        ])
    }
    return {
        height: max_height,
        width: inputData.width,
        squares: resPos
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
    const [problemResult, setProblemResult] = useState<ProblemResult>({squares: null, width:0,height: 0})
    const [inputValue, setInputValue] = useState<string>("")
    const [outputValue, setOutputValue] = useState<string>("")

    const [selAlgo, setSelAlgo] = useState<string>("NF");

    const runSolver = () => {
        const inp = parseInputText(inputValue);
        const algo = selAlgo;
        const algoFnc = {
            BLF:BLF_solve,
            NFDH: NFDH_solve,
            NF: NF_solve,
        }
        init().then(() => {
            const res = algoFnc[algo](inp);
            console.log(res)
            const txt=res.pos_list.map(v=>v.join(" ")).join("\n")
            console.log(txt)
            setOutputValue(txt)
            bindProblemResult(inputValue,txt)
        }).catch((e) => {
            console.log(e)
            setOutputValue("invalid input")
        })
    }

    const changeAlgo = (name: string) => {
        setSelAlgo((v) => name)
    }

    let changeRandomDataset = () => {
        const dataset = generate_random_dataset(100, 400)
        let txt = `${dataset.squares.length} ${dataset.width}\n`;
        const squares = dataset.squares.map(s => `${s[0]} ${s[1]}`).join("\n");
        txt += squares + "\n"
        setInputValue(txt)
    }
    useMountEffect(() => {
        // runSolver(selAlgo, dataset)
    })

    const handleChangeInput = (event) => {
        setInputValue(event.target.value)
    }

    const bindProblemResult=(inputText,outputText)=>{
        try{
            const inp = parseInputText(inputValue)
            const parseData = parseOutputText(outputText, inp)
            console.log(parseData)
            setProblemResult(parseData)
        }catch (e){
            setProblemResult({height:0, width:0, squares:[]})
            console.log(e);
        }
    }

    const handleChangeOutput = (event) => {
        setOutputValue(event.target.value)
        let txt: string = event.target.value;
        txt = txt.trim();
        if (!txt || txt.length === 0) return

        bindProblemResult(inputValue,event.target.value)
    }

    const _label_style = {
        margin: "5px auto",
        backgroundColor: "white",
        color: "black",
        width: 400
        // width: ((dataset.width < 400) ? 400 : dataset.width) + "px",
    }
    const input_placeholder = "N W\nw1 h1\nw2 h2"
    const output_placeholder = "x1 y1\nx2 y2"

    return (
        <>
            <div className={"container"}>
                <div>
                    <div className="input">
                        <div>
                            <label htmlFor="inp">入力データ</label>
                            <textarea id="inp"
                                      name="inp"
                                      placeholder={input_placeholder}
                                      value={inputValue}
                                      onChange={handleChangeInput}
                            ></textarea>
                        </div>
                        <button onClick={changeRandomDataset}>ランダムデータ作成</button>
                    </div>
                    <div className="output">
                        <label htmlFor="output">出力</label>
                        <textarea id="output" name="output"
                                  placeholder={output_placeholder}
                                  value={outputValue}
                                  onChange={handleChangeOutput}
                        ></textarea>
                    </div>
                </div>
                <div className="solver">
                    <h3>ソルバー</h3>
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
                    <label>
                        <input
                            name="algo"
                            type="radio"
                            value="BLF"
                            checked={selAlgo === "BLF"}
                            onChange={() => changeAlgo("BLF")}
                        />
                        BLF法
                    </label>
                    <button onClick={runSolver}>ソルバー実行</button>
                </div>
            </div>
            {/* end left panel */}
            <div>
                <div style={_label_style}>height: {problemResult.height}</div>
                <Canvas {...problemResult}/>
            </div>
        </>
    )
}
export default Container