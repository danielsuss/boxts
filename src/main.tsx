import React, { useState, useRef } from "react";
import ReactDOM from "react-dom/client";

function App() {
  const [text, setText] = useState("");
  const [cursorPos, setCursorPos] = useState(0);
  const [hasSelection, setHasSelection] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Text:", text);
    setText("");
    setCursorPos(0);
  };

  const updateCursorPos = () => {
    setTimeout(() => {
      if (inputRef.current) {
        const start = inputRef.current.selectionStart || 0;
        const end = inputRef.current.selectionEnd || 0;
        setCursorPos(start);
        setHasSelection(start !== end);
      }
    }, 0);
  };

  const measureTextWidth = (text: string, pos: number) => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    ctx.font = '16px Consolas, "Courier New", monospace';
    return ctx.measureText(text.slice(0, pos)).width;
  };

  const getCharWidth = () => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    ctx.font = '16px Consolas, "Courier New", monospace';
    return ctx.measureText("M").width; // Use 'M' as it's typically the widest character
  };

  const getFontHeight = () => {
    return 19;
  };

  return (
    <form
      onSubmit={handleSubmit}
      style={{
        background: "transparent",
        margin: 0,
        padding: 0,
        position: "relative",
      }}
    >
      <input
        ref={inputRef}
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onSelect={updateCursorPos}
        onKeyDown={updateCursorPos}
        onClick={updateCursorPos}
        placeholder="Enter text..."
        autoFocus
        style={{
          width: "400px",
          height: "50px",
          borderRadius: "25px",
          border: "2px solid #94a3b8",
          padding: "0 20px",
          fontSize: "16px",
          fontFamily: "Consolas, 'Courier New', monospace",
          outline: "none",
          backgroundColor: "#020817",
          color: "#94a3b8",
          caretColor: "transparent",
        }}
      />
      {!hasSelection && (
        <div
          style={{
            position: "absolute",
            left: `${20 + measureTextWidth(text, cursorPos) + 1}px`,
            top: "50%",
            transform: "translateY(calc(-50% + 1px))",
            width: `${getCharWidth()}px`,
            height: `${getFontHeight()}px`,
            backgroundColor: "#94a3b8",
            color: "#020817",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            fontSize: "16px",
            fontFamily: "Consolas, 'Courier New', monospace",
          }}
        >
          {text[cursorPos] || ""}
        </div>
      )}
    </form>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />
);
