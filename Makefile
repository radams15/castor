# Install to /usr unless otherwise specified, such as `make PREFIX=/app`
PREFIX=/usr

# What to run to install various files
INSTALL=install
# Run to install the actual binary
INSTALL_PROGRAM=$(INSTALL)
# Run to install application data, with differing permissions
INSTALL_DATA=$(INSTALL) -m 644

# Directories into which to install the various files
bindir=$(DESTDIR)$(PREFIX)/bin
sharedir=$(DESTDIR)$(PREFIX)/share

# Just tell make that clean, install, and uninstall doesn't generate files
.PHONY: clean clean-all install install-data copy-data uninstall

# Build the application
target/release/castor : src
	cargo build --release

install : target/release/castor install-data
	# Create the bindir, if need be
	mkdir -p $(bindir)
	# Install binary
	$(INSTALL_PROGRAM) target/release/castor $(bindir)/castor

# Install the data files and update the caches
install-data : copy-data
	# Force icon cache refresh
	touch $(sharedir)/icons/hicolor
	update-desktop-database

# Just copy the data files, without updating caches
copy-data :
	# Create icon folders if needed
	mkdir -p $(sharedir)/icons/hicolor/scalable/apps/
	mkdir -p $(sharedir)/icons/hicolor/16x16/apps/
	mkdir -p $(sharedir)/icons/hicolor/32x32/apps/
	mkdir -p $(sharedir)/icons/hicolor/64x64/apps/
	mkdir -p $(sharedir)/icons/hicolor/128x128/apps/
	# Install icons
	$(INSTALL_DATA) data/org.typed-hole.castor.svg $(sharedir)/icons/hicolor/scalable/apps/org.typed-hole.castor.svg
	$(INSTALL_DATA) data/org.typed-hole.castor-16.png $(sharedir)/icons/hicolor/16x16/apps/org.typed-hole.castor.png
	$(INSTALL_DATA) data/org.typed-hole.castor-32.png $(sharedir)/icons/hicolor/32x32/apps/org.typed-hole.castor.png
	$(INSTALL_DATA) data/org.typed-hole.castor-64.png $(sharedir)/icons/hicolor/64x64/apps/org.typed-hole.castor.png
	$(INSTALL_DATA) data/org.typed-hole.castor-128.png $(sharedir)/icons/hicolor/128x128/apps/org.typed-hole.castor.png
	# Install desktop file
	$(INSTALL_DATA) data/Castor.desktop $(sharedir)/applications/Castor.desktop

uninstall :
	# Remove the .desktop
	rm -f $(sharedir)/applications/Castor.desktop
	# Remove the icon
	rm -f $(sharedir)/icons/hicolor/scalable/apps/org.typed-hole.castor.svg
	# Remove the binary
	rm -f $(bindir)/bin/castor

clean :
	cargo clean
