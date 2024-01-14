# RUSTBU

how 2 run:

<details>
  <summary>1. Rust (duh)</summary>
    # Windows:
    Install Rustup using this link
    https://www.rust-lang.org/tools/install

    If you're on macOS / Windows
    Run `curl https://sh.rustup.rs -sSf | sh`

</details>

<details>
  <summary>2. Get sqlite3</summary>

    For windows:

    1. Download precompiled binaries for windows here:
    https://www.sqlite.org/download.html

    2. Extract them somewhere. Like C:\Program Files (x86)\sqlite3

    3. Edit your environment variables
    * create a new variable named "sqlite3" and add C:\Program Files (x86)\sqlite3\sqlite3.exe 
    * find and edit the variable named "Path" and add C:\Program Files (x86)\sqlite3\

    4. ???

    5. Profit

    For MacOS/Linux:
    Idk man, do yay -S sqlite3 or something. 
    TODO: Update this ^

</details>

3. `cargo run`

idk I guess that's it?