# Menhera Productivity

We like anime and we like productivity. So we put the two together to create

![salmon](https://preview.redd.it/heian-era-sukuna-and-gojo-hitting-the-absolute-cinema-pose-v0-7w0nmpk3klge1.jpeg?width=1125&format=pjpg&auto=webp&s=0ff9981dd9f60c376b46e7c807ae2a8a97cdaec1)

Menhera-chan (see [origin](#origin-of-menhera)) will sit in the corner of your screen and monitor for whether or not you are being productive. She will be happy when you stay productive, but get sad or angry when you lose focus. So stay focused! 

This project is a machine learning-supported productivity application built using Tauri, SvelteKit, and PyTorch. Programmed using Rust and TypeScript, along with Python for training a fine-tuned ResNet 18.

Project submission for [Innovation Hacks 2025 Hackathon GDSC challenge](https://devpost.com/software/menhera-productivity).


## Topic: AI For Productivity

Maintaining peak productivity while studying or working is tougher than ever - distractions like phones, social media, or even zoning out at your desk make it tough to stay on task. Timers and to-do lists help, but they don't keep you accountable in the moment. Your challenge is to design a solution that helps individuals stay on task by identifying moments of drift and guiding them back into a productive state.

## Origin of Menhera

"Menhera-chan" or Kurumi Nanase (Japanese: 七瀬くるみ) is a fictional character produced by the Japanese company Joynet and illustrated by the company's designer Pom (ぽむ). The character first appeared on the company's product line as stickers for the instant messenger app LINE on December 27th, 2017, and had a serialized manga that began on Pom's official website on May 8th. For further history and information on the character, see: https://knowyourmeme.com/memes/kurumi-nanase

According to Joynet's IP policy (see [here](https://enjoynet.co.jp/ip_policy/)), "The illustrated materials distributed by us can be used free of charge, whether by individuals or corporations, commercial or non-commercial" (Google Translated). Since this project is currently unmonetized and will remain unmonetized for the foreseeable future, this project makes all efforts to comply with their intellectual property policy. All rights belong to illustrator Pom and company Joynet.

Menhera images taken from the r/menhera subreddit ([here](https://www.reddit.com/r/menhera/comments/14w5bq7/leak_menherachan_hub_emote_archives/))

## Installation & Usage

### Environment Setup

Follow this page to install Tauri: https://v2.tauri.app/start/prerequisites

1. Install the necessary [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies) according to your operating system.
2. Install Rust
3. Install Node.js

Then follow this page to install pnpm: https://pnpm.io/installation

#### Installing Torch Dependencies

Follow the instructions in the [tch-rs](https://github.com/LaurentMazare/tch-rs/tree/main) repository to install the necessary dependencies for your operating system. Under is a summary of the installation process for each operating system.

##### Windows

1. Download LibTorch from [PyTorch](https://pytorch.org/get-started/locally/)

    - Stable (2.6.0)
    - Windows
    - LibTorch
    - C++/Java
    - CUDA 11.8

2. Unzip downloaded file
3. Enter environment variables
    - Create `LIBTORCH` and assign `X:\path\to\libtorch`
    - Add `X:\path\to\libtorch\lib` to the `Path` variable
4. Restart device

##### MacOS M1-M4 Chips

Install the dependencies using pip and symlink the libtorch library (based on [this comment](https://github.com/LaurentMazare/tch-rs/issues/488#issuecomment-1825404820)):

```sh
python3 -m pip install torch==2.6.0
export LIBTORCH_USE_PYTORCH=1
sudo cp <your_python_site_package_path>/torch/lib/lib** /usr/local/lib
cargo run
```

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Building the Project

First, install the project dependencies using pnpm:

```sh
pnpm install
```

Then build the project by running:

```sh
pnpm tauri dev
```
