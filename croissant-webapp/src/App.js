import Crossword from '@jaredreisinger/react-crossword';
import React, {Fragment, useRef, useState} from "react";
import './App.css'

function App() {
    const [grid, setGrid] = useState(initialData())
    const crosswordRef = useRef()
    return (
        <>
            <div className="App">
                <h1>This is 🥐</h1>
                <Crossword ref={crosswordRef} data={grid}/>
                <div className="button-container">
                    <button className="btn btn-primary btn-lg"
                            onClick={async () => setGrid(await filled(grid))}>
                        Auto-fill 🪄
                    </button>
                    <button className="btn btn-danger btn-lg" onClick={() => crosswordRef.current.reset()}>
                        Reset
                    </button>
                </div>
            </div>
        </>
    );
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
                answer: '....',
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

function filled({grid}) {
    console.log("Asynchronous solving", grid)
    return import("croissant-wasm")
        .then((wasm) => wasm.solve(solverInputFrom(grid)))
        .then(console.log)
        .then((solverOutput) => gridFrom(solverOutput))
        .catch(e => {
            console.error("Error calling solver:", e)
            return initialData()
        })
}

function solverInputFrom(grid) {
    // TODO implement
    return ""
}

function gridFrom(solverOutput) {
    // TODO implement
    return initialData()
}

export default App;