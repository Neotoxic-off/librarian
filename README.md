# 📂 Librarian

**Librarian** is a Rust-based **media malware scanner** that recursively scans directories for hidden threats in media files (MP4, MP3, AVI, MKV, DOC, PNG, etc.).

## 👓 Features

- ✅ Deep File Inspection – Verifies file headers to prevent polyglot attacks
- ✅ Entropy Analysis – Flags files with unusually high entropy (possible encryption or packing)
- ✅ Script Injection Detection – Scans raw bytes for malicious patterns
- ✅ Fast Recursive Scanning – Uses <code>walkdir</code> for deep directory analysis
- ✅ Multi-threaded Processing – Parallel file scanning for maximum performance
- ✅ CLI-based & Lightweight – Built with <code>clap</code> and <code>log</code> for efficiency

<img src="assets/icon.png" height="25%" width="25%"/>

## 🔧 Installation

Ensure you have Rust installed. Then, clone the repository and build the project:

```sh
# Clone the repository
git clone https://github.com/Neotoxic-off/librarian.git
cd librarian

# Build the project
cargo build --release
```

## 🛠️ Usage

Run the scanner by specifying a folder to scan:

```sh
./librarian --folder /path/to/scan
```

### Optional Arguments

```sh
--entropy-threshold <value>   # Set a custom entropy threshold (default: 8.0)
--threads <value>   # Set a custom threads count (default: 2)
```

Example:

```sh
./librarian --folder /media --entropy-threshold 8.0 --threads 4
```

## 📜 License

Librarian is licensed under the MIT License. See [LICENSE](LICENSE) for details.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE

---

👨‍💻 **Built with Rust for speed & security!**
