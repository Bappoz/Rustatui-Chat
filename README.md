# Rusty Chat - TUI Chat Application

Chat application with Rust using Ratatui for TUI interface.



### Initializing

#### 1️⃣ Inicie o Servidor

```bash
# Terminal 1 - Servidor
cargo run --bin chat-server
```

**You may see:**

```
INFO rusty_chat_server:  Rusty Chat Server started
INFO rusty_chat_server:  Listening on 0.0.0.0:4556
```

#### After Initialize the TUI client

```bash
# Terminal 2 - Cliente
cargo run --bin chat-tui

# Or from chat-tui directory
cd chat-tui
cargo run
```

## Using the TUI Client

### Connection Screen

1. Enter your username
2. Press `Tab` to navigate between fields
3. Press `Enter` on the "Connect" button to connect

### Chat Interface

**Keyboard Controls:**

- `i` - Enter editing mode (to type messages)
- `ESC` - Exit editing mode
- `Enter` - Send message (in editing mode)
- `n` - Next room
- `p` - Previous room
- `↑/↓` - Scroll messages
- `q` - Quit (in normal mode)
- `Ctrl+C` - Force quit (any time)

### Commands

While in chat, you can use these commands:

- `/join <room>` - Join a room
- `/rooms` - List available rooms
- `/users` - List users in current room
- `/whisper <user> <message>` - Send private message
- `/help` - Show help
- `/quit` - Disconnect

## Development

### Building

```bash
# Build everything
cargo build

# Build only server
cargo build --bin chat-server

# Build only TUI
cargo build --bin chat-tui
```

### Testing

```bash
# Run all tests
cargo test

# Test specific package
cargo test -p chat-core
cargo test -p chat-tui
```

## Logs

Server logs are sent to `stderr` and won't interfere with the TUI.

To see server logs:

```bash
cargo run --bin chat-server 2> server.log
```

##  Common Problems

### Erro: "Connection Refused" (10061)

**Cause:** Server not running

**Solution:** Try initializing the server (chat-server) first:

```bash
# Terminal 1
cargo run --bin chat-server

# Terminal 2 (depois que o servidor iniciar)
cargo run --bin chat-tui
```

### Port 4556 already in use

**Solution:** Kill the process which is using this port:

```powershell
netstat -ano | findstr 4556
taskkill /F /PID <PID>
```

### Other problemas
Try contacting me via github or my social medias and I can try helping you.

## Architecture

- **chat-core**: Pure server logic, no terminal output
- **chat-tui**: TUI client using Ratatui, connects to server via TCP
- Both use async/await with Tokio
- Messages are parsed from server responses
