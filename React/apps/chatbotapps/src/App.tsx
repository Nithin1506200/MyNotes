import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import Draggable from "react-draggable";
function App() {
  const [count, setCount] = useState(0);
  return (
    <div className="App">
      <Draggable>
        <div>hello</div>
      </Draggable>
    </div>
  );
}

export default App;
