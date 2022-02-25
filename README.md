# RustDo

> Your Todo's backend

## Environment

Create your local databse

```
mysql> create database rustdo;
```

Configure your `.env` file

```
> cp .env.example .env
```

## Migrations

At first install `Diesel CLI`

```
cargo install diesel_cli
```

Then run migrations

```
diesel migration run
```

## Routes

### Index

>GET `/todos`

### Show

>GET `/todos/{id}`

### Store

>POST `/todos`

### Update

>PUT `/todos/{id}`

### Delete

>DELETE `/todos/{id}`
