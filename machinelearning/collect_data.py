from pynput import keyboard
import pyautogui
import time
import os
import sys

# CHANGE THIS TO DETERMINE WHAT IMAGE WE ARE SAVING
IS_DISTRACTED = True

base_dir = "dataset"
subdir = "distracted" if IS_DISTRACTED else "focused"
save_dir = os.path.join(base_dir, subdir)
os.makedirs(save_dir, exist_ok=True)

def take_screenshot():
    timestamp = time.strftime("%Y%m%d-%H%M%S")
    filename = os.path.join(save_dir, f"screenshot_{timestamp}.png")
    pyautogui.screenshot().save(filename)
    print(f"Screenshot saved to {filename}")

def exit_script():
    print("Exiting script via Ctrl+C hotkey.")
    os._exit(0)  # Immediate exit; use sys.exit(0) if you prefer

with keyboard.GlobalHotKeys({
        '<ctrl>+<alt>+s': take_screenshot,
        '<ctrl>+c': exit_script
    }) as h:
    print(f"Running. Press Ctrl+Alt+S to save a screenshot to '{subdir}'.")
    print("Press Ctrl+C at any time to exit.")
    h.join()
