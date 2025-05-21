default: dev

# watch cargo run with watchexec
dev:
  watchexec -e rs -r cargo run
