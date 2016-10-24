# Maintainer: pajowu <github@ca.pajowu.de>
pkgname=poettering-bot
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A twitter bot helping the lennart to come up with new things to reinvent"
url="https://github.com/pajowu/poettering-bot"
license=('GPL')
backup=('etc/config.json')

_gitroot='https://github.com/pajowu/poettering-bot.git'
_gitname='poettering-bot'

prepare() {
  	if [[ -d "$_gitname" ]]; then
    	cd "$_gitname" && git pull origin
  	else
	    git clone "$_gitroot" "$_gitname"
  	fi
}

build() {
    return 0
}

package() {
    cd "$_gitname"
    cargo install --root="$pkgdir"
    mkdir -p "$pkgdir/etc/poettering-bot"
    install -vDm644 wordlist $pkgdir/etc/poettering-bot/wordlist
    install -vDm644 config.json.example $pkgdir/etc/poettering-bot/config.json.example
    install -vDm644 systemd/poettering-bot.service $pkgdir/usr/lib/systemd/system/poettering-bot.service
    install -vDm644 systemd/poettering-bot.timer $pkgdir/usr/lib/systemd/system/poettering-bot.timer
}

post_install() {
    echo ">>> You need to add your consumer key/secret to /etc/poettering-bot/config.json (see /etc/poettering-bot/config.json.example)"
    echo ">>> You need to enable the systemd  timer by running:"
    echo ">>> systemctl enable poettering-bot.timer"
}