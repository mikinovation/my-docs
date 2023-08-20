# アプリの作成の仕方

1. Dockerfileの作成

[最新のRubyイメージ](https://hub.docker.com/_/ruby)を参照

```Dockerfile
FROM ruby:3.2.2
RUN apt-get update -qq && apt-get install -y nodejs default-mysql-client
WORKDIR /app
COPY Gemfile /app/Gemfile
COPY Gemfile.lock /app/Gemfile.lock
RUN bundle install

COPY entrypoint.sh /usr/bin/
RUN chmod +x /usr/bin/entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]
EXPOSE 3000

CMD ["rails", "server", "-b", "0.0.0.0"]
```

DBは`mysql`なら`default-mysql-client`、`postgres`なら`postgresql-client`をインストールする

2. Gemfileの作成

[最新のRailsバージョン](https://rubyonrails.org/)を参照

```Gemfile
source 'https://rubygems.org'
gem 'rails', '~>7.0.7'
```

3. Gemfile.lockの作成

```bash
touch Gemfile.lock
```

4. entrypoint.shの作成

```shell
#!/bin/bash
set -e

# Remove a potentially pre-existing server.pid for Rails.
rm -f /myapp/tmp/pids/server.pid

# Then exec the container's main process (what's set as CMD in the Dockerfile).
exec "$@"
```

5. docker-compose.ymlの作成

```yml
services:
  db:
    image: mysql:8.1.0
    volumes:
      - ./tmp/db:/var/lib/postgresql/data
    environment:
      POSTGRES_PASSWORD: password
  web:
    build: .
    command: bash -c "rm -f tmp/pids/server.pid && bundle exec rails s -p 3000 -b '0.0.0.0'"
    volumes:
      - .:/myapp
    ports:
      - "3000:3000"
    depends_on:
      - db
```

6. Railsアプリの作成

```bash
docker compose run --no-deps web rails new . --force --database=postgresql --api
```