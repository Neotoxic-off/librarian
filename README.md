# 📂 Librarian

**Librarian** is a Rust-based **media malware scanner** that recursively scans directories for hidden threats in media files (MP4, MP3, AVI, MKV, DOC, PNG, etc.).

<div style="display: flex; align-items: center;">
  <img src="assets/icon.png" height="25%" width="25%" style="margin-right: 15px;"/>
  <div>
    <h2>🚀 Features</h2>
    <ul>
      <li>✅ <b>Deep File Inspection</b> – Verifies file headers to prevent polyglot attacks</li>
      <li>✅ <b>Entropy Analysis</b> – Flags files with unusually high entropy (possible encryption or packing)</li>
      <li>✅ <b>Script Injection Detection</b> – Scans raw bytes for malicious patterns</li>
      <li>✅ <b>Fast Recursive Scanning</b> – Uses <code>walkdir</code> for deep directory analysis</li>
      <li>✅ <b>Multi-threaded Processing</b> – Parallel file scanning for maximum performance</li>
      <li>✅ <b>CLI-based & Lightweight</b> – Built with <code>clap</code> and <code>log</code> for efficiency</li>
    </ul>
  </div>
</div>

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
