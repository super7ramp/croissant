onmessage = function({data}) {
    console.log("Received message\n", data)
    const action = data.action;
    switch (action) {
        case "solve":
            onSolve(data.grid)
            break
        default:
            console.warn("Received message with unknown action", data)
    }
}

const onSolve = (grid) => {
    import('croissant-wasm').then(wasm => {
        console.log("Solving\n", grid)
        const result = wasm.solve(grid)
        console.log("Solved", grid)
        console.log("Result", result)
        postMessage({type: "solver-result", solution: result})
    }).catch(e => {
        postMessage({type: "solver-failed", error: e})
    })
}
