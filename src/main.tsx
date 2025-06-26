import React, { useState, useRef, useEffect } from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { register, unregister, isRegistered } from "@tauri-apps/plugin-global-shortcut";

function App() {
  const [text, setText] = useState("");
  const [cursorPos, setCursorPos] = useState(0);
  const [hasSelection, setHasSelection] = useState(false);
  const [isVisible, setIsVisible] = useState(true);
  const inputRef = useRef<HTMLInputElement>(null);

  // Color scheme
  const colors = {
    background: "#131313",
    text: "#c4c4c4",
    border: "#535353",
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
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
    ctx.font = "16px Consolas, 'Courier New', monospace";
    return ctx;
  };

  const measureTextWidth = (text: string, pos: number) => {
    return Math.round(getCanvasContext().measureText(text.slice(0, pos)).width);
  };

  const getCharWidth = () => {
    return Math.round(getCanvasContext().measureText("A").width);
  };

  // Register global shortcut
  useEffect(() => {
    const shortcut = "Alt+Enter";
    
    const setupGlobalShortcut = async () => {
      try {
        // Unregister if already registered (for hot reloads)
        if (await isRegistered(shortcut)) {
          await unregister(shortcut);
        }
        
        await register(shortcut, (event) => {
          if (event.state === 'Pressed') {
            setIsVisible(prev => !prev);
          }
        });
        
      } catch (error) {
        console.error("Failed to register global shortcut:", error);
      }
    };
    
    setupGlobalShortcut();

    return () => {
      unregister(shortcut).catch(console.error);
    };
  }, []);

  // Handle window visibility changes
  useEffect(() => {
    const window = getCurrentWindow();
    
    const updateWindowVisibility = async () => {
      try {
        if (isVisible) {
          await window.show();
          await window.setFocus();
        } else {
          await window.hide();
        }
      } catch (error) {
        console.error("Failed to update window visibility:", error);
      }
    };
    
    updateWindowVisibility();
  }, [isVisible]);

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
            if (
              e.key === "ArrowLeft" ||
              e.key === "ArrowRight" ||
              e.key === "Home" ||
              e.key === "End"
            ) {
              updateCursorPos();
            }
            if (e.ctrlKey && e.key === "a") {
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
            borderRadius: "4px",
            border: `1px solid ${colors.border}`,
            padding: "0 10px",
            fontSize: "16px",
            fontFamily: "Consolas, 'Courier New', monospace",
            outline: "none",
            backgroundColor: `${colors.background}80`,
            color: colors.text,
            caretColor: "transparent",
          }}
        />
        {!hasSelection && (
          <div
            style={{
              position: "absolute",
              left: `${
                Math.round(
                  10 +
                    measureTextWidth(text, cursorPos) -
                    (inputRef.current?.scrollLeft || 0)
                ) + 1
              }px`,
              top: "9px",
              width: `${getCharWidth()}px`,
              height: "19px",
              backgroundColor: colors.text,
              color: colors.background,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              fontSize: "16px",
              fontFamily: "Consolas, 'Courier New', monospace",
              lineHeight: "19px",
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
