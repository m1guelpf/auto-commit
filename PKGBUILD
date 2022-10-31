pkgname="auto-commit"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="A CLI tool that generates commit messages from your staged changes, built in Rust and using OpenAI's Codex."
arch=("x86_64" "arm")
license=("mit")
url='https://github.com/m1guelpf/auto-commit'
makedepends=("git")
source=("git+https://github.com/m1guelpf/auto-commit.git")
sha512sums=("SKIP")

package() {
    cd auto-commit
    bash "./install.sh"
}
