#  Rust EFL 

[Rust](https://www.rust-lang.org) bindings for the [Enlightenment Foundation Libraries](https://www.enlightenment.org/about-efl).

## Status

The [original rust-efl project](https://github.com/araujol/rust-efl) was created in April, 2014. [Araujol's](https://github.com/araujol)
commits stopped end of June 2014; then it was immediately [forked](https://github.com/amty/rust-efl),
followed by commits by [amty](https://github.com/amty) and  [suhr](https://github.com/suhr); they stopped work December 2014.

I (Arlo) have started updating the code for Rust 1.9.0 and EFL 1.17.1, but
Rust and the EFL have changed significantly since the first rust-efl code was written.
At the moment, the *test_simple* example runs, but other examples still need to be updated.

The code __needs a complete review__. I would not recommend using this for any serious project yet.
I will be contacting the original authors to see if they want to continue work and try to establish
who will be hosting the official repository.

## Requirements

The **rust-efl** bindings correspond to the latest stable code base
both for **EFL** and the **Rust** language. Slightly older versions may work, but
**rust-efl** development is based on these versions.
*Update in progress (as mentioned in above Status)*

- [Rust 1.9](https://www.rust-lang.org/downloads.html)
- [efl 1.17.1](https://www.enlightenment.org/download) *EFL core libraries*
- evas_generic_loaders *Loaders for Evas using 'generic' module*
- emotion_generic_players *Players for Emotion using 'generic' module*
- elementary *EFL widget toolkit*

## Building library:

In the top level directory of this source code:

    $ cargo build

## Run:

    $ cargo run --example test_simple

Enjoy!

