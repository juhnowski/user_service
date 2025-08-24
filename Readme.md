# Install
## MySQL
### Локально:
```bash
sudo systemctl start mysql
sudo systemctl enable mysql
sudo systemctl status mysql
mysql --version
mysql  Ver 8.0.42-0ubuntu0.24.04.2 for Linux on x86_64 ((Ubuntu))

sudo mysql
CREATE USER 'ilya'@'localhost' IDENTIFIED BY '1';
ALTER USER 'ilya'@'localhost' IDENTIFIED WITH caching_sha2_password BY 'pwd12345';

show databases;
create database user_service;
GRANT ALL PRIVILEGES ON user_service.* TO 'ilya'@'localhost';
exit;

mysql -u ilya -p
use user_service;
CREATE TABLE users ( id BIGINT AUTO_INCREMENT PRIMARY KEY, username VARCHAR(255) NOT NULL, email VARCHAR(255) NOT NULL);
exit;
```

### Удаленно:
```bash
mysql -h 192.168.0.108 -P 3306 -u root -p

CREATE USER 'ilya'@'172.17.0.1' IDENTIFIED BY '1';
ALTER USER 'ilya'@'172.17.0.1' IDENTIFIED WITH caching_sha2_password BY 'pwd12345';
show databases;
create database user_service;
GRANT ALL PRIVILEGES ON user_service.* TO 'ilya'@'172.17.0.1';
exit;
mysql -h 192.168.0.108 -P 3306 -u ilya -p
use user_service;

CREATE TABLE users ( id BIGINT AUTO_INCREMENT PRIMARY KEY, username VARCHAR(255) NOT NULL, email VARCHAR(255) NOT NULL);
exit;
```

## Redis
```bash
sudo apt install redis-server -y

Прописываем ```supervised systemd``` в файл:
```
sudo vi /etc/redis/redis.conf
```
Рестартуем сервис
```bash
sudo systemctl restart redis
```
Проверяем установку
```bash
sudo systemctl status redis
redis-server --version
```
Redis server v=7.0.15

Для проверки запускаем cli
```bash
redis-cli
exit
```

## Build
```bash
sudo snap install rustup
cargo build
cargo run
```

# Test
## Создать пользователя:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"username":"test","email":"test@example.com"}' http://localhost:8080/users
```

## Получить пользователя:
```bash
curl http://localhost:8080/users/1
```

## Debug
```bash
rust-gdb target/debug/user_service
(gdb) break routes.rs:13
run
c или n
```
Проверить ключи в кэше:
```bash
redis-cli -h 192.168.0.108
get user:8
```

