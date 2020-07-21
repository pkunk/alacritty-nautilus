pkgname=alacritty-nautilus
pkgver=0.1.0
pkgrel=1
pkgdesc="Alacritty plugin for Nautilus"
arch=('x86_64')
url="https://github.com/pkunk/alacritty-nautilus"
license=('GPL3')
depends=('alacritty' 'nautilus')
makedepends=('cargo' 'git')
source=("$pkgname::git+https://github.com/pkunk/$pkgname")
sha256sums=('SKIP')

build() {
  cd "$pkgname"

  cargo build --release --locked
}

pkgver() {
  cd "$pkgname"
  local tag=$(git tag --sort=-v:refname | grep '^[0-9]' | head -1)
  local commits_since=$(git rev-list $tag..HEAD --count)
  echo "$tag.r$commits_since.$(git log --pretty=format:'%h' -n 1)"
}

package() {
  cd "$pkgname"

  install -Dm644 "target/release/libalacritty_nautilus.so" "$pkgdir/usr/lib/nautilus/extensions-3.0/libalacritty_nautilus.so"
  install -Dm644 "COPYING" "$pkgdir/usr/share/licenses/${pkgname}/COPYING"
}
