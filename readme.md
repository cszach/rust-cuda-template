# Rust CUDA template

This is a template that you can use for your own Rust CUDA projects. It took me
a while to set this up so my goal is to compile every problem I've come across
to help others save time. An example `add` package is included so you can verify
the setup.

## My system

- OS: Fedora 39
- CUDA version: 11.8 (GCC 11.2)
- LLVM version: 7.0.1

## Project structure

- `cpu/`: contains CPU packages, each in its own directory. CPU packages do
  stuff on the CPU like I/O and launching the GPU kernel;
- `gpu/`: contains GPU packages, which contains GPU kernels;
- `resources/`: where the compiled PTX files are created. This directory is
  initially empty.

All child folders in `cpu/` and `gpu/` are the project's virtual workspace
members (see [`Cargo.toml`](Cargo.toml)). You may want to add a root package.

## Build prerequisites

[1]: https://github.com/Rust-GPU/Rust-CUDA/blob/master/guide/src/guide/getting_started.md
[2]: #installing-multiple-cuda-versions
[3]: https://gist.github.com/ax3l/9489132
[4]: #installing-gcc

There are prerequisites documented by the Rust CUDA project (see [here][1]), but
I think it is incomplete. I will document what I needed to install on my system
below.

- The CUDA SDK, version 11.2 **to 11.8**. CUDA 12.0+ won't work at the moment.
  Also, see _[Installing multiple CUDA versions][2]_ below if you need.
  - When installing CUDA, you might also need the appropriate GCC version (or
    clang, but I use GCC). See [this compatibility table][3] and the
    _[Installing GCC][4]_ section below.
- LLVM 7.0-7.4. For this, I installed `llvm7.0-devel` and `llvm7.0-static` from
  the repositories.
  - At first, I only installed `llvm7.0-devel`, but build errors complained that
    my environment was missing some static LLVM libraries, hence
    `llvm7.0-static`. You might or might not need to install the latter.

### Installing multiple CUDA versions

[5]: https://blog.kovalevskyi.com/multiple-version-of-cuda-libraries-on-the-same-machine-b9502d50ae77

Many thanks to [this blog post][5] by Viacheslav.

When downloading CUDA, you need to select the "runfile (local)" installer type.
To run the installer, use the command below:

```shell
sudo sh cuda_11.8.0_520.61.05_linux.run --silent --toolkit --toolkitpath=/usr/local/cuda-11.8
```

- Be sure to replace the paths.
- If the installer says there is not enough space in `/tmp`, append the option
  `--tmpdir=/path/to/tmp`, replacing `/path/to/tmp` with a temporary folder that
  you created (e.g. in your home directory, like `/home/zach/tmp`).
- The installer will set the link `/usr/local/cuda` to point to the version that
  you just installed. You can make it point back to another version if you like.

### Installing GCC

[6]: https://gcc.gnu.org/mirrors.html

I couldn't find the version of GCC I needed in the Fedora DNF repositories, so
I had to build GCC myself (Rest. In. Peace). You might be able to find it in the
repositories; if not, here are the steps to build GCC.

1. Select from one of [the mirror sites][6]. Go to `/releases`, select the
   appropriate version of GCC, then download the archive (`.xz` for faster
   download);
2. Extract the folder, then `cd` into it;
3. Download the prerequisites: `./contrib/download_prerequisites`;
4. Create a build directory and `cd` into it: `mkdir build && cd build`;
5. Configure the build:
    ```shell
    ../configure --enable-languages=c,c++ --disable-multilib --program-suffix=-11.2
    ```
    Replace the program suffix with a string you like (usually `-` followed by
    the GCC version that is being installed). You may also want to adjust the
    `--prefix`; the default installation directory will be `/usr/local/`.
6. `make && sudo make install`. The `make` step might take a couple of hours, so
   sit back, relax, and enjoy. You can use the `-j` option for parallel builds;
7. When using the runfile installer to install CUDA (like in the above section),
   the installer will use `/usr/bin/gcc` and there is no option to change this
   path. As such, you might need to create a temporary symbolic link:
    ```shell
    sudo mv /usr/bin/gcc /usr/bin/gcc.backup # If /usr/bin/gcc is a binary
    sudo ln -s /usr/local/bin/gcc-11.2 /usr/bin/gcc
    sudo sh cuda… # Install CUDA as documented in the previous section
    sudo mv /usr/bin/gcc.backup /usr/bin/gcc # Restore /usr/bin/gcc
    ```
   …or use `update-alternatives`, if `/usr/bin/gcc` does not exist.

#### Double declared enum in `libsanitizer`

[7]: https://github.com/gcc-mirror/gcc/commit/d2356ebb0084a0d80dbfe33040c9afe938c15d19

When running `make` to build GCC 11.2, I encountered an error about an enum that
was already declared in `libsanitizer`. I edited the file
`libsanitizer/sanitizer_common/sanitizer_platform_limits_posix.cpp` according to
[this commit][7] and that solved the problem.

## Environment

After having the prerequisites installed, we need to set some environment
variables before building Rust CUDA projects. The environment variables needed
are:

1. `CUDA_HOME`: CUDA installation directory e.g. `/usr/local/cuda-11.8`;
2. `LLVM_CONFIG`: path to the `llvm-config` binary e.g.
   `/usr/lib64/llvm7.0/bin/llvm-config`. Might be redundant if
   running `llvm-config --version` outputs `7.x.x`;
3. `LD_LIBRARY_PATH`: set to the parent directory of `libnvvm.so` e.g.
   `/usr/local/cuda-11.8/nvvm/lib64`.

You may use the `locate` Linux command to locate where files are.

## Build

[8]: https://github.com/Rust-GPU/Rust-CUDA/issues/7#issuecomment-979426844

Try to build the [`add`](cpu/add) package:

```shell
cargo build -p add
```

If you get an `undefined symbol: setupterm` error, build with
`LLVM_LINK_SHARED=1` ([source][8]):

```shell
LLVM_LINK_SHARED=1 cargo build -p add
```
