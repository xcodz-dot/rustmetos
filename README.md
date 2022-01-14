# RustMetOS

To get it up and running, install standard rust compiler using rustup
and then run the following commands in below sections to build certain
components.

# Build

## Standard Distribution Archive

you also need python installed to run the build script. Python 3.7 should be
the minimum requirement

### Unix

```
cd {Root of the project}
cd rustmetos_std
python3 build.py
cp target/archives/standard.zip ../standard_dist.zip
```

### Windows

```
cd {Root of the project}
cd rustmetos_std
py -3 build.py
copy target\archives\standard.zip ..\standard_dist.zip
```

## Rustmetos Binary

## Unix / Windows

```
cd {Root of the project}
cd rustmetos
cargo build --release
```

the above should create a `rustmetos` or `rustmetos.exe` file under `{root of project}/rustmetos/target/release/`

Simply run the executable generated and when asked for standard distribution archive,
provide absolute path to the standard archive that we built using the commands before.
The archive should be in the root of the project after the steps were followed.

# Guide

A Simple guide to administrating this system. Let's start
at the step when we are done installing and building. Now
we are on `Rustmetos Shell` as it says after loading the
init configuration.

Let's study the directory structure of the Standard 
Installation. Standard installation comes
with a few directories at root one of the important ones
is `/bin/` and `/conf/`. The first contains binaries
neccessary to administrate the system and the latter
contains basic configurations. one of them is `init.conf`.
It contains the following content by default:

```ini
[exec]
name=sh
```

in the above configuration it defines a section named `exec`
which defines what executable to run at begginning of the 
program. By default it is internally harcoded to be 
`internal:core_shell` so that user has at least some
way to work with the sytem without any installation. `sh`
is an extension to that with multiple other features.

you can always delete the configuration or within the `sh`
you can type `boot set internal:core_shell` which just 
modifies this configuration to boot into core shell on boot.

In core shell, `help` can be typed to list all the commands.
The commands in core shell are concrete and are never
expected to be improved in feature but that is not the same
for `sh`. It is expected to improve with time and have
multiple features into it. You can do one very important
thing in core shell and that is to `reset`. As soon as you
type that, home is destroyed and is left only to be
recreated on next boot. The system has lost all the memories
and is back, fresh as new.

Below this guide you will find the list of commands provided
in standard distribution and in the order of there creation.

# Standard Commands

## `sh`

Shell for short. Just invoke it as is and it will lead you
to an interactive prompt that can be used to do further
more powerfull tasks that can be done in core shell.

To invoke it within the core shell:

```
exec sh
```

## `boot`

Boot command provides a simple configuration modification
utility for the `exec` section of `init.conf`. It allows
for one command as of now which is to `set` the boot
executable.

You might wanna use it to switch between core shell 
and `sh` or any other custom executable such as an init
system.

### Switch to core shell from `sh`
```
boot set internal:core_shell
```
### Switch to `sh` from core shell
```
exec boot set sh
```

## `read`

Reads from a file and prints to the console. It is more of
a copy of `cat {file}` from unix but way underpowered than
`cat`. You can use it to read any file in usually text format
to standard output.

### Reading the `init.conf` in `sh`

```
home
read conf/init.conf
```

## `unzip`

Unzips a zip file into a provided directory. For example given
a file named `example.zip` you can use this to extract its
contents to a folder.

### Extracting `example.zip` to a folder called `my_example` in `sh`

```
unzip example.zip my_example
```

# Contributing

Contribution is a neccessary motivation and heart of any open
source project. This project was just meant to be educational
as I needed some sort of purpose to learn rust with. I
encountered multiple situations while its development only to
help me learn and understand rust better. So this project
serves the purpose of motivation to me to learn rust but any
contributions are welcomed because I have my flaws and I would 
be greatful to learn about them.

The best way to contribute would be to improve existing software
or improving this single page documentation. The secondary way
to help would be to introduce more executables to the standard 
distrubtution.

