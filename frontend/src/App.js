import React, { useState, useEffect } from 'react';
import './App.css';


function Law({id, callback}) {
    return (<div> <input
                                    type="text"
              onChange={(e) => callback(e.target.value)}
            /> <br/> </div>)
}

const SoundLaws = ({ onChange, id }) => {
    const [inputs, setInputs] = useState({
        name: '',
        input1: '',
        input2: '',
        leftContext: '',
        rightContext: '',
        single: false,
    });

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        const newValue = type === 'checkbox' ? checked : value;
        const updatedInputs = { ...inputs, [name]: newValue };
        setInputs(updatedInputs);
        onChange(id, updatedInputs);
    };

    return (
        <div>
            <input
                type="text"
                name="name"
                value={inputs.name}
                onChange={handleChange}
                placeholder="Name"
            />
            <input
                type="text"
                name="input1"
                value={inputs.input1}
                onChange={handleChange}
                placeholder="Input 1"
            />
            <input
                type="text"
                name="input2"
                value={inputs.input2}
                onChange={handleChange}
                placeholder="Input 2"
            />
            <input
                type="text"
                name="leftContext"
                value={inputs.leftContext}
                onChange={handleChange}
                placeholder="Left Context"
            />
            <input
                type="text"
                name="rightContext"
                value={inputs.rightContext}
                onChange={handleChange}
                placeholder="Right Context"
            />
            <label>
                Single:
                <input
                    type="checkbox"
                    name="single"
                    checked={inputs.single}
                    onChange={handleChange}
                />
            </label>
        </div>
    );
};

function App() {
    const [soundLawsList, setSoundLawsList] = useState([{ id: 1, inputs: {} }]);

    const handleAddSoundLaw = () => {
        const newId = soundLawsList.length + 1;
        setSoundLawsList([...soundLawsList, { id: newId, inputs: {} }]);
    };

    const handleInputChange = (id, updatedInputs) => {
        const updatedList = soundLawsList.map(item =>
            item.id === id ? { ...item, inputs: updatedInputs } : item
        );
        setSoundLawsList(updatedList);
    };

    const handleSubmit = async () => {
        try {
            // const response = await axios.post('YOUR_API_ENDPOINT', soundLawsList.map(item => item.inputs));
            // console.log(response.data);
        } catch (error) {
            console.error('There was an error sending the data!', error);
        }
    };

    return (
        <div className="App">
            {soundLawsList.map(({ id, inputs }) => (
                <SoundLaws key={id} id={id} onChange={handleInputChange} />
            ))}
            <button onClick={handleAddSoundLaw}>Add SoundLaw</button>
            <button onClick={handleSubmit}>Submit</button>
        </div>
    );
}

export default App;
