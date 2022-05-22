pkg_name=transect_backend
pkg_origin=lancewf
pkg_version="0.1.0"
pkg_maintainer="Lance Finfrock <lancewf@gmail.com>"
pkg_license=("Apache-2.0")
pkg_bin_dirs=(bin)

pkg_deps=(
  core/glibc
  core/gcc-libs
  core/mysql-client
)

pkg_build_deps=(
  core/rust
  core/gcc
  core/pkg-config
  core/make
  core/openssl
)

pkg_exports=(
   [port]=port
   [local_only]=local_only
)

pkg_binds=(
  [database]="port username password local_only"
)

do_build() {
  pushd "${PLAN_CONTEXT}/.."
    cp -r src ${CACHE_PATH}/.
    cp Cargo.toml ${CACHE_PATH}/.
    sed -i "s/pkg_name/$pkg_name/g" ${CACHE_PATH}/Cargo.toml
  popd
}

do_install() {
  cargo install --root "${pkg_prefix}" --path "${CACHE_PATH}" --verbose
}
