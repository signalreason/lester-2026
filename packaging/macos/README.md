# macOS Packaging Checklist

## Bundle layout (draft)
- `Lester.app/Contents/MacOS/browserd`
- `Lester.app/Contents/MacOS/llm-worker`
- `Lester.app/Contents/Resources/ui/` (Vite build output)
- `Lester.app/Contents/Info.plist`

## Signing
- Use a Developer ID Application certificate.
- Sign both binaries and the app bundle.

## Notarization
- Zip the app bundle.
- `xcrun notarytool submit Lester.zip --keychain-profile <profile> --wait`
- `xcrun stapler staple Lester.app`

## Notes
- Store the SQLite DB in `~/Library/Application Support/Lester`.
- Provide a first-run migration to move an existing `lester.db`.
