FROM ruby:3.2.2
RUN apt-get update -qq && apt-get install -y nodejs default-mysql-client
WORKDIR /var/www/app
COPY ./Gemfile .
COPY ./Gemfile.lock .
RUN bundle install

COPY entrypoint.sh /usr/bin/
RUN chmod +x /usr/bin/entrypoint.sh
EXPOSE 3000