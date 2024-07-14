import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
    const [columns, setColumns] = useState([['']]);
    const [saves, setSaves] = useState([]);
    const [currentSave, setCurrentSave] = useState(null);

    useEffect(() => {
        const fetchSaves = async () => {
            const response = await fetch('/saves');
            const data = await response.json();
            setSaves(data);
        };
        fetchSaves();
    }, []);

    const handleChange = (colIndex, rowIndex, value) => {
        const newColumns = columns.map((col, cIndex) =>
            col.map((row, rIndex) => (cIndex === colIndex && rIndex === rowIndex) ? value : row)
        );
        setColumns(newColumns);
    };

    const addColumn = () => {
        setColumns([...columns, ['']]);
    };

    const addRow = (colIndex) => {
        const newColumns = columns.map((col, index) =>
            index === colIndex ? [...col, ''] : col
        );
        setColumns(newColumns);
    };

    const saveData = async () => {
        const response = await fetch('/save', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ columns }),
        });
        const data = await response.json();
        setSaves([...saves, { columns }]);
        setColumns([['']]);
    };

    const loadData = async (index) => {
        const response = await fetch(`/load/${index}`);
        const data = await response.json();
        setCurrentSave(index);
        setColumns(data.columns);
    };

    return (
        <div className="App">
            <header className="App-header">
                <div className="input-columns">
                    {columns.map((column, colIndex) => (
                        <div key={colIndex} className="column">
                            {column.map((value, rowIndex) => (
                                <input
                                    key={rowIndex}
                                    type="text"
                                    value={value}
                                    onChange={(e) => handleChange(colIndex, rowIndex, e.target.value)}
                                    placeholder={`Column ${colIndex + 1}, Row ${rowIndex + 1}`}
                                />
                            ))}
                            <button onClick={() => addRow(colIndex)}>Add Row</button>
                        </div>
                    ))}
                </div>
                <button onClick={addColumn}>Add Column</button>
                <button onClick={saveData}>Save</button>
                <div className="saves-list">
                    <h3>Saved Entries</h3>
                    {saves.map((save, index) => (
                        <div key={index}>
                            <button onClick={() => loadData(index)}>
                                Load Save {index + 1}
                            </button>
                        </div>
                    ))}
                </div>
            </header>
        </div>
    );
}

export default App;
