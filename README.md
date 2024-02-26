<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT LOGO -->
<br />
<!--<div align="center">
  <a href="https://github.com/haoud/silicium">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>-->

<!--<h3 align="center">Silicium</h3>-->
<h1 align="center">Silicium</h1>
  <p align="center">
    A micro-kernel written in Rust exploring modern concepts
    <br />
    <a href="https://github.com/haoud/silicium"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/haoud/silicium">View Demo</a>
    ·
    <a href="https://github.com/haoud/silicium/issues">Report Bug</a>
    ·
    <a href="https://github.com/haoud/silicium/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#building">Building</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

> [!IMPORTANT]
> Silicium is a work in progress and is not ready for production use.

Silicium is a *micro-kernel* written in Rust. It try to be a fast, portable kernel with a modern design. Its main goals are:
 - **Being portable**: Silicium is designed to be able to run on many architectures, by including the smallest 
 amount of architecture-specific code as possible in the kernel without sacrificing performance.
 - **Being (relatively) fast**: Silicium is designed to be fast. Micro-kernels are known to be slower than monolithic kernels, but Silicium try to mitigate this by being scalable and using modern techniques.
 - **Being ABI-agnostic**, meaning that it should be able to run, in theory, any program written for Linux, Windows or MacOS by using a server that will translate foreign ABIs to Silicium's ABI.
 - **Being secure**: Silicium is designed to be secure by default. It should be able to run untrusted code without any risk of compromising the system.
 - **Being educational**. Silicium is designed to be a learning experience for me and for others. It should be easy to understand and to contribute to, and well documented

> [!NOTE]
> The goals of Silicium are very ambitious and it is likely that it will never be able to achieve them. However, I think it's a good idea to aim high and to correctly design the project from the start to be able to achieve these goals in the future.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

To build this project, you will need to have Rust **and** `rustup` installed on your machine. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

There are also a few more dependencies in order to build and run the project:
- `qemu` for running the kernel in a virtual machine. You can install it with your package manager. Make sure to install the version corresponding to your target architecture (e.g. `qemu-system-x86_64` if you want to run the x86_64 kernel). This is not necessary the same as your host architecture.
- `gcc` for compiling the [Limine](https://github.com/limine-bootloader/limine) bootloader
- `lld` for linking the kernel
- `xorriso` for creating the ISO image

### Building

Clone the repository:
```sh
git clone --depth 1 https://github.com/haoud/silicium.git
```
Make all the scripts contained in the `scripts` directory executable:
```sh
chmod +x scripts/*
```
Build the kernel, servers and userland programs:
```sh
make build
```
Run the kernel in Qemy:
```sh
make run
```

> [!TIP]
> If you are lost, you can run `make help` to see all the available commands.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

> [!CAUTION]
> Running the kernel on real hardware is **strongly discouraged** as the kernel is not stable and may, even if unlikely, permanently damage your hardware or erasing your data. Use at your own risk.

To run Silicium:
  * http://copy.sh/v86/ : Upload .iso file as an CD-ROM image
  * QEMU: `make run-{i686/x86_64/aarch64}`
  * Real hardware: Burn .iso file to USB or CD.

By default, the kernel is built for all supported architectures, and the x86_64 version is run in a virtual machine.
If you want to run a specific target, you must specify it in the `make` command:
```sh
make run-aarch64
```

> [!WARNING]
> Although the kernel is designed to be portable, it is not yet tested on all architectures. The "main" architecture is x86_64, and the other architectures are not as well supported. By default, the `make run` command will run the x86_64 version of the kernel.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

### Milestone 1: Baremetal kernel (x86-64 only)

- [x] Babysteps (unit tests, CI, benchmarks...)
- [x] Serial logging
- [x] GDT/IDT/TSS
- [ ] Physical memory manager
- [ ] Virtual memory manager
- [ ] Allocations
- [ ] Paging
- [ ] APIC
- [ ] APIC timer
- [ ] SIMD
- [ ] SMP

### Milestone 2: Userspace, here I come !

- [ ] Userspace
- [ ] Syscalls
- [ ] Scheduling
- [ ] Multi-threading
- [ ] Asynchronous IPC
- [ ] Userspace ELF loader

### Milestone 3: Add a little bit of spice

 - [ ] VFS server
 - [ ] Ram filesystem
 - [ ] Graphical server
 - [ ] aarch64 support
 - [ ] i686 suport

### Milestone 4: An interactive userspace !
 
 - [ ] Shell
 - [ ] Basic commands
 - [ ] Mouse and keyboard support
 - [ ] Porting DOOM

### Milestone 5: ABI compatibility

 - [ ] Linux ABI

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Silicium is dual-licensed under the Apache License, Version 2.0 and the MIT license.
See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Writing an OS in Rust](https://os.phil-opp.com/), an amazing starter guide for OS development in Rust
* [OSDev](https://wiki.osdev.org/Main_Page), the golden resource for OS development
* [Limine](https://github.com/limine-bootloader/limine), a powerful bootloader that is simple and easy to use, and greatly reduce the complexity of the kernel
* [Rust](https://www.rust-lang.org/), for creating a amazing language that is (almost) perfect for OS development
* [README Template](https://github.com/othneildrew/Best-README-Template/blob/master/README.md)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
