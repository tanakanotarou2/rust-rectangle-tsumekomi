import {useContext, useEffect, useRef, useState} from "react";

type Props = {
    squares: Array<Array<number>> | null
    width: number
    height: number
}


const Canvas = (props: Props) => {
    const canvasRef = useRef(null);
    let canvasHeight, canvasWidth;

    let clearCanvas = (context, width, height) => {
        // fill background
        context.beginPath();
        context.fillStyle = 'rgb( 255, 255, 255)';
        context.fillRect(0, 0, width, height);

        context.fillStyle = 'rgb( 50, 50, 50)';
        context.fillRect(props.width, 0, width, height);
        context.save();
    }

    let drawRect = (ctx, no, attr) => {
        const [x0, y0, width, height] = attr;

        ctx.fillStyle = 'rgb(158,168,216)'
        ctx.fillRect(x0, canvasHeight - y0 - height, width, height)
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
            props.squares.forEach((v, i) => drawRect(ctx, i, v))
        }
    }
    useEffect(() => {
        // TODO: (確認)キャンバスはロード完了してから描画するもよう。
        drawSquares()
    })
    return (
        <>
            <canvas ref={canvasRef} width={props.width || 400} height={(props.height || 400) + 10}></canvas>
        </>
    )
}
export default Canvas
