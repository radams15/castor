# Castor

A graphical client for plain-text protocols written in Rust with GTK.
It currently supports the Gemini, Gopher and Finger protocols.

Gemini:

![Screenshot](https://juliensharing.s3.amazonaws.com/castor_gemini.png)


Gopher:

![Screenshot](https://juliensharing.s3.amazonaws.com/castor_gopher.png)


Finger:

![Screenshot](https://juliensharing.s3.amazonaws.com/castor_finger.png)


Gemini with some theming:

![Screenshot](https://juliensharing.s3.amazonaws.com/castor_theme.png)



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


## Changing settings

You can change some settings like initial URL, colors and delimiters for Headers, Lists, Text and Background.
Edit `~/.local/share/castor_settings.toml` and add the values you need.
These are the keys currently supported, you can use hex codes, plain colors names or even emojis!

```
[general]
start_url = "gemini://gemini.circumlunar.space/capcom"

[colors]
h1 = "red"
h2 = "#FF6347"
h3 = "green"
list = "#C71585"
text = "#FF1493"
background = "#FFC0CB"

[characters]
h1 = ">"
h2 = "))"
h3 = "}}}"
list = "🌼"
```


## Using client certificate

Castor expects your certificates to be placed in your home directory and named after the gemini capsule domain.
For example to water your plant on `gemini://astrobotany.mozz.us/plant` you will need to have `astrobotany.mozz.us.crt`
and `astrobotany.mozz.us.key` available in your home.


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
