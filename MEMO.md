## server

### up

```bash
docker compose up -d
docker compose exec -it server /bin/bash
```

### run

```bash
cargo run
```

### watch

```bash
./watch.sh
```


## db

### setup

```bash
cp .env.dist .env
```

### up

```bash
docker compose up -d
docker compose exec -it db /bin/bash
```

### check

```bash
mysql -u root -p
enter password: password
show databases;
use mydatabase;
show tables;
```