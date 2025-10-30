# Maintainer: youao.du@gmail.com
pkgname=requiem
pkgver=0.0.1
pkgrel=1
pkgdesc="A lightweight, high-performance HTTP client built with Rust and iced"
arch=('x86_64')
url="https://github.com/youaodu/Requiem"
license=('MIT')
depends=(
  'gcc-libs'           # libgcc_s.so.1
  'glibc'              # libc.so.6, libm.so.6
  'openssl'            # libssl.so.3, libcrypto.so.3
  'fontconfig'         # Required by iced GUI framework
)
optdepends=(
  'adobe-source-han-sans-otf-fonts: Chinese character support'
  'nodejs: Required for AI features (Claude Code/Codex)'
  'npm: Required for AI features (Claude Code/Codex)'
)
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Run 'updpkgsums' to generate

prepare() {
  cd "$pkgname-$pkgver"
  # Download Rust dependencies
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --release --frozen
}

check() {
  cd "$pkgname-$pkgver"
  cargo test --release --frozen
}

package() {
  cd "$pkgname-$pkgver"

  # Install binary
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # Install license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

  # Install desktop file (future)
  # install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"

  # Install icon (future)
  # install -Dm644 "assets/icon.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"

  # Install locales
  install -Dm644 locales/en.json "$pkgdir/usr/share/$pkgname/locales/en.json"
  install -Dm644 locales/zh.json "$pkgdir/usr/share/$pkgname/locales/zh.json"
}
