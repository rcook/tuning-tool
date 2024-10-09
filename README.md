# Tuning Tool

[![CI](https://github.com/rcook/tuning-tool/actions/workflows/ci.yaml/badge.svg)][ci-workflow]
[![Release](https://github.com/rcook/tuning-tool/actions/workflows/release.yaml/badge.svg)][release-workflow]
[![Latest release](https://img.shields.io/github/v/tag/rcook/tuning-tool)][latest-release]

Command-line tool to import synthesizer microtunings from [Scala][scala]
`.scl` and `.kbm` files as well as
[MIDI Tuning Specification (MTS)][mts] SysEx bulk tuning dumps and send
them as MTS real-time tuning changes to hardware synths such as
[Novation Bass Station II][bass-station-ii]. This project also aims to
be interoperable with [Surge XT][surge-xt].

## Licence

[MIT License](LICENSE)

[bass-station-ii]: https://novationmusic.com/products/bass-station-ii
[ci-workflow]: https://github.com/rcook/tuning-tool/actions/workflows/ci.yaml
[latest-release]: https://github.com/rcook/tuning-tool/releases
[mts]: https://midi.org/midi-tuning-updated-specification
[release-workflow]: https://github.com/rcook/tuning-tool/actions/workflows/release.yaml
[scala]: https://huygens-fokker.org/scala/
[surge-xt]: https://surge-synthesizer.github.io/
