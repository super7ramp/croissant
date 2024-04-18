import {rowsFromSolverOutput, solverInputFromRows} from "./solver-io.js";
import {expect, test} from 'vitest'

test('rows to solver input', () => {
    expect(solverInputFromRows([
        ['A', 'B', 'C', 'D'],
        ['E', 'F', 'G', 'H'],
        ['I', 'J', 'K', 'F'],
        ['L', 'M', 'N', 'O']]))
        .toEqual("ABCD\nEFGH\nIJKF\nLMNO");
})

test('rows with holes to solver input', () => {
    expect(solverInputFromRows([
        [' ', 'B', 'C', 'D'],
        ['E', 'F', 'G', 'H'],
        ['I', 'J', '', 'F'],
        ['L', '', 'N', 'O']]))
        .toEqual(".BCD\nEFGH\nIJ.F\nL.NO");
})

test('solver output to rows', () => {
    expect(rowsFromSolverOutput("ABCD\nEFGH\nIJKF\nLMNO"))
        .toEqual([
            ['A', 'B', 'C', 'D'],
            ['E', 'F', 'G', 'H'],
            ['I', 'J', 'K', 'F'],
            ['L', 'M', 'N', 'O']]);
});