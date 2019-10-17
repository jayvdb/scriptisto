# Scriptisto

**This is not an officially supported Google product**

## Installation

Install Rust, for example via https://rustup.rs/. Then

```shell
$ cd scriptisto
$ cargo install --force --path .
```

## Running

Try one of the script templates from one of the language available. To see the
list:

```shell
$ scriptisto -g
```

Then generate your basic script:

```shell
$ scriptisto -g rust | tee ./rust-script
$ chmod +x ./rust-script
$ ./rust-script
Hello, Rust!
```

## Source Code Headers

Every file containing source code must include copyright and license
information. This includes any JS/CSS files that you might be serving out to
browsers. (This is to help well-intentioned people avoid accidental copying that
doesn't comply with the license.)

Apache header:

    Copyright 2019 Google LLC

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        https://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.