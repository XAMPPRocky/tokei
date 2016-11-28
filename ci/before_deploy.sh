# `before_deploy` phase: here we package the build artifacts

set -ex

. $(dirname $0)/utils.sh

# Generate artifacts for release
mk_artifacts() {
    cargo build --target $TARGET --release
}

mk_tarball() {
    # create a "staging" directory
    local td=$(mktempd)
    local out_dir=$(pwd)

    # TODO update this part to copy the artifacts that make sense for your project
    # NOTE All Cargo build artifacts will be under the 'target/$TARGET/{debug,release}'
    cp target/$TARGET/release/hello $td

    pushd $td

    # release tarball will look like 'rust-everywhere-v1.2.3-x86_64-unknown-linux-gnu.tar.gz'
    tar czf $out_dir/${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz *

    popd
    rm -r $td
}

# Package your artifacts in a .deb file
# NOTE right now you can only package binaries using the `dobin` command. Simply call
# `dobin [file..]` to include one or more binaries in your .deb package. I'll add more commands to
# install other things like manpages (`doman`) as the needs arise.
# XXX This .deb packaging is minimal -- just to make your app installable via `dpkg` -- and doesn't
# fully conform to Debian packaging guideliens (`lintian` raises a few warnings/errors)
mk_deb() {
    # TODO update this part to package the artifacts that make sense for your project
    dobin target/$TARGET/release/hello
}

main() {
    mk_artifacts
    mk_tarball

    if [ $TRAVIS_OS_NAME = linux ]; then
        if [ ! -z $MAKE_DEB ]; then
            dtd=$(mktempd)
            mkdir -p $dtd/debian/usr/bin

            mk_deb

            mkdir -p $dtd/debian/DEBIAN
            cat >$dtd/debian/DEBIAN/control <<EOF
Package: $PROJECT_NAME
Version: ${TRAVIS_TAG#v}
Architecture: $(architecture $TARGET)
Maintainer: $DEB_MAINTAINER
Description: $DEB_DESCRIPTION
EOF

            fakeroot dpkg-deb --build $dtd/debian
            mv $dtd/debian.deb $PROJECT_NAME-$TRAVIS_TAG-$TARGET.deb
            rm -r $dtd
        fi
    fi
}

main
