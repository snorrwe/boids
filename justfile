export RUST_LOG := "info,boids=debug,boids-core=debug,engine=debug"

watch-native:
    just launchers/native/watch

build-native flags="":
    just launchers/native/build {{flags}}
