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

or 

```bash
make db-up
```

### check

```bash
mysql -u root -p
enter password: password
show databases;
use mydatabase;
show tables;
select * from circles;
select * from members;
```

### down

```bash
make db-down
```