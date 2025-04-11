# Docker

One command to rule them all
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
