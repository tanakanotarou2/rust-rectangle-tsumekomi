import {useContext, useEffect, useRef, useState} from "react";
import {useMountEffect} from "./utils";

type Props = {
    squares: Array<Array<number>> | null
    width: number
    height: number
}

let clearCanvas = (context, width, height) => {
    // fill background
    context.beginPath();
    context.fillStyle = 'rgb( 255, 255, 255)';
    context.fillRect(0, 0, width, height);
    context.save();
}


const Canvas = (props: Props) => {
    const canvasRef = useRef(null);
    let canvasHeight, canvasWidth;

    let drawRect = (ctx, attr) => {
        const [no, x0, y0, width, height] = attr;

        ctx.strokeStyle = 'rgb( 0, 0, 0)';
        ctx.strokeRect(x0, canvasHeight - y0 - height, width, height)

        ctx.fillStyle = 'rgb( 255, 0, 0)'
        ctx.fillText(no + "", x0, canvasHeight - y0 - height + 10)
    }
    const drawSquares = () => {
        if (!canvasRef.current) return;
        const canvas: any = canvasRef.current;
        const ctx = canvas.getContext('2d');
        clearCanvas(ctx, canvas.width, canvas.height)
        canvasWidth = canvas.width;
        canvasHeight = canvas.height;

        if (!!props.squares && props.squares.length > 0) {
            ctx.font = "12px serif";
            props.squares.forEach(v => drawRect(ctx, v))
        }
    }
    useEffect(()=>{
        // TODO: (確認)キャンバスはロード完了してから描画するもよう。
        drawSquares()
    })


    // drawSquares()

    return (
        <>
            <canvas ref={canvasRef} width={props.width || 400} height={(props.height||400)+10}></canvas>
        </>
    )
}
export default Canvas
