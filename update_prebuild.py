#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import subprocess
import shutil
import glob
import sys
from typing import List, Optional

# --- Configuration ---
# The directory where your pre-generated code for different features is stored.
PREBUILD_DIR = "src/prebuilds"
# The cargo target directory.
TARGET_DIR = "target"
# A list of files to copy from the OUT_DIR to the prebuild directory.
FILES_TO_COPY = ["_generated.rs", "regs.rs", "features.txt"]

CRATE_NAME = "musb"

def find_latest_out_dir(crate_name: str) -> Optional[str]:
    """
    Finds the most recently modified build script output directory (OUT_DIR)
    for the given crate name. This is necessary because Cargo adds a hash
    to the build directory name.
    """
    # Path where build script outputs are located for debug builds.
    debug_build_path = os.path.join(TARGET_DIR, "debug", "build")
    if not os.path.isdir(debug_build_path):
        print(f"Warning: Build directory '{debug_build_path}' does not exist.")
        return None

    # Create a search pattern for the crate's build directories.
    pattern = os.path.join(debug_build_path, f"{crate_name}-*")
    
    # Find all matching directories.
    build_dirs = [d for d in glob.glob(pattern) if os.path.isdir(d)]
    if not build_dirs:
        print(f"Warning: No build directories found for crate '{crate_name}'.")
        return None
        
    # Get the most recently modified directory.
    latest_dir = max(build_dirs, key=os.path.getmtime)
    out_dir = os.path.join(latest_dir, "out")
    
    if os.path.isdir(out_dir):
        return out_dir
    else:
        print(f"Warning: 'out' subdirectory not found in '{latest_dir}'.")
        return None


def run_command(command: List[str], description: str) -> bool:
    """
    Runs a shell command, captures its output, and prints errors if any.
    Returns True on success, False on failure.
    """
    print(f"-> Running: {' '.join(command)}")
    try:
        subprocess.run(
            command,
            check=True,       # Raise an exception for non-zero exit codes.
            capture_output=True, # Capture stdout and stderr.
            text=True,        # Decode output as text.
            encoding="utf-8"
        )
        print(f"   {description} successful.")
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error during '{description}':")
        # Print stderr for detailed error information.
        print(e.stderr)
        return False
    except FileNotFoundError:
        print(f"Error: Command '{command[0]}' not found. Is Cargo installed and in your PATH?")
        return False


def main():
    """
    Main function to orchestrate the prebuild update process.
    """
    # 1. Get the crate name from Cargo.toml.
    crate_name = CRATE_NAME
    print(f"Operating on crate: '{crate_name}'")

    # 2. Discover all 'builtin-xxx' features from subdirectories in PREBUILD_DIR.
    if not os.path.isdir(PREBUILD_DIR):
        print(f"Error: Prebuild directory '{PREBUILD_DIR}' not found.")
        sys.exit(1)
        
    features = [d for d in os.listdir(PREBUILD_DIR) if os.path.isdir(os.path.join(PREBUILD_DIR, d))]
    if not features:
        print(f"No feature directories found in '{PREBUILD_DIR}'. Nothing to do.")
        return
        
    print(f"Found features to update: {', '.join(features)}")

    # 3. Iterate over each feature, build the project, and copy the generated files.
    for feature in features:
        full_feature_name = f"builtin-{feature}"
        print(f"\n--- Processing feature: {full_feature_name} ---")

        # Clean the project to ensure a fresh build for each feature.
        # This prevents using a cached OUT_DIR from a previous build with different features.
        if not run_command(["cargo", "clean"], "Project cleanup"):
            print("Cleanup failure.")
            break

        # Build the project with the specific feature enabled.
        build_command = ["cargo", "build", "--no-default-features", "--features", full_feature_name]
        if not run_command(build_command, "Cargo build"):
            print(f"Build failed for feature '{full_feature_name}'. Skipping.")
            break

        # Find the OUT_DIR for this specific build.
        out_dir = find_latest_out_dir(crate_name)
        if not out_dir:
            print(f"Could not find OUT_DIR for feature '{full_feature_name}'. Skipping file copy.")
            break
        print(f"   Found OUT_DIR: {out_dir}")

        # Define the destination directory for the generated files.
        dest_dir = os.path.join(PREBUILD_DIR, feature)
        print(f"   Copying generated files to: {dest_dir}")

        # Copy each required file from OUT_DIR to the destination.
        copied_count = 0
        for filename in FILES_TO_COPY:
            src_file = os.path.join(out_dir, filename)
            
            if os.path.exists(src_file):
                try:
                    shutil.copy(src_file, dest_dir)
                    print(f"     - Copied {filename}")
                    copied_count += 1
                except IOError as e:
                    print(f"     - Error copying {filename}: {e}")
            else:
                print(f"     - Source file not found, skipping: {src_file}")
        
        if copied_count == 0:
            print("   Warning: No files were copied for this feature. Please check build.rs output.")

    print("\n--- Prebuild update process finished. ---")


if __name__ == "__main__":
    main()
