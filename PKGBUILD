# Maintainer: Your Name <you@example.com>
pkgname=requiem
pkgver=0.1.0
pkgrel=1
pkgdesc="A lightweight, high-performance HTTP client built with Rust and iced"
arch=('x86_64')
url="https://github.com/youao/requiem"
license=('MIT')
depends=('glibc' 'openssl' 'gcc-libs')
makedepends=('rust' 'cargo')
optdepends=('adobe-source-han-sans-otf-fonts: Chinese character support')
source=()
sha256sums=()

build() {
  cd "$startdir"
  cargo build --release --locked
}

check() {
  cd "$startdir"
  cargo test --release --locked
}

package() {
  cd "$startdir"

  # Install binary
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # Install license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE" 2>/dev/null || true

  # Install documentation
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md" 2>/dev/null || true
}
