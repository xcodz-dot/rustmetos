# CBA Specs
## Description
CBA is stands for Cross Binary Archive/App. It is an archive
in zip format. It contains all the neccessary data files as well
as metadata about the app. Uninstallation metadata can also be
provided to maintain some artifacts on the Host for future
reinstallation.

## Format

```
helloworld.cba
|-- bin
|   |-- hello.windows.x86_64
|   |-- hello.linux.x86_64
|   |-- hello.macos.aarch64
|   |-- hello.macos.x86_64
|   |-- hello.freebsd.riscv64
|
|-- data
|   |-- translations
|       |-- en.json
|       |-- gb.json
|
|-- platforms.list
|-- metadata.json
|-- install.json
|-- uninstall.json (*)

* - Optional
```

The source for above can be seen in the [examples](https://github.com/xcodz-dot/rustmetos/tree/master/rustmetos_apps/examples/helloworld) repository.

Below are the definations and schemas for the above expected files.

### `cba/bin/`
Bin folder should contain binaries with `binary_name.platform.architecture`. On windows the prefix `.exe` is also required to
be removed because it will be added while unpacking.

### `cba/data/`
Cba data folder should contain all the data related to the
application. All the data in this folder is placed under
`HOME/data/{application_data_folder_name}/`

### `cba/platforms.list`
It is a list of supported platforms separated by newlines.
Example content would be for the above example would be.

```
windows.x86_64
linux.x86_64
macos.aarch64
macos.x86_64
freebsd.riscv64
```

The platform name is taken from the `std::env::consts::OS`. And
the architecture value is taken from `std::env::const::ARCH`.

### `metadata.json`
All application related metadata is provided here.
In the above example the contents would be

```json
{
    "id": "rustmetos_apps.examples.helloworld",
    "name": "rustmetos",
    "homepage": "https://github.com/xcodz-dot/rustmetos"
}
```

The following feilds can be provided and the feilds marked
with astrix are optional:

#### `"id"`
Application ID to identify it. The standard way
of writing it is `{host}.{repository}.{app_name}`.
Applications that are stored within this repository
are named `rustmetos_apps.{repository}.{app_name}`.
Host can be any identifier but these are standard
and can avail special features in some cases.

* `rustmetos_apps`
* `github`
* `gitlab`

#### `"name"` (\*)
General identifier.

#### `"description"` (\*)
Description of the application

#### `"homepage"` (\*)
Homepage of the application

#### `"license"` (\*)
Name of license file. It is shown to the user while installation
of the file.

### `install.json`
Installation metadata is neccessary and must be provided.

For the above case the metadata would look something like this:

```json
{
    "bin": ["hello"],
    "data_directory_name": "rustmetos_apps.example.helloworld",
}
```
The following feilds can be provided and the feilds marked
with astrix are optional:

#### `"bin"`
Binary names in provided in `cba/bin` that are to be copied
to the `HOME/bin` directory. Make sure you do not
include the suffix that is meant for platform and architecture.

#### `"data_directory_name"`
Data directory name is supposed to be the name of the
directory created while installation in the `HOME/data`
folder. Under that directory all the contents of data
is extracted.

### `uninstall.json` (\*)
Does not serve any purpose currently and is reserved for future.
