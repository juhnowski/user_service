# Install
## MySQL
```bash
sudo systemctl start mysql
sudo systemctl enable mysql
sudo systemctl status mysql
mysql --version
mysql  Ver 8.0.42-0ubuntu0.24.04.2 for Linux on x86_64 ((Ubuntu))
sudo mysql
CREATE USER 'ilya'@'localhost' IDENTIFIED BY '1';
ALTER USER 'ilya'@'localhost' IDENTIFIED WITH caching_sha2_password BY 'pwd12345';


sudo mysql -h localhost

show databases;
create database user_service;
GRANT ALL PRIVILEGES ON user_service.* TO 'ilya'@'localhost';
exit;

mysql -u ilya -p
use user_service;

CREATE TABLE users ( id INT AUTO_INCREMENT PRIMARY KEY, username VARCHAR(255) NOT NULL, email VARCHAR(255) NOT NULL);

show tables;
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


