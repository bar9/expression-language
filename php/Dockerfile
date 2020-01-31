FROM php:7.4

RUN apt-get update && apt-get install libffi-dev
RUN  docker-php-ext-configure ffi --with-ffi \
    && docker-php-ext-install ffi
