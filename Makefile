SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

.PHONY: license
license:
	addlicense -l mit -s=only -c "AprilNEA LLC" crates
	addlicense -l mit -s=only -c "AprilNEA LLC" src-tauri
	addlicense -l mit -s=only -c "AprilNEA LLC" src
