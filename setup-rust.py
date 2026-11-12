import os
from pathlib import Path
import stat
import tempfile
import urllib.request


def log(msg):
    # the "\033[93m" and "\033[0m" are ANSI escape codes for yellow text
    print(f"\033[93m[CS242] {msg}\033[0m")


HOME = Path.home()

# check if rustup is already installed
rustup = os.path.join(HOME, ".cargo", "bin", "rustup")
if os.path.exists(rustup):
    log(f'rustup is already installed in "{rustup}"!')
    exit(0)

with tempfile.NamedTemporaryFile(prefix="rustup-init-") as file:
    # close the file so that urlretrieve can overwrite it
    file.file.close()
    name = file.name

    # download rustup-init
    log("Downloading rustup-init...")
    urllib.request.urlretrieve(
        "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init",
        name,
    )

    # make rustup-init executable
    st = os.stat(name)
    os.chmod(name, st.st_mode | stat.S_IEXEC)

    # run rustup-init
    log("Running rustup-init...")
    os.environ["RUSTUP_INIT_SKIP_PATH_CHECK"] = "yes"
    os.system(name + " -y --no-modify-path --default-toolchain none")

    # configure rustup
    log("Configuring rustup...")
    os.system(rustup + " default 1.73.0")

    log("Done!")
