# PrettyEVMadress

Generate custom Ethereum addresses with specific prefixes and suffixes using multi-threaded vanity address generation.

📱 **Community Links**
- Telegram Channel: [MimbleWimbleLAB](https://t.me/MimbleWimbleLAB)
- Telegram Chat: [MimbleWimbleLAB_chat](https://t.me/MimbleWimbleLAB_chat)

## Features

- 🚀 Multi-threaded address generation for maximum performance
- ⚙️ Configurable prefix and suffix patterns (NOT CHECKSUMED!)
- 📊 Real-time progress monitoring with speed and time estimates
- ✅ EIP-55 compliant checksum addresses
- 💾 Automatic saving of address-private key pairs
- 🔄 Batch generation support

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- Git

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/smeshny/PrettyEVMadress.git
   cd PrettyEVMadress
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   ./target/release/PrettyEVMadress
   ```

## Configuration

### `settings.toml`

All application settings are configured through the `settings.toml` file. Below is an example configuration:

```toml
# Example settings.toml
prefix = "777"
suffix = "777"
threads = 8
```

- `prefix`: The desired prefix for the Ethereum address.
- `suffix`: The desired suffix for the Ethereum address.
- `threads`: Number of threads to use for address generation.

## Usage

### Running the Application

Simply run the application after configuring your settings in `settings.toml`:

```bash
./target/release/PrettyEVMadress
```

### Example

Make sure your `settings.toml` file contains the desired configuration:

```toml
prefix = "abc"
suffix = "xyz"
threads = 4
```

### Data Storage

Generated addresses and their corresponding private keys are automatically saved in the `address_key_pair.txt` file in the project's root directory. Each line contains an entry in the following format:
```
Address: 0x777...777 | Private Key: 0x1234...5678
```

⚠️ Important: Keep your `address_key_pair.txt` file secure as it contains sensitive private key information!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
