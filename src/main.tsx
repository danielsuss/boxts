import React, { useState } from "react";
import ReactDOM from "react-dom/client";

function App() {
  const [text, setText] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Text:", text);
    setText("");
  };

  return (
    <form onSubmit={handleSubmit} style={{ background: "transparent", margin: 0, padding: 0 }}>
      <input
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Enter text..."
        autoFocus
        style={{
          width: "400px",
          height: "50px",
          borderRadius: "25px",
          border: "2px solid #94a3b8",
          padding: "0 20px",
          fontSize: "16px",
          outline: "none",
          backgroundColor: "#020817",
          color: "white",
        }}
      />
    </form>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />
);
