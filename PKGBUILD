# Maintainer: Alex Kolarski <alex@kolarski.com>
pkgname=s
pkgver=0.1.2
pkgrel=1
pkgdesc="A minimalist wrapper around screen that makes terminal session management dead simple"
arch=('x86_64')
url="https://github.com/kolarski/s"
license=('GPL-3.0')
depends=('screen')
source=("$pkgname-$pkgver::https://github.com/kolarski/s/releases/download/v$pkgver/s-linux-amd64")
sha256sums=('4c78c0353d0eb05b964b48a3f9f3f7add9f706bb2872b91d0232d63e198063eb')

package() {
    install -Dm755 "$srcdir/$pkgname-$pkgver" "$pkgdir/usr/bin/s"
    install -Dm644 "$srcdir/../LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "$srcdir/../README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
} 