import React, {useEffect, useState} from 'react';
import './App.css'
import Crossword from "./components/Crossword.jsx";
import {rowsFromSolverOutput, solverInputFromRows} from "./solver-io.js";

const INITIAL_ROWS = [
    ['', '', '', ''],
    ['', '', '', ''],
    ['', '', '', ''],
    ['', '', '', '']
];

export default function App() {
    const [rows, setRows] = useState(INITIAL_ROWS)
    const [fillingInProgress, setFillingInProgress] = useState(false)
    const [worker, setWorker] = useState(null)

    useEffect(() => {
        const worker = startWorker();
        setWorker(worker)
        return () => {
            console.log("Terminating worker")
            worker.terminate()
        }
    }, [])

    const startWorker = () => {
        const worker = new Worker(new URL('solver-worker.js', import.meta.url))
        worker.onmessage = ({data}) => {
            console.log("Received response from worker\n", data)
            switch (data.type) {
                case "solver-result":
                    setFillingInProgress(false)
                    setRows(rowsFromSolverOutput(data.solution))
                    break;
                case "solver-failed":
                    setFillingInProgress(false)
                    break;
                default:
                    console.warn("Received worker response with unknown type", data)
            }
        }
        return worker
    }

    const onStartClick = () => {
        console.log("Sending task to worker\n", rows)
        setFillingInProgress(true)
        worker.postMessage({action: "solve", grid: solverInputFromRows(rows)})
    }

    const onStopClick = () => {
        console.log("Forcibly stopping worker")
        worker.terminate()
        setFillingInProgress(false)
        setWorker(startWorker())
    }

    const onResetClick = () => {
        setRows(INITIAL_ROWS)
    }

    const onCellChange = (newValue, rowIndex, columnIndex) => {
        console.log("Cell change at row " + rowIndex + ", column " + columnIndex + " changed to " + newValue)
        const modifiedRows = [...rows]
        modifiedRows[rowIndex][columnIndex] = newValue
        setRows(modifiedRows)
    }

    return (<div className="App">
            <h1>This is 🥐</h1>
            <Crossword rows={rows} onCellChange={onCellChange}/>
            <div className="button-container">
                {fillingInProgress ?
                    <button className="btn btn-warning btn-lg" onClick={onStopClick}>Stop filling</button> :
                    <button className="btn btn-primary btn-lg" onClick={onStartClick}>Auto-fill 🪄</button>}
                <button
                    className="btn btn-danger btn-lg"
                    disabled={fillingInProgress}
                    onClick={onResetClick}>
                    Reset
                </button>
            </div>
        </div>)
}
