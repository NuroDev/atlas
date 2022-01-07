<div align='center'>

  <a href='https://github.com/nurodev/atlas/releases'>
    <img alt='Atlas' src='./assets/Banner.png?sanitize=true' />
  </a>

[![Build-Windows](<https://img.shields.io/github/workflow/status/nurodev/atlas/%F0%9F%9A%80%20Engine%20(Windows)?label=%20&logo=windows&logoColor=white&style=for-the-badge>)](https://github.com/NuroDev/atlas/actions?query=workflow%3A%22%F0%9F%9A%80+Engine+%28Windows%29%22)
[![Build - macOS](<https://img.shields.io/github/workflow/status/nurodev/atlas/%F0%9F%9A%80%20Engine%20(macOS)?label=%20&logo=apple&logoColor=white&style=for-the-badge>)](https://github.com/NuroDev/atlas/actions?query=workflow%3A%22%F0%9F%9A%80+Engine+%28macOS%29%22)
[![Build - Linux](<https://img.shields.io/github/workflow/status/nurodev/atlas/%F0%9F%9A%80%20Engine%20(Linux)?label=%20&logo=linux&logoColor=white&style=for-the-badge>)](https://github.com/NuroDev/atlas/actions?query=workflow%3A%22%F0%9F%9A%80+Engine+%28Linux%29%22)

<!-- [![Book](https://img.shields.io/badge/%20%F0%9F%93%95%20book-ef3242.svg?longCache=true&style=for-the-badge)](https://atlasengine.dev)
[![Docs](https://img.shields.io/badge/%20%F0%9F%92%A1%20docs-3287ef.svg?longCache=true&style=for-the-badge)](https://docs.atlasengine.dev/atlas)
[![License](https://img.shields.io/badge/%20%F0%9F%93%84%20mit-blueviolet.svg?longCache=true&style=for-the-badge)](https://opensource.org/licenses/MIT) -->

  <br />
</div>

## 👨‍💻 Development

### Requirements

To use this project you will need the following tools installed on your system:

- [Rust](https://rustup.rs/)
- [rustfmt](https://github.com/rust-lang/rustfmt)

### Clone

To fetch the repository, use the following command:

```zsh
git clone https://github.com/nurodev/atlas.git
```

### Build

To build the project with your desired target platform, use the following command & replacing the `--features` parameter with your desired backend target:

```zsh
cargo build --features [dx11|dx12|metal|vulkan]
```

### Run

Similarly to run the applicatio, use the following command:

```zsh
cargo run --features [dx11|dx12|metal|vulkan]
```

## 🔑 License

MIT © [Ben Dixon](https://github.com/NuroDev/atlas/blob/main/LICENSE)
