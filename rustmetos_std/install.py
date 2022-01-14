import shutil
import os

shutil.rmtree(os.path.expanduser("~/.rustmetos"))
shutil.unpack_archive(
    "target/archives/standard.zip", os.path.expanduser("~/.rustmetos")
)
