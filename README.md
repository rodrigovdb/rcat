# Docker

build the image
```zsh
$ docker build -t cat-rust .
```

## Running the container

```zsh
$ docker run -it cat-rust
```

## Using the container as local env

Running a bash into the container
```zsh
$ docker run -it cat-rust bash
```

Compiling
```zsh
$ rustc main.rs
```

Running
```zsh
$ ./main
```

# Docker Compose

Build
```zsh
$ docker-compose build .
```

Run
```zsh
$ docker-compose run --rm app
```

Bash
```zsh
$ docker-compose run --rm app bash
```
