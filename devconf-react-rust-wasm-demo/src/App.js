import init, { add, concat_names_from_js, concat_names_from_rust } from "rust-wasm";
import React, { useState, useEffect } from 'react';

function App() {
  const [sum, setSum] = useState(0);
  const [benchmarkJs, setBenchmarkJS] = useState(0);
  useEffect(() => {
    async function initialize() {
      await init();
      setSum(add(3, 4));

      let users = [
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
        { first_name: "John", last_name: "Doe", id:"1"},
      ];

      const concat_names_in_js = (users) => {
        return users.reduce((prev, current) => {
          return `${prev.first_name}:${current.first_name}`
        }, "");
      }

      const benchmark_from_js = () => {
        const start = window.performance.now();
        concat_names_in_js(users);
        return window.performance.now() - start;
      }

      const benchmark_from_rust = () => {
        const start = window.performance.now();
        concat_names_from_rust();
        return window.performance.now() - start;
      }

      setBenchmarkJS(benchmark_from_rust() - benchmark_from_js());
    }
    initialize();
  }, [])
  return (
    <div className="App">
      <div>Output of 3 + 4 is { sum }</div>
      <div>Output of first js benchmark { benchmarkJs }</div>
    </div>
  );
}

export default App;
