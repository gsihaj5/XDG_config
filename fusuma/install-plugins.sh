#!/bin/sh
# ponytail: one-shot setup; patches tap plugin gemspec so it loads under fusuma 3.x
set -e

gem install --user-install fusuma-plugin-tap fusuma-plugin-appmatcher

tap_gem=$(ruby -e "puts Gem::Specification.find_by_name('fusuma-plugin-tap').gem_dir")
gemspec="$tap_gem/fusuma-plugin-tap.gemspec"
sed -i "s/add_dependency 'fusuma', '~> 2.0'/add_dependency 'fusuma', '>= 2.0'/" "$gemspec"

pkill -x fusuma 2>/dev/null || true
sleep 1
fusuma -d
