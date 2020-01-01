# Castor

`castor` is a graphical Gemini Project client written in Rust with GTK.

![Screenshot](https://juliensharing.s3.amazonaws.com/screenshot.png)


## Installation

Grab a release or build castor and install it manually.

- clone the repository

- install openssl, gtk+3, gdk-pixbuf, pango, atk, cairo development libraries.

- run `cargo build --release`

- copy the release somewhere and make it executable.

- Visit gemini://gemini.circumlunar.space


## TODO

- [x] Open links in external apps
- [x] Support input type
- [x] Bookmarking
- [x] Parse color codes
- [x] Handle Gopher
- [ ] Handle Gopher Images
- [ ] Build executables for multiple platforms
- [ ] Pass Conman's torture redirections tests
- [ ] Support theming
- [ ] Add loading state
- [ ] Change initial state
- [ ] Add/make an icon
