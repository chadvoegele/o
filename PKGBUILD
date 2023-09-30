# Maintainer: Chad Voegele

pkgname=o-git
pkgver=1
pkgrel=1
pkgdesc="open files based on their mime type"
arch=('i686' 'x86_64')
license=('MIT')
depends=('file')
makedepends=('git' 'cargo')
url="https://github.com/chadvoegele/o"
source=("$pkgname::git+$url")
sha256sums=('SKIP')
provides=('o=$pkgver')

pkgver() {
  git -C "${pkgname}" rev-parse --short HEAD
}

build() {
  cd "${srcdir}/${pkgname}"
  cargo build --release
}

package() {
  cd "${srcdir}/${pkgname}"
  install -D target/release/o -t "${pkgdir}/usr/bin/"
}
