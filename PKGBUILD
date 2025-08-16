# Maintainer: Andrew McCall - AndrewDavidMcCall@hotmail.com

pkgname=timer-cli-rs
pkgver=1.0.0
pkgrel=1
pkgdesc="A super lightweight and zero-dependency command-line countdown timer."
arch=('x86_64')
url="https://github.com/Andrew-McCall/timer-cli-rs"
license=('MIT')
depends=()
makedepends=('cargo')
source=("$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('ac4cdf0faf0b2f8f6a27d3dc8999b0c0d4f61929330f45fb705433c2c67e9608')  

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/timer" "$pkgdir/usr/bin/timer"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 ReadMe.md "$pkgdir/usr/share/doc/$pkgname/ReadMe.md"
}

