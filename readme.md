
# RustSeek

## Overview

This Rust program provides a command-line interface (CLI) for searching files within a specified directory. The program allows users to search for specific terms within files and can also search within ZIP archives if enabled. It utilizes multi-threading to efficiently traverse directories and process files in parallel, ensuring fast search results.

## Features

- **Interactive CLI:** User-friendly interface for setting search parameters and displaying results.
- **Directory Search:** Recursively searches through directories for files containing the specified search term.
- **ZIP File Search:** Option to include ZIP archives in the search.
- **Progress Indicator:** Displays progress during the search process. (BROKEN RIGHT NOW)
- **Search Summary:** Provides a summary of the search results, including the number of files processed, results found, and search duration.
- **Speed:** Achieved speeds of ~220,000 files/second on my 8 core i7-10700F

## Installation

1. **Clone the repository:**
    ```sh
    git clone https://github.com/hdunl/RustSeek.git
    cd RustSeek
    ```

2. **Build the project:**
    ```sh
    cargo build --release
    ```

3. **Run the executable:**
    ```sh
    ./target/release/RustSeek.exe
    ```

## Usage

After running the program, you will have an interactive menu to configure your search parameters.

### Menu Options

1. **Start Search:** Initiates the search based on the configured search term, directory, and ZIP search settings.
2. **Change Search Term:** Allows you to set or modify the search term.
3. **Change Directory:** Allows you to set or modify the directory to search in.
4. **Toggle ZIP Search:** Enables or disables searching within ZIP archives.
5. **Exit:** Exits the program.

### Example

```sh
RustSeek
-------------------------------------------
Current Settings:
📝 Search Term: Not set
📁 Search Directory: Not set
🗜️ Search ZIP files: Disabled

🚀 Choose an option:
  1. 🔎 Start Search
  2. ✏️ Change Search Term
  3. 📂 Change Directory
  4. 🔄 Toggle ZIP Search
  5. ❌ Exit
```
![image](https://github.com/hdunl/RustSeek/assets/54483523/2dbf5218-b296-4f5e-b724-bbcf9b3540d6)


Follow the prompts to configure your search parameters. Once you have set the search term and directory, select "Start Search" to begin the search process. The results will be displayed along with a progress indicator and search summary.

## Author

Hayden Dunlap

- Email: [hdunlap936@gmail.com](mailto:hdunlap936@gmail.com), [hrdunlap@valdosta.edu](mailto:hrdunlap@valdosta.edu)
- GitHub: [github.com/hdunl](https://github.com/hdunl)

---
