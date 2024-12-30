# Stock analytics

## Environment

- python 3.10.12

## Setup

### 1. Create .env

```env
DB_PATH=data.sqlite3
DB_URL=sqlite:///data.sqlite3
```

### 2. Create database

```sh
# Install pip dependencies
$ make setup_tools
$ make migrate
$ make seed
```