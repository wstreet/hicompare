import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import AceDiff from 'ace-diff';
import 'ace-diff/dist/ace-diff.min.css';
import "./App.css";



function App() {
  useEffect(() => {
    var differ = new AceDiff({
      element: '.acediff',
      left: {
        content: 'your first file content here',
      },
      right: {
        content: 'your second file content here',
      },
    });
  }, []);
  return (
    <div className="container">
      <div className="acediff"></div>
    </div>
  );
}

export default App;
