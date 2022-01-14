#!/usr/bin/env python3

import glob
import os
import shutil
import platform

shutil.rmtree("target/archives")

print("Building Binaries")
os.system("cargo build --release")  # Build the file

print("Preparing binaries")
os.makedirs("target/archives/raw_archive/bin")

if platform.system() == "Windows":
    executables = glob.glob("*.exe", root_dir="target/release")
else:
    executables = []
    for x in os.listdir("target/release/"):
        if os.path.isfile(f"target/release/{x}") and not "." in x:
            executables.append(x)

for x in executables:
    shutil.copyfile(f"target/release/{x}", f"target/archives/raw_archive/bin/{x}")
    print(f"Copied {x} to /bin")

print("Copying Data Files")
data = os.listdir("data")
for x in data:
    if os.path.isdir(f"data/{x}"):
        shutil.copytree(f"data/{x}", f"target/archives/raw_archive/{x}")
    else:
        shutil.copyfile(f"data/{x}", f"target/archives/raw_archive/{x}")
    print(f"Copied '{x}'")


print("Creating Standard Archive")
shutil.make_archive(  # Create the target archive
    "target/archives/standard", "zip", "target/archives/raw_archive/"
)
