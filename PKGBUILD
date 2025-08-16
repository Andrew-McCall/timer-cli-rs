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
sha256sums=('06544b2ccb202bb15b780d05b4e20da2de9da4ef5fa19f3c03b9cfc3c8d4ac78')  

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

