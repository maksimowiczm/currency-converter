## Currency converter

Currency Converter is a command-line tool that enables currency conversion using real-time exchange rate data fetched from an API.

## Run application

### Requirements

- Docker
- Docker compose (if you want to use caching)
- [FreeCurrencyAPI](https://freecurrencyapi.com) API key which you can obtain on your dashboard [here](https://app.freecurrencyapi.com/dashboard)

#### Using Docker image

Even though both x64 and arm64 images are available on [DockerHub repository](https://hub.docker.com/r/maksimowiczm/currency-converter)
you can build your own image.

```sh
$ docker build . -t maksimowiczm/currency-converter
```

You can perform specific exchange rate lookups and convert given amounts.

```sh
$ docker run --rm maksimowiczm/currency-converter \
--api-key <API-KEY> <SOURCE_CURRENCY_CODE> <TARGET_CURRENCY_CODE> <AMOUNT>
```

Example usage - convert 10 USD to PLN:

```sh
$ docker run --rm maksimowiczm/currency-converter \
--api-key <API-KEY> USD PLN 10
39.713004176 3.9713004176
```

You can also list all available exchange rates for a given currency.
```sh
$ docker run maksimowiczm/currency-converter \
--api-key <API-KEY> <SOURCE_CURRENCY_CODE>
```

Example usage - list all exchange rates for USD:


```sh
$ docker run maksimowiczm/currency-converter \
--api-key <API-KEY> USD

Exchange rates for USD
AUD = 1.532520295
BGN = 1.7961802696
BRL = 5.031370905
CAD = 1.3572002336
... (all available exchange rates)
```


### Caching

You can enhance the performance by utilizing Redis as your caching service.
Steps to set up and use Redis caching:

**1. Docker Compose Setup**

Start Redis using the provided Docker Compose file included in the repository.

```sh
$ docker-compose up -d
```

**2. Environment Variables**

Set the connection string for Redis server using the **REDIS_URL** environmental variable.
You can track cache reads and writes enabling logger with **RUST_LOG** enviromental variable.

**3. Example usage**

```sh
$ docker run --rm --network maksimowiczm-currency \
-e RUST_LOG=info -e REDIS_URL=redis://cache:6379 maksimowiczm/currency-converter \
--api-key <API_KEY> USD

[INFO  currency_converter] Using Redis as cache service
[INFO  currency::api::cache_currency_service] Cache miss with key = "USD-PLN"
[INFO  currency::api::api_currency_service] Fetched USD-PLN
[INFO  currency::api::cache_currency_service] Stored key = "USD-PLN"
39.713004176 3.9713004176

# run again with same Redis server
$ docker run --rm --network maksimowiczm-currency \
-e RUST_LOG=info -e REDIS_URL=redis://cache:6379 maksimowiczm/currency-converter \
--api-key <API_KEY> USD

[INFO  currency_converter] Using Redis as cache service
[INFO  currency::api::cache_currency_service] Cache hit with key = "USD-PLN"
39.713004176 3.9713004176
```


### CLI Manual

```
Usage: currency-converter --api-key <API_KEY> <SOURCE_CURRENCY_CODE> [ <TARGET_CURRENCY_CODE> <AMOUNT> ]

Arguments:
  <SOURCE_CURRENCY_CODE>  Source currency code
Optional arguments:
  <TARGET_CURRENCY_CODE>  Target currency code
  <AMOUNT>                Amount which will be converted

Options:
      --api-key <API_KEY>  API key used for authentication
```

### TODO
- more unit testing
- user-friendly interactive mode that guides users through the currency conversion process step by step.