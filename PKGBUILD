pkgname=dotfiles-installer
pkgver=0.2.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('any')

build() {
    cargo build --release
}

package() {
    cd $srcdir/..
    install -Dm755 "target/release/$pkgname" \
        -t "$pkgdir/usr/bin"
}
