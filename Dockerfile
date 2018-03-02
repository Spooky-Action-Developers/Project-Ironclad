#This is a commented out stand-in file
#for what will be our Docker distribution file

#FROM rust:1.23.0

#WORKDIR
#COPY . /usr/src/myapp

#RUN cargo install
#CMD ["project-ironclad"]

#Below are command line instructions to build and compile release version
#of project-ironclad project

#$ docker build -t project-ironclad
#$ docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.23.0 cargo build --release
