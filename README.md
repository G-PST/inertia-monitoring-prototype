# Inertia Monitoring Dashboard

Install `rust`, `node` and `tauri`

### Develop

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm install
npm run build
```

### Release

Change version number in all these files:

```bash
package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json
```

Commit and push to GitHub.
Then make new release on GitHub.
