FROM php:8.3-fpm

# Installation des dépendances pour les modules PHP
RUN apt-get update && \
    apt-get install -y zip curl libcurl3-dev libzip-dev libpng-dev libonig-dev libxml2-dev

# Installation des extensions PHP
RUN docker-php-ext-install curl gd

# Installe et configure ImageMagick
RUN apt-get install -y libmagickwand-dev
RUN pecl install imagick
RUN docker-php-ext-enable imagick
RUN apt-get purge -y libmagickwand-dev

# Installe Composer
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

# Installation des extensions PHP
RUN docker-php-ext-install mbstring mysqli pdo pdo_mysql xml

# Installation d'xdebug
RUN pecl install xdebug && docker-php-ext-enable xdebug
