import React, { useState, useRef, useEffect } from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/plugin-global-shortcut";

function App() {
  const [text, setText] = useState("");
  const [cursorPos, setCursorPos] = useState(0);
  const [hasSelection, setHasSelection] = useState(false);
  const [availableCommands, setAvailableCommands] = useState<string[]>([]);
  const [suggestion, setSuggestion] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  // Color scheme
  const colors = {
    background: "#131313",
    text: "#c4c4c4",
    border: "#535353",
    suggestion: "#79f079",
    error: "#FF6B6B",
  };

  const updateSuggestion = (inputText: string) => {
    if (inputText.startsWith("/") && inputText.length > 1) {
      const command = inputText.slice(1);
      const match = availableCommands.find((cmd) => cmd.startsWith(command));
      setSuggestion(match ? match.slice(command.length) : "");
    } else {
      setSuggestion("");
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      await invoke("process_input", { text });
    } catch (error) {
      console.error("Error processing input:", error);
    }

    setText("");
    setCursorPos(0);
    setSuggestion("");
    // Reset input scroll position
    if (inputRef.current) {
      inputRef.current.scrollLeft = 0;
    }
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

  // Fetch available commands on startup
  useEffect(() => {
    invoke<string[]>("get_available_commands")
      .then(setAvailableCommands)
      .catch(console.error);
  }, []);

  // Register global shortcut and window focus handling
  useEffect(() => {
    const shortcut = "Alt+Enter";
    const window = getCurrentWindow();

    const setupGlobalShortcut = async () => {
      try {
        // Unregister if already registered (for hot reloads)
        if (await isRegistered(shortcut)) {
          await unregister(shortcut);
        }

        await register(shortcut, async (event) => {
          if (event.state === "Pressed") {
            const isVisible = await window.isVisible();
            if (isVisible) {
              await window.hide();
            } else {
              await window.show();
              await window.setFocus();
            }
          }
        });
      } catch (error) {
        console.error("Failed to register global shortcut:", error);
      }
    };

    const setupFocusHandler = async () => {
      try {
        // Hide window when it loses focus
        const unlistenBlur = await window.onFocusChanged(
          ({ payload: focused }) => {
            if (!focused) {
              window.hide().catch(console.error);
            }
          }
        );

        return unlistenBlur;
      } catch (error) {
        console.error("Failed to setup focus handler:", error);
        return () => {};
      }
    };

    setupGlobalShortcut();
    let unlistenFocus: (() => void) | undefined;

    setupFocusHandler().then((unlisten) => {
      unlistenFocus = unlisten;
    });

    return () => {
      unregister(shortcut).catch(console.error);
      if (unlistenFocus) {
        unlistenFocus();
      }
    };
  }, []);

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
          type="text"
          value=""
          readOnly
          placeholder=""
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
            color: "transparent",
            caretColor: "transparent",
            position: "relative",
            zIndex: 0,
            pointerEvents: "none",
          }}
        />
        <input
          ref={inputRef}
          type="text"
          value={text}
          onChange={(e) => {
            setText(e.target.value);
            updateCursorPos();
            updateSuggestion(e.target.value);
          }}
          onSelect={updateCursorPos}
          onKeyDown={(e) => {
            if (e.key === "Tab") {
              e.preventDefault();
              if (suggestion) {
                const fullCommand = `/${text.slice(1)}${suggestion}`;
                setText(fullCommand);
                setSuggestion("");
                updateSuggestion(fullCommand);
                setTimeout(() => {
                  if (inputRef.current) {
                    const newPos = fullCommand.length;
                    inputRef.current.setSelectionRange(newPos, newPos);
                    setCursorPos(newPos);
                  }
                }, 0);
              }
            } else if (
              e.key === "ArrowLeft" ||
              e.key === "ArrowRight" ||
              e.key === "Home" ||
              e.key === "End"
            ) {
              updateCursorPos();
            } else if (e.ctrlKey && e.key === "a") {
              updateCursorPos();
            }
          }}
          onClick={updateCursorPos}
          placeholder=""
          autoFocus
          spellCheck={false}
          onContextMenu={(e) => e.preventDefault()}
          style={{
            position: "absolute",
            left: "0px",
            top: "0px",
            width: "400px",
            height: "35px",
            borderRadius: "4px",
            border: "none",
            padding: "0 10px",
            fontSize: "16px",
            fontFamily: "Consolas, 'Courier New', monospace",
            outline: "none",
            backgroundColor: "transparent",
            color: (() => {
              if (text.startsWith("/") && text.length > 1) {
                // Check for multiple slashes
                const hasMultipleSlashes = text.slice(1).includes('/');
                if (hasMultipleSlashes) {
                  return colors.error;
                }
                
                const command = text.slice(1).split(' ')[0];
                const isValidCommand = availableCommands.includes(command);
                const showError = !suggestion && !isValidCommand;
                return showError ? colors.error : colors.text;
              }
              return colors.text;
            })(),
            caretColor: "transparent",
            zIndex: 2,
          }}
        />
        {suggestion && (
          <div
            style={{
              position: "absolute",
              left: `${10 + measureTextWidth(text, text.length)}px`,
              top: "8px",
              fontSize: "16px",
              fontFamily: "Consolas, 'Courier New', monospace",
              color: colors.suggestion,
              pointerEvents: "none",
              zIndex: 1,
              lineHeight: "19px",
            }}
          >
            {suggestion}
          </div>
        )}
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
              top: "8px",
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
              zIndex: 3,
            }}
          >
            {suggestion && cursorPos === text.length
              ? suggestion[0]
              : text[cursorPos] || ""}
          </div>
        )}
      </form>
    </>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />
);
