export default function Crossword({rows, onCellChange}) {

    const Cell = ({value, rowIndex, columnIndex}) => {
        return <td>
            <input
                value={value}
                onChange={(event) => onCellChange(event.target.value.toUpperCase(), rowIndex, columnIndex)}
            />
        </td>
    }

    const Row = ({cells, rowIndex}) => {
        return <tr>
            {cells.map((cell, columnIndex) =>
                <Cell
                    key={rowIndex.toString() + "," + columnIndex.toString()}
                    value={cell}
                    rowIndex={rowIndex}
                    columnIndex={columnIndex}
                />
            )}
        </tr>
    }

    return <table>
        <tbody>
        {rows.map((row, rowIndex) => <Row key={rowIndex} cells={row} rowIndex={rowIndex}/>)}
        </tbody>
    </table>
}
