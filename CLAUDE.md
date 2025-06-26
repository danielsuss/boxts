# Development Guidelines for Boxts

## Original Prompt
> "just a heads up on our development approach, we only add code when absolutely necessary, not adding any extra complexity where it's not needed. when implementing a new feature we discuss it until clear, and then i will give a go ahead for implementation. this minimalist and thorough approach will help us to write clean and maintainable code, whilst cutting down on complexity. similar to how with traditional coding you make sure one thing works at a time before moving on to the next thing."

# Development Guidelines for Boxts

## Core Development Approach

- **Minimalist**: Only add code when absolutely necessary - no extra complexity where it's not needed
- **Deliberate**: Discuss features thoroughly until clear before implementation 
- **Incremental**: Make sure one thing works at a time before moving on to the next
- **Clean**: Write maintainable code, avoid premature optimization
- **User-driven**: Wait for explicit go-ahead before implementing any new features

## Implementation Process

1. Discuss feature requirements and approach thoroughly
2. Get explicit approval from user before coding
3. Implement one working piece at a time
4. Test/verify functionality before moving to next component
5. Keep codebase lean and focused

## Development Notes

- User handles all npm/build commands - do not run npm commands via Bash tool

## Tauri v2 Documentation

⚠️ **Important**: Current model training data is limited to Tauri v1. When working with Tauri v2 features, always prompt the user to provide relevant Tauri v2 documentation links or code examples to ensure accurate implementation.

## Project Context

Boxts is a text-to-speech overlay application:
- Floating text input activated by Alt+Enter global hotkey
- Normal text → Python backend → RealtimeTTS with Coqui TTS
- `/commands` → handled in Rust layer for configuration
- Built with Tauri v2 + React TypeScript frontend, Python backend