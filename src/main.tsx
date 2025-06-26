import React, { useState, useRef } from "react";
import ReactDOM from "react-dom/client";

function App() {
  const [text, setText] = useState("");
  const [cursorPos, setCursorPos] = useState(0);
  const [hasSelection, setHasSelection] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  // Color scheme
  const colors = {
    background: "#131313",
    text: "#c4c4c4",
    border: "#535353",
  };

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

  const getCanvasContext = () => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    ctx.font = '16px Consolas, "Courier New", monospace';
    return ctx;
  };

  const measureTextWidth = (text: string, pos: number) => {
    return getCanvasContext().measureText(text.slice(0, pos)).width;
  };

  const getCharWidth = () => {
    return getCanvasContext().measureText("M").width;
  };

  const fontHeight = 19;

  return (
    <>
      <style>
        {`
          input::selection {
            background-color: ${colors.text};
            color: ${colors.background};
          }
        `}
      </style>
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
        onChange={(e) => {
          setText(e.target.value);
          updateCursorPos();
        }}
        onSelect={updateCursorPos}
        onKeyDown={(e) => {
          if (e.key === 'ArrowLeft' || e.key === 'ArrowRight' || e.key === 'Home' || e.key === 'End') {
            updateCursorPos();
          }
        }}
        onClick={updateCursorPos}
        placeholder=""
        autoFocus
        spellCheck={false}
        style={{
          width: "400px",
          height: "35px",
          borderRadius: "3px",
          border: `1px solid ${colors.border}`,
          padding: "0 10px",
          fontSize: "16px",
          fontFamily: "Consolas, 'Courier New', monospace",
          outline: "none",
          backgroundColor: colors.background,
          color: colors.text,
          caretColor: "transparent",
        }}
      />
      {!hasSelection && (
        <div
          style={{
            position: "absolute",
            left: `${
              10 +
              measureTextWidth(text, cursorPos) -
              (inputRef.current?.scrollLeft || 0)
            }px`,
            top: "50%",
            transform: "translateY(-50%)",
            lineHeight: "1",
            width: `${getCharWidth()}px`,
            height: `${fontHeight}px`,
            backgroundColor: colors.text,
            color: colors.background,
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
    </>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />
);
