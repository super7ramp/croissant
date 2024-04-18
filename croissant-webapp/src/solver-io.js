export function solverInputFromRows(rows) {
    return rows.map(row => row.map(cell => cell.trim() === '' ? '.' : cell).join('')).join('\n')
}

export function rowsFromSolverOutput(solverOutput) {
    return solverOutput.split('\n').map(row => row.split(''))
}
