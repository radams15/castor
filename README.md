# Castor

A graphical client for plain-text protocols written in Rust with GTK.
It currently supports the Gemini, Gopher and Finger protocols.

![Screenshot](https://juliensharing.s3.amazonaws.com/castor-icon.png)


## Installation

Castor needs a recent Rust version >= 1.39. Please consider using [Rustup](https://rustup.rs)
if you OS does not package a recent version.

### Dependencies

You will need some development libraries:

- openssl
- gtk+3
- gdk-pixbuf
- pango
- atk
- cairo

### Build and install

- run `make` to build Castor
- install with `sudo make install`
- Open Castor and visit gemini://gemini.circumlunar.space and enjoy your trip!


## Mailing list

If you have questions, feature requests, bugs or you just want to keep up to date with Castor you
can send a message to the [mailing list](https://lists.sr.ht/~julienxx/castor)


## Roadmap

You can view my current roadmap [here](https://todo.sr.ht/~julienxx/Castor)


## Thanks

- Leonora Tindall for the [great article](https://nora.codes/tutorial/speedy-desktop-apps-with-gtk-and-rust/) on Rust and GTK that helped me bootstrap this project
- tiwesdaeg for the incredible icon
- sloum for the great advices
- the gemini/gopher/finger community for being awesome
