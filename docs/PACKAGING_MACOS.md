# Packaging for macOS

## Strategy
- Bundle the Rust daemon and worker inside a desktop shell (Tauri or native).
- Serve the UI assets locally within the bundle.
- Provide an app-specific data directory for the SQLite database.

## Steps (draft)
1. Build Rust binaries in release mode.
2. Build UI assets with Vite.
3. Place binaries and UI assets in the app bundle layout.
4. Sign with a Developer ID certificate.
5. Notarize with `notarytool` and staple.

See `packaging/macos/README.md` for a checklist.
