# Maintainer: Thom Cameron <thomcm@proton.me>
pkgname='isw'
pkgdesc='a simple terminal stopwatch application'
url='https://gitlab.com/thom-cameron/isw'
license=('GPL-3.0-or-later')
pkgver='0.3.5.10.g2b93306'
pkgrel=1
arch=('i686' 'x86_64')
depends=('gcc-libs' 'glibc')
makedepends=('git' 'cargo')
provides=('isw')
conflicts=('isw')
source=("$pkgname::git+https://gitlab.com/thom-cameron/isw.git")
sha1sums=('SKIP')

pkgver() {
	cd "$pkgname"
	
	git describe --tags | sed 's/-/./g'
}

build() {
    cd $pkgname
    
    cargo build --release --locked
}

check() {
    cd $pkgname
    
    cargo test --release --locked
}

package() {
    cd "$pkgname"
    
    install -Dm755 target/release/isw "$pkgdir/usr/bin/isw"
    install -Dm644 license.txt "$pkgdir/usr/share/licenses/$pkgname/license.txt"
} 
