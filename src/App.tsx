import React, { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [value, setValue] = useState("");
  // const [focused, setFocused] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null); // Reference to the input field

  // await register('CommandOrControl+Space', async () => {
    
  //   if (focused == true) {
  //     console.log('minimizing window');
  //     await getCurrentWindow().minimize();
  //     setFocused(false);
  //   } else {
  //     console.log('focusing window');
  //     await getCurrentWindow().setFocus();
  //     setFocused(true);
  // }});

  // Focus input when window is focused
  useEffect(() => {
    const handleWindowFocus = () => {
      if (inputRef.current) {
        inputRef.current.focus();
      }
    };

    window.addEventListener("focus", handleWindowFocus);
    return () => window.removeEventListener("focus", handleWindowFocus);
  }, []);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    console.log(value);
    invoke("input", { text: value });
    setValue("");
  };

  return (
    <form onSubmit={handleSubmit} className="input-form">
      <input
        ref={inputRef} // Attach the ref to the input field
        className="text-box"
        type="text"
        value={value}
        onChange={(e) => setValue(e.target.value)}
      />
    </form>
  );
}

export default App;
