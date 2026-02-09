# Rusty Chat - TUI Chat Application

Chat application with Rust using Ratatui for TUI interface.

## Project Structure

```
chat-server-rust/
├── chat-core/          # Server-side logic
│   └── src/
│       ├── main.rs     # Server entry point
│       ├── lib.rs      # Library exports
│       ├── server/     # Server implementation
│       ├── client/     # Client management (server-side)
│       ├── message/    # Message types and processing
│       └── utils/      # Utilities
└── chat-tui/           # TUI Client
    └── src/
        ├── main.rs     # TUI entry point
        ├── app.rs      # Application logic
        ├── client/     # Server connection
        ├── state/      # Application state
        ├── view/       # UI components
        ├── event/      # Event handling
        └── input/      # Input handling
```

## 🚀 Quick Start

### Opção 1: Início Automático (Windows)

```batch
start.bat
```

Este script vai:

1. Compilar o projeto
2. Iniciar o servidor em segundo plano
3. Testar a conexão
4. Iniciar o cliente TUI

### Opção 2: Início Manual

#### 1️⃣ Inicie o Servidor

```bash
# Terminal 1 - Servidor
cargo run --bin chat-server
```

**Você deve ver:**

```
INFO rusty_chat_server: 🦀 Rusty Chat Server started
INFO rusty_chat_server: 📡 Listening on 0.0.0.0:4556
```

#### 2️⃣ Inicie o Cliente TUI

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

## ⚠️ Problemas Comuns

### Erro: "Conexão recusada" (10061)

**Causa:** Servidor não está rodando

**Solução:** Certifique-se de iniciar o servidor ANTES do cliente:

```bash
# Terminal 1
cargo run --bin chat-server

# Terminal 2 (depois que o servidor iniciar)
cargo run --bin chat-tui
```

### Porta 4556 já em uso

**Solução:** Mate o processo usando a porta:

```powershell
netstat -ano | findstr 4556
taskkill /F /PID <PID>
```

### Ver mais soluções

Consulte [TROUBLESHOOTING.md](TROUBLESHOOTING.md) para guia completo de resolução de problemas.

## Architecture

- **chat-core**: Pure server logic, no terminal output
- **chat-tui**: TUI client using Ratatui, connects to server via TCP
- Both use async/await with Tokio
- Messages are parsed from server responses
