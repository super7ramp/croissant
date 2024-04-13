import Crossword from '@jaredreisinger/react-crossword';
import React, {useEffect, useRef, useState} from 'react';
import './App.css'

const INITIAL_GRID = "WORD\n....\n....\n....";

export default function App() {
    const crosswordRef = useRef(null)
    const [grid, setGrid] = useState(INITIAL_GRID)
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
    useEffect(() => {
        const rows = grid.split('\n')
        rows.forEach((row, rowIndex) => {
            Array.from(row).forEach((letter, columnIndex) => {
                const actualLetter = letter.replace('.', ' ')
                console.log("Setting letter '" + actualLetter + "' at row " + rowIndex + ", column " + columnIndex)
                try {
                    crosswordRef.current.setGuess(rowIndex, columnIndex, actualLetter)
                } catch (e) {
                    console.error("Error updating crossword component", e)
                }
            })
        })
    }, [grid])

    const startWorker = () => {
        const worker = new Worker(new URL('solver-worker.js', import.meta.url))
        worker.onmessage = ({data}) => {
            console.log("Received response from worker\n", data)
            switch (data.type) {
                case "solver-result":
                    setFillingInProgress(false)
                    setGrid(data.solution)
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
        console.log("Sending task to worker\n", grid)
        setFillingInProgress(true)
        worker.postMessage({action: "solve", grid: grid})
    }
    const onStopClick = () => {
        console.log("Forcibly stopping worker")
        worker.terminate()
        setFillingInProgress(false)
        setWorker(startWorker())
    }
    const onResetClick = () => setGrid(INITIAL_GRID)
    const onCellChange = (rowIndex, columnIndex, letter) => {
        const rows = grid.split('\n')
        const row = rows[rowIndex]
        const oldLetter = row.charAt(columnIndex)
        if (oldLetter !== letter) {
            console.log("Letter changed from '" + oldLetter + "' to '" + letter + "' at row " + rowIndex + ", column "
                + columnIndex)
            rows[rowIndex] = row.substring(0, columnIndex) + letter + row.substring(columnIndex + 1)
            const updatedGrid = rows.join('\n')
            setGrid(updatedGrid)
        }
    }

    return (
        <div className="App">
            <h1>This is 🥐</h1>
            <Crossword data={initialData()} onCellChange={onCellChange} ref={crosswordRef}/>
            <div className="button-container">
                {fillingInProgress
                    ? <button className="btn btn-warning btn-lg" onClick={onStopClick}>Stop filling</button>
                    : <button className="btn btn-primary btn-lg" onClick={onStartClick}>Auto-fill 🪄</button>}
                <button
                    className="btn btn-danger btn-lg"
                    disabled={fillingInProgress}
                    onClick={onResetClick}>
                    Reset
                </button>
            </div>
        </div>
    )
}

function initialData() {
    return {
        across: {
            1: {
                answer: '....',
                clue: '',
                row: 0,
                col: 0,
            },
            2: {
                answer: '....',
                clue: '',
                row: 1,
                col: 0,
            },
            3: {
                answer: '....',
                clue: '',
                row: 2,
                col: 0,
            },
            4: {
                answer: '....',
                clue: '',
                row: 3,
                col: 0,
            }
        },
        down: {
            I: {
                answer: 'ABCD',
                clue: '',
                row: 0,
                col: 0,
            },
            II: {
                answer: '....',
                clue: '',
                row: 0,
                col: 1,
            },
            III: {
                answer: '....',
                clue: '',
                row: 0,
                col: 2,
            },
            IV: {
                answer: '....',
                clue: '',
                row: 0,
                col: 3,
            },
        }
    };
}
