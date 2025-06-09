const path = require('node:path');
const concurrently = require('concurrently');

concurrently(
  [
    {
      command: 'RUST_LOG=debug cargo watch -x "run --bin engelsystem-rs-frontend" -i "*sqlite*"',
      name: 'UI',
      prefixColor: 'cyan',
      cwd: path.resolve(__dirname, 'engelsystem-rs-frontend')
    },
    {
      command: 'RUST_LOG=debug cargo watch -x "run --bin engelsystem-rs-api" -i "*sqlite*"',
      name: 'API',
      prefixColor: 'magenta'
    },
    {
      command: 'npx tailwindcss -i ./assets/css/base.css -o ./assets/css/base-gen.css --watch',
      name: 'CSS',
      prefixColor: 'yellow',
      cwd: path.resolve(__dirname, 'engelsystem-rs-frontend')
    }
  ],
  {
    killOthers: ['failure', 'success'],
    restartTries: 0
  }
).result.catch(() => process.exit(1));
