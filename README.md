 # monitor_rdp

**monitor_rdp** is a simple tool designed for company environments, enabling teams to share their Remote Desktop Protocol (RDP) connection information. Its primary goal is to prevent users from being disconnected from remote Windows computers by other teammates and provide a nice user history.


## Features

- **Rust-based CLI**: A command-line interface for client interactions.
- **C Integration**: An intuitive user interface built with C.
- **Python Server**: Handles database interactions and provides necessary APIs.

## Installation

Follow these steps to install the project:
### Prerequisites

Ensure you have **CMake** and **Raylib** installed on your machine. If needed, adjust the paths in the `CMakeLists.txt` file located in the `c_ui` directory.


1. **Clone the repository**:
   ```bash
   git clone https://github.com/d-shiri/rdp_monitor
   ```
2. Run `build.ps1` and it will build and copy everything to `output_bins` to be used.


## Usage
Before running the application, ensure you have a .env file in the same directory as nct.exe.

- Refer to env_example for guidance on the format of the .env file.
See `env_example` to see the `.env` format.
To see how to use the tool, execute:
```bash
nct.exe --help
# or 
nct.exe -h
```

