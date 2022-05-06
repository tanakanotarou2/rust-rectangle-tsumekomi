import React, {useEffect, useState} from 'react';
import logo from './logo.svg';
import './App.css';
import init, { solve} from "rust-tsumekomi";
import Container from "./Container";

function App() {

    return (
        <div className="App">
            <header className="App-header">
                <Container/>
            </header>
        </div>
    );
}

export default App;
