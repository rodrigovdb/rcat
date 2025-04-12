# Docker

The docker image will compile the program and then link the executable to `/usr/bin/rcat`.

## Running commands

TL;DR - One command to rule them all
```zsh
$ docker-compose run --build --rm app  
```

## Working with the container

build the image
```zsh
$ docker build -t cat-rust .
```

running the image
```zsh
$ docker run -it cat-rust
```

Running a bash into the container
```zsh
$ docker run -it cat-rust bash
```

Running rcat
```zsh
$ rcat -h
```

## Working with  Docker Compose

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

Running rcat
```zsh
$ rcat -h
```
