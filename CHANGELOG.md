# Changelog

## [Unreleased]

* Add command line option `--transpose` to specify a number of semitones to be added or subtracted before printing the chord chart ([#24](https://github.com/noeddl/ukebox/issues/24)).

## [0.5.0] - 2020-01-02

* Add subcommand `name` for looking up the chord name(s) corresponding to a given chord fingering ([#18](https://github.com/noeddl/ukebox/issues/18)).
* Move the existing functionality of looking up chord charts to a new subcommand called `chart`.

## [0.4.0] - 2019-12-23

* Add a bunch of new chord types ([#31](https://github.com/noeddl/ukebox/issues/31)).
* Remove incorrect chord shapes for `D7` and `Dm7` ([#34](https://github.com/noeddl/ukebox/issues/34)).

## [0.3.0] - 2019-11-24

* Set up CI for the repo.
* Add command line option `--tuning` to specify a tuning (`C`, `D` or `G`) ([#1](https://github.com/noeddl/ukebox/issues/1)).

## [0.2.0] - 2019-10-13

* Add dominant and minor 7th chords.

## [0.1.0] - 2019-10-02

Initial version that

* can handle basic major and minor chords and
* allows the definition of a minimum fret.
